use rand::rngs::ThreadRng;

use crate::{Point3, Ray, Vec3};

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64,
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
        aperture: f64,
        focus_distance: f64,
    ) -> Self {
        let theta = vertical_fov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (*look_from - *look_at).normalize();
        let u = vup.cross(&w).normalize();
        let v = w.cross(&u);

        let origin = *look_from;
        let horizontal = u * viewport_width * focus_distance;
        let vertical = v * viewport_height * focus_distance;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w * focus_distance;

        let lens_radius = aperture / 2.0;

        Self {
            horizontal,
            lens_radius,
            lower_left_corner,
            origin,
            u,
            v,
            vertical,
            w,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64, rng: &mut ThreadRng) -> Ray {
        let rd = Vec3::new_random_in_unit_disk(rng) * self.lens_radius;
        let offset = self.u * rd.x() + self.v * rd.y();

        let direction =
            self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin - offset;
        Ray::new(&(self.origin + offset), &direction)
    }
}
