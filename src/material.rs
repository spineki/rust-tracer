use std::fmt::Debug;

use rand::rngs::ThreadRng;

use crate::{Color3, HitRecord, Ray, Vec3};

pub trait Material: Debug {
    /// # returns
    /// scattered ray
    /// color
    /// if true, the ray was reflected
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_record: &HitRecord,
        rng: &mut ThreadRng,
    ) -> (Ray, Color3, bool);
}

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
/// A metal material that reflects light
#[derive(Debug)]
pub struct Metal {
    /// The raw "color" of the metal
    albedo: Color3,
    /// The greater, the blurrer the reflection. Should be 0 and 1
    fuzziness: f64,
}

impl Metal {
    pub fn new(color: &Color3, fuzziness: f64) -> Self {
        Self {
            albedo: *color,
            fuzziness,
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_record: &HitRecord,
        rng: &mut ThreadRng,
    ) -> (Ray, Color3, bool) {
        // reflecting the incoming ray along the the hit normal
        let reflected = ray_in.direction().normalize().reflect(&hit_record.normal);

        let random_fuziness_direction = Vec3::new_randow_in_unit_sphere(rng) * self.fuzziness;
        let ray_scattered = Ray::new(&hit_record.point, &(reflected + random_fuziness_direction));

        let is_reflected = ray_scattered.direction().dot(&hit_record.normal) > 0.0;

        (ray_scattered, self.albedo, is_reflected)
    }
}
