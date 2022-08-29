use crate::{material::Material, Point3, Ray, Vec3};

use super::Triangle;

#[derive(Debug)]
pub struct HitRecord<'a> {
    /// if true, the ray hit the front face
    pub front_face: bool,
    // reference to the material hit
    pub material: &'a dyn Material,
    /// normal of the hit
    pub normal: Vec3,
    /// point hit by the ray
    pub point: Point3,
    /// time of the hit
    pub t: f64,
}

impl<'h> HitRecord<'h> {
    pub fn new<'m>(
        // the hit record should life less than the material
        // a hit only exists if the material is still alive
        ray: &Ray,
        point: &Point3,
        outward_normal: &Vec3,
        material: &'m dyn Material,
        t: f64,
    ) -> HitRecord<'h>
    where
        'm: 'h,
    {
        let front_face = ray.direction().dot(outward_normal) < 0.0;
        let normal = if front_face {
            *outward_normal
        } else {
            *outward_normal * (-1.0)
        };

        Self {
            front_face,
            material,
            normal,
            point: *point,
            t,
        }
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

/// This trait should be implemented for shapes that are compound shapes
/// (a square = 2 triangles)
/// (a tetrahedron = 4 triangles)
pub trait MultiFaceHittable: Send + Sync + Hittable {
    fn get_closest_hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let objects = self.get_faces();

        let mut closest_hit_record: Option<HitRecord> = None;

        for object in objects {
            if let Some(hit_record) = object.hit(ray, t_min, t_max) {
                match closest_hit_record {
                    None => {
                        closest_hit_record = Some(hit_record);
                    }
                    Some(ref closest_hit_record_value) => {
                        if hit_record.t < closest_hit_record_value.t {
                            closest_hit_record = Some(hit_record);
                        }
                    }
                }
            }
        }

        closest_hit_record
    }

    fn get_faces(&self) -> &Vec<Triangle>;
}
