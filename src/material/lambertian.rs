use rand::rngs::ThreadRng;

use super::Material;
use crate::{hittable::HitRecord, Color3, Ray, Vec3};

#[derive(Debug)]
pub struct Lambertian {
    albedo: Color3,
}

impl Lambertian {
    pub fn new(color: &Color3) -> Self {
        Self { albedo: *color }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _ray_in: &Ray, //? the incomming ray is not used for lambertian materials
        hit_record: &HitRecord,
        rng: &mut ThreadRng,
    ) -> (Ray, Color3, bool) {
        let mut scatter_direction = hit_record.normal + Vec3::new_randow_unit_vector(rng);

        // handling case too small if the sum collapses
        if scatter_direction.is_near_zero() {
            scatter_direction = hit_record.normal;
        }

        let ray_scattered = Ray::new(&hit_record.point, &scatter_direction);

        (ray_scattered, self.albedo, true)
    }
}
