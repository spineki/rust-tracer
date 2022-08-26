use rand::{rngs::ThreadRng, Rng};

use super::Material;
use crate::{hittable::HitRecord, Color3, Ray};

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

impl Dielectric {
    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        // Schlick's approximation
        let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        let r1 = r0 * r0;

        r1 + (1.0 - r1) * (1.0 - cosine).powi(5)
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

        let cos_theta = (unit_direction * -1.0).dot(&hit_record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let direction =
            if cannot_refract || Self::reflectance(cos_theta, refraction_ratio) > rng.gen() {
                unit_direction.reflect(&hit_record.normal)
            } else {
                unit_direction.refract(&hit_record.normal, refraction_ratio)
            };

        let ray_scattered = Ray::new(&hit_record.point, &direction);

        (ray_scattered, Color3::white(), true)
    }
}
