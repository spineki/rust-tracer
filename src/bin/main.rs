use gpu_attempt::{Color3, Point3, Ray, Vec3};

fn hit_sphere(center: &Point3, radius: f64, ray: &Ray) -> f64 {
    let oc = ray.origin() - *center;
    let a = ray.direction().dot(&ray.direction());
    let b = oc.dot(&ray.direction()) * 2.0;
    let c = oc.dot(&oc) - radius * radius;
    let discriminant = b * b - a * c * 4.0;

    // 0 => not intersection with the sphre
    // 1 => one intersection (tangent)
    // 2 => fully intersection (going trought)
    if discriminant < 0.0 {
        return -1.0;
    }

    // returning the root
    (-b - discriminant.sqrt()) / (2.0 * a)
}

fn ray_color(ray: &Ray) -> Color3 {
    let mut t = hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5, &ray);
    if t > 0.0 {
        let N = (ray.at(t) - Vec3::new(0.0, 0.0, -1.0)).normalize();
        return Color3::new(N.x() + 1.0, N.y() + 1.0, N.z() + 1.0) * 0.5;
    }

    let unit_direction = ray.direction().normalize();
    t = 0.5 * (unit_direction.y() + 1.0);
    Color3::new(1.0, 1.0, 1.0) * (1.0 - t) + Color3::new(0.5, 0.7, 1.0) * t
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width: u32 = 400;
    let image_height = (image_width as f64 / aspect_ratio) as u32;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

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
            let pixel_color = ray_color(&ray);
            pixel_color.write();
        }
        println!("");
    }
}
