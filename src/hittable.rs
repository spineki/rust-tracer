use crate::{Point3, Ray, Vec3};

#[derive(Debug, PartialEq)]
pub struct HitRecord {
    /// point hit by the ray
    pub point: Point3,
    /// normal of the hit
    pub normal: Vec3,
    /// time of the hit
    pub t: f64,
    /// if true, the ray hit the front face
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(ray: &Ray, point: &Point3, outward_normal: &Vec3, t: f64) -> Self {
        let front_face = ray.direction().dot(outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal.clone()
        } else {
            *outward_normal * (-1.0)
        };

        Self {
            point: point.clone(),
            normal,
            t,
            front_face,
        }
    }
}

pub trait Hittable {
    // todo: refactor hit to use rust option
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
