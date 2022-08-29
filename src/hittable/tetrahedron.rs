use crate::{
    hittable::{HitRecord, Hittable},
    material::Material,
    Point3, Ray,
};

use super::{hittable::MultiFaceHittable, HittableList, Triangle};

pub struct Tetrahedron<'a> {
    faces: Vec<Triangle<'a>>,
}

impl<'a> Hittable for Tetrahedron<'a> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.get_closest_hit(ray, t_min, t_max)
    }
}

impl<'a> MultiFaceHittable for Tetrahedron<'a> {
    fn get_faces(&self) -> &Vec<Triangle> {
        &self.faces
    }
}

impl<'a> Tetrahedron<'a> {
    pub fn new(
        vertex_0: &Point3,
        vertex_1: &Point3,
        vertex_2: &Point3,
        vertex_3: &Point3,
        material: &'a dyn Material,
    ) -> Self {
        let face_0 = Triangle::new(vertex_0, vertex_1, vertex_2, material);
        let face_1 = Triangle::new(vertex_0, vertex_1, vertex_3, material);
        let face_2 = Triangle::new(vertex_1, vertex_2, vertex_3, material);
        let face_3 = Triangle::new(vertex_0, vertex_3, vertex_2, material);

        Tetrahedron {
            faces: vec![face_0, face_1, face_2, face_3],
        }
    }
}
