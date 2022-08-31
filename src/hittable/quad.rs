use crate::{
    hittable::{HitRecord, Hittable},
    material::Material,
    Point3, Ray,
};

use super::{hittable::MultiFaceHittable, Triangle};

/// A quad shape. Can be used to create rectangles, squares
/// TODO: Currenlty, we don't check if all the quad vertices are all within the same plane
pub struct Quad<'a> {
    faces: Vec<Triangle<'a>>,
}

impl<'a> Hittable for Quad<'a> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.get_closest_hit(ray, t_min, t_max)
    }
}

impl<'a> MultiFaceHittable for Quad<'a> {
    fn get_faces(&self) -> &Vec<Triangle> {
        &self.faces
    }
}

impl<'a> Quad<'a> {
    pub fn new(
        vertex_0: &Point3,
        vertex_1: &Point3,
        vertex_2: &Point3,
        vertex_3: &Point3,
        material: &'a dyn Material,
    ) -> Self {
        let triangle_0 = Triangle::new(vertex_0, vertex_1, vertex_2, material);
        let triangle_1 = Triangle::new(vertex_2, vertex_3, vertex_0, material);

        Quad {
            faces: vec![triangle_0, triangle_1],
        }
    }
}
