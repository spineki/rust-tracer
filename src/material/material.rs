use std::fmt::Debug;

use rand::rngs::ThreadRng;

use crate::{Color3, HitRecord, Ray};

pub trait Material: Debug + Sync {
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
