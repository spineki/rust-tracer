use gpu_attempt::{
    material::{Dielectric, Lambertian, Metal},
    Camera, Color3, Hittable, HittableList, Point3, Ray, Sphere,
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
    let aspect_ratio = 16.0 / 9.0;
    let image_width: u32 = 400;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let max_depth = 50;
    let samples_per_pixel = 100;

    // World
    let mut world = HittableList::new();
    // adding two sphres to the world

    let material_ground = Lambertian::new(&Color3::new(0.8, 0.8, 0.0));
    let material_center = Lambertian::new(&Color3::new(0.1, 0.2, 0.5));
    let material_left = Dielectric::new(1.5);
    let material_right = Metal::new(&Color3::new(0.8, 0.6, 0.2), 0.0);

    let sphere1 = Sphere::new(&Point3::new(0.0, -100.5, -1.0), 100.0, &material_ground);
    world.add(&sphere1);

    let sphere2 = Sphere::new(&Point3::new(0.0, 0.0, -1.0), 0.5, &material_center);
    world.add(&sphere2);

    let sphere3 = Sphere::new(&Point3::new(-1.0, 0.0, -1.0), 0.5, &material_left);
    world.add(&sphere3);

    let sphere_hollow = Sphere::new(&Point3::new(-1.0, 0.0, -1.0), -0.4, &material_left);
    world.add(&sphere_hollow);

    let sphere4 = Sphere::new(&Point3::new(1.0, 0.0, -1.0), 0.5, &material_right);
    world.add(&sphere4);

    // Camera
    let camera = Camera::new();
    // Render

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
