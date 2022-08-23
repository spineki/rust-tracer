use crate::{HitRecord, Hittable, Point3};

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &crate::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().mag_squared();
        let half_b = oc.dot(&ray.direction());
        let c = oc.mag_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;

        // no intersection
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;

        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        // creating the hit record
        let t = root;
        let point = ray.at(t);
        let outward_normal = (point - self.center) / self.radius;
        let hit_record = HitRecord::new(&ray, &point, &outward_normal, t);

        Some(hit_record)
    }
}

impl Sphere {
    pub fn new(center: &Point3, radius: f64) -> Self {
        Sphere {
            center: *center,
            radius,
        }
    }
}
