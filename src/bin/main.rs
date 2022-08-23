use gpu_attempt::{sphere::Sphere, Color3, Hittable, HittableList, Point3, Ray, Vec3};

fn ray_color(ray: &Ray, world: &dyn Hittable) -> Color3 {
    if let Some(hit_record) = world.hit(ray, 0.0, f64::INFINITY) {
        return (hit_record.normal + Color3::new(1.0, 1.0, 1.0)) * 0.5;
    }

    let unit_direction = ray.direction().normalize();
    let t = 0.5 * (unit_direction.y() + 1.0);

    Color3::new(1.0, 1.0, 1.0) * (1.0 - t) + Color3::new(0.5, 0.7, 1.0) * t
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width: u32 = 400;
    let image_height = (image_width as f64 / aspect_ratio) as u32;

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
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    // Render

    println!("P3");
    println!("{image_width} {image_height}");
    println!("255");

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;
            let ray = Ray::new(
                &origin,
                &(lower_left_corner + horizontal * u + vertical * v - origin),
            );
            let pixel_color = ray_color(&ray, &world);
            pixel_color.write();
        }
        println!("");
    }
}
