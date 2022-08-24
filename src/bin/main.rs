use gpu_attempt::{camera::Camera, Color3, Hittable, HittableList, Point3, Ray, Sphere, Vec3};
use rand::Rng;

fn ray_color(ray: &Ray, world: &dyn Hittable) -> Color3 {
    if let Some(hit_record) = world.hit(ray, 0.0, f64::INFINITY) {
        return (hit_record.normal + Color3::new(1.0, 1.0, 1.0)) * 0.5;
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
    let samples_per_pixel = 100;

    // World
    let mut world = HittableList::new();
    // adding two sphres to the world

    let point1 = Point3::new(0.0, 0.0, -1.0);
    let sphere = Sphere::new(&point1, 0.5);
    world.add(&sphere);

    let point2 = Point3::new(0.0, -100.0, -1.0);
    let sphere2 = Sphere::new(&point2, 100.0);
    world.add(&sphere2);

    // Camera
    let camera = Camera::new();
    // Render

    println!("P3");
    println!("{image_width} {image_height}");
    println!("255");

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let mut pixel_color = Color3::black();

            for s in 0..samples_per_pixel {
                let u = (i as f64 + rng.gen::<f64>()) / (image_width - 1) as f64;
                let v = (j as f64 + rng.gen::<f64>()) / (image_height - 1) as f64;

                let ray = camera.get_ray(u, v);

                pixel_color += ray_color(&ray, &world);
            }
            pixel_color.write(samples_per_pixel);
        }
        println!("");
    }
}
