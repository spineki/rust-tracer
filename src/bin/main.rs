use gpu_attempt::{
    material::{Dielectric, Lambertian, Material, Metal},
    Camera, Color3, Hittable, HittableList, Point3, Ray, Sphere, Vec3,
};
use rand::{rngs::ThreadRng, Rng};

fn ray_color(ray: &Ray, world: &dyn Hittable, depth: u32, rng: &mut ThreadRng) -> Color3 {
    // the ray bounced too many times, we abort the ray and return no light (black)
    if depth <= 0 {
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

fn main() {
    // Rng
    let mut rng = rand::thread_rng();

    // Image
    let aspect_ratio = 3.0 / 2.0;
    let image_width: u32 = 1200;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel = 500;
    let max_depth = 50;

    // World

    let mut world = HittableList::new();

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

    let mut spheres_element: Vec<(Point3, Box<dyn Material>)> = Vec::new();
    let mut spheres: Vec<Sphere> = Vec::new();

    for a in -11..11 {
        for b in -11..11 {
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

    // Camera -----------------------------------
    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let vertical_fov = 20.0;

    let camera = Camera::new(&look_from, &look_at, &vup, vertical_fov, aspect_ratio);

    // Render -----------------------------------

    println!("P3");
    println!("{image_width} {image_height}");
    println!("255");

    for j in (0..image_height).rev() {
        eprint!("\r remaining {j}");

        for i in 0..image_width {
            let mut pixel_color = Color3::black();

            for _ in 0..samples_per_pixel {
                let u = (i as f64 + rng.gen::<f64>()) / (image_width - 1) as f64;
                let v = (j as f64 + rng.gen::<f64>()) / (image_height - 1) as f64;

                let ray = camera.get_ray(u, v);

                pixel_color += ray_color(&ray, &world, max_depth, &mut rng);
            }
            pixel_color.write(samples_per_pixel);
        }
        println!("");
    }
}
