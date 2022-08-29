use crate::{
    hittable::{HitRecord, Hittable},
    material::Material,
    Point3,
};

pub struct Triangle<'a> {
    vertex_0: Point3,
    vertex_1: Point3,
    vertex_2: Point3,
    material: &'a dyn Material,
}

impl<'a> Hittable for Triangle<'a> {
    fn hit(&self, ray: &crate::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // Möller–Trumbore algorithm
        let vertex_0 = self.vertex_0;
        let vertex_1 = self.vertex_1;
        let vertex_2 = self.vertex_2;

        // edges of the triangle
        let edge_1 = vertex_1 - vertex_0;
        let edge_2 = vertex_2 - vertex_0;

        let h = ray.direction().cross(&edge_2);
        let a = edge_1.dot(&h);

        const EPSILON: f64 = 0.0000001;

        if a.abs() < EPSILON {
            return None; // ray parallel to triangle
        }

        let f = 1.0 / a;
        let s = ray.origin() - vertex_0;
        let u = f * s.dot(&h);

        if u < 0.0 || u > 1.0 {
            return None; // no solution
        }

        let q = s.cross(&edge_1);
        let v = f * ray.direction().dot(&q);
        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        // We know now that an intersection happened
        // computing t to know where it happened
        let t = f * edge_2.dot(&q);

        // if the intersection is not in the expected range, abort the ray
        if t < t_min || t > t_max {
            return None;
        }

        let point = ray.at(t);

        // the cross product gives a normal vector, we just need to normalize it
        let outward_normal = edge_1.cross(&edge_2).normalize();

        Some(HitRecord::new(
            ray,
            &point,
            &outward_normal,
            self.material,
            t,
        ))
    }
}

impl<'a> Triangle<'a> {
    pub fn new(
        vertex_0: &Point3,
        vertex_1: &Point3,
        vertex_2: &Point3,
        material: &'a dyn Material,
    ) -> Self {
        Triangle {
            vertex_0: *vertex_0,
            vertex_1: *vertex_1,
            vertex_2: *vertex_2,
            material,
        }
    }
}
