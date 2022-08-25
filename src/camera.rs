use crate::{Point3, Ray, Vec3};

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    /// vertical_field of view in degree
    /// aspect_ratio: e.g. 16 / 9
    /// vup:  view up vector

    pub fn new(
        look_from: &Point3,
        look_at: &Point3,
        vup: &Vec3,
        vertical_fov: f64,
        aspect_ratio: f64,
    ) -> Self {
        let theta = vertical_fov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (*look_from - *look_at).normalize();
        let u = vup.cross(&w).normalize();
        let v = w.cross(&u);

        let origin = *look_from;
        let horizontal = u * viewport_width;
        let vertical = v * viewport_height;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w;

        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let direction =
            self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin;
        Ray::new(&self.origin, &direction)
    }
}
