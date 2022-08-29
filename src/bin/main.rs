use std::{
    fs::File,
    io::Write,
    sync::{Arc, Mutex},
    thread,
    time::Instant,
};

use gpu_attempt::{
    hittable::{Hittable, HittableList, Sphere, Triangle},
    material::{Dielectric, Lambertian, Material, Metal},
    Camera, Color3, Point3, Ray, Vec3,
};
use rand::{rngs::ThreadRng, Rng};

fn ray_color(ray: &Ray, world: &dyn Hittable, depth: u32, rng: &mut ThreadRng) -> Color3 {
    // the ray bounced too many times, we abort the ray and return no light (black)
    if depth == 0 {
        return Color3::new(0.0, 0.0, 0.0);
    }

    // using 0.001 instead of 0.0 to fix shadow acne (ray reflected not exactly at 0)
    if let Some(hit_record) = world.hit(ray, 0.001, f64::INFINITY) {
        let (scattered, attenuation, is_reflected) =
            hit_record.material.scatter(ray, &hit_record, rng);

        if is_reflected {
            return ray_color(&scattered, world, depth - 1, rng).hadamar(&attenuation);
        }

        return Color3::black();
    }

    let unit_direction = ray.direction().normalize();
    let t = 0.5 * (unit_direction.y() + 1.0);

    Color3::new(1.0, 1.0, 1.0) * (1.0 - t) + Color3::new(0.5, 0.7, 1.0) * t
}

/// compute a scene:
/// if dry_mode is true, computation are made but are not streamed to stdout
fn compute_scene(
    camera: &Camera,
    world: &HittableList,
    image_width: u32,
    image_height: u32,
    samples_per_pixel: u32,
    max_depth: u32,
) -> Vec<Vec<Color3>> {
    let scene = Arc::new(Mutex::new(vec![
        vec![Color3::white(); image_width as usize];
        image_height as usize
    ]));

    let nb_thread = 6;

    // taking in account rounding up
    let nb_line_per_thread = if image_height % nb_thread == 0 {
        image_height / nb_thread
    } else {
        1 + image_height / nb_thread
    };

    thread::scope(|scope| {
        let mut handles = Vec::new();

        for num_thread in 0..nb_thread {
            // cloning the arc to be able to share reference of underlying data
            let scene = scene.clone();

            let current_line = num_thread * nb_line_per_thread;

            let handle = scope.spawn(move || {
                let mut rng = rand::thread_rng();

                for i in current_line..(current_line + nb_line_per_thread).min(image_height) {
                    // eprint!("\r remaining lines {}", image_height - i);
                    let mut row = Vec::with_capacity(image_width as usize);

                    for j in 0..image_width {
                        let mut pixel_color = Color3::black();

                        for _ in 0..samples_per_pixel {
                            let u = (j as f64 + rng.gen::<f64>()) / (image_width - 1) as f64;
                            let v = (i as f64 + rng.gen::<f64>()) / (image_height - 1) as f64;

                            let ray = camera.get_ray(u, v, &mut rng);

                            pixel_color += ray_color(&ray, world, max_depth, &mut rng);
                        }

                        row.push(pixel_color);
                    }
                    scene.lock().unwrap()[i as usize] = row;
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle
                .join()
                .expect("an error occured while joining threads");
        }
    });

    Arc::try_unwrap(scene).unwrap().into_inner().unwrap()
}

/// save a scene in ppm format
/// the incoming schene is expected to be an vector of rows.
/// So we iterate this way: scene[line][column]
/// the sample per pixels is necessary to scale colors down and then apply gamma correction
fn save_scene(scene: &Vec<Vec<Color3>>, samples_per_pixel: u32) {
    let nb_lines = scene.len();
    let nb_columns = scene[0].len();

    let mut file = File::create("scene.ppm").expect("could not create the file");
    write!(file, "P3\n");
    write!(file, "{} {}\n", nb_columns, nb_lines);
    write!(file, "255\n");

    write!(file, "\n");
    for i in (0..nb_lines).rev() {
        for j in 0..nb_columns {
            let pixel_color = scene[i][j];

            write!(file, "{}\n", pixel_color.as_ppm(samples_per_pixel));
        }
        write!(file, "\n");
    }
}

fn main() {
    println!("starting rendering");

    let starting_time = Instant::now();

    // Rng --------------------------------------
    let mut rng = rand::thread_rng();

    // Image ------------------------------------
    let aspect_ratio = 3.0 / 2.0;
    let image_width: u32 = 1200;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel = 5; // 500 is great
                               // max number of ray bounces
    let max_depth = 50;

    // World ------------------------------------

    let mut world = HittableList::new();

    // First, adding big spheres ----------------

    // a massive sphere
    let material_ground = Lambertian::new(&Color3::new(0.5, 0.5, 0.5));
    let sphere_ground = Sphere::new(&Point3::new(0.0, -1000.0, 0.0), 1000.0, &material_ground);
    world.add(&sphere_ground);

    let material_dielectric = Dielectric::new(1.5);
    let sphere_dialectric = Sphere::new(&Point3::new(0.0, 1.0, 0.0), 1.0, &material_dielectric);
    world.add(&sphere_dialectric);

    let material_lambertian = Lambertian::new(&Color3::new(0.4, 0.2, 0.1));
    let sphere_lambertian = Sphere::new(&Point3::new(-4.0, 1.0, 0.0), 1.0, &material_lambertian);
    world.add(&sphere_lambertian);

    let material_metal = Metal::new(&Color3::new(0.7, 0.6, 0.5), 0.0);
    let sphere_metal = Sphere::new(&Point3::new(4.0, 1.0, 0.0), 1.0, &material_metal);
    world.add(&sphere_metal);

    // Then adding little spheres ---------------
    let mut spheres_element: Vec<(Point3, Box<dyn Material>)> = Vec::new();
    let mut spheres: Vec<Sphere> = Vec::new();

    // number of nodes to display on a grid
    let nb_grid_nodes = 11; // 11

    for a in -nb_grid_nodes..nb_grid_nodes {
        for b in -nb_grid_nodes..nb_grid_nodes {
            let random_choose: f64 = rng.gen();
            let center = Point3::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).mag() > 0.9 {
                let sphere_material: Box<dyn Material> = if random_choose < 0.8 {
                    // lambertian (diffuse)
                    let albedo = Color3::new_clamped_random(0.0, 1.0, &mut rng)
                        .hadamar(&Color3::new_clamped_random(0.0, 1.0, &mut rng));

                    Box::new(Lambertian::new(&albedo))
                } else if random_choose < 0.95 {
                    // metal
                    let albedo = Color3::new_clamped_random(0.5, 1.0, &mut rng);
                    let fuzziness = rng.gen_range(0.0..=0.5);

                    Box::new(Metal::new(&albedo, fuzziness))
                } else {
                    //glass
                    Box::new(Dielectric::new(1.5))
                };

                spheres_element.push((center, sphere_material));
            }
        }
    }

    for (center, material) in &spheres_element {
        let sphere = Sphere::new(center, 0.2, material.as_ref());
        spheres.push(sphere);
    }

    for sphere in &spheres {
        world.add(sphere);
    }

    // finally adding triangles for tests

    let triangle = Triangle::new(
        &Point3::new(3.0, 0.0, 3.0),
        &Point3::new(3.0, 0.0, 0.0),
        &Point3::new(3.0, 2.0, 3.0),
        &material_metal,
    );

    world.add(&triangle);

    // Camera -----------------------------------
    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let vertical_fov = 20.0;
    let aperture = 0.1;
    let focus_distance = 10.0;

    let camera = Camera::new(
        &look_from,
        &look_at,
        &vup,
        vertical_fov,
        aspect_ratio,
        aperture,
        focus_distance,
    );

    // Render -----------------------------------

    let scene = compute_scene(
        &camera,
        &world,
        image_width,
        image_height,
        samples_per_pixel,
        max_depth,
    );

    let duration = starting_time.elapsed();
    eprintln!("the rendering function took {:?} to run", duration);

    eprintln!("Saving the values to a file...");
    // comment this for benchmarks
    save_scene(&scene, samples_per_pixel);
}
