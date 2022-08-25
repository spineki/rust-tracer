use rand::rngs::ThreadRng;

use super::Material;
use crate::{Color3, HitRecord, Ray, Vec3};

/// A Dielectric material that reflects and refracts light
#[derive(Debug)]
pub struct Dielectric {
    // the index of refraction of the material ( c / v) with c, celrity, v, speed of light in the material
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_record: &HitRecord,
        rng: &mut ThreadRng,
    ) -> (Ray, Color3, bool) {
        let refraction_ratio = if hit_record.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = ray_in.direction().normalize();
        let refracted = unit_direction.refract(&hit_record.normal, refraction_ratio);

        let ray_scattered = Ray::new(&hit_record.point, &refracted);

        (ray_scattered, Color3::white(), true)
    }
}
