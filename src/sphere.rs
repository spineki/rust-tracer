use crate::{HitRecord, Hittable, Material, Point3};

pub struct Sphere<'a> {
    center: Point3,
    radius: f64,
    material: &'a dyn Material,
}

impl<'a> Hittable for Sphere<'a> {
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
        let hit_record = HitRecord::new(&ray, &point, &outward_normal, self.material, t);

        Some(hit_record)
    }
}

impl<'a> Sphere<'a> {
    pub fn new(center: &Point3, radius: f64, material: &'a dyn Material) -> Self {
        Sphere {
            center: *center,
            radius,
            material,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{Color3, Lambertian, Point3, Ray, Vec3};

    use super::*;

    #[test]
    fn it_should_detect_intersection() {
        let material_black = Lambertian::new(&Color3::black());

        // unit sphere
        let sphere = Sphere::new(&Vec3::new(0.0, 0.0, 0.0), 1.0, &material_black);

        // ray comming from the left
        let ray = Ray::new(&Vec3::new(-100.0, 0.0, 0.0), &Vec3::new(1.0, 0.0, 0.0));

        // hit result
        let hit_record = sphere.hit(&ray, 0.000, f64::INFINITY).unwrap();

        let expected_record = HitRecord {
            front_face: true,
            material: &material_black,
            normal: Vec3::new(-1.0, 0.0, 0.0),
            point: Point3::new(-1.0, 0.0, 0.0),
            t: 99.0,
        };

        assert_eq!(hit_record.point, expected_record.point);
        assert_eq!(hit_record.normal, expected_record.normal);
        assert_eq!(hit_record.t, expected_record.t);
        assert_eq!(hit_record.front_face, expected_record.front_face);
    }

    #[test]
    fn it_should_detect_intersection_from_within() {
        let material_black = Lambertian::new(&Color3::black());

        // unit sphere
        let sphere = Sphere::new(&Vec3::new(0.0, 0.0, 0.0), 1.0, &material_black);

        // ray comming from the center (left, to right)
        let ray = Ray::new(&Vec3::new(0.0, 0.0, 0.0), &Vec3::new(1.0, 0.0, 0.0));

        // hit result
        let hit_record = sphere.hit(&ray, 0.000, f64::INFINITY).unwrap();

        let expected_record = HitRecord {
            front_face: false, //? notice that the inner colision is detected
            material: &material_black,
            normal: Vec3::new(-1.0, 0.0, 0.0), //? notice the normal oriented to the left
            point: Point3::new(1.0, 0.0, 0.0),
            t: 1.0,
        };

        assert_eq!(hit_record.point, expected_record.point);
        assert_eq!(hit_record.normal, expected_record.normal);
        assert_eq!(hit_record.t, expected_record.t);
        assert_eq!(hit_record.front_face, expected_record.front_face);
    }

    #[test]
    fn it_should_ignore_outer_rays() {
        let material_black = Lambertian::new(&Color3::black());

        // unit sphere
        let sphere = Sphere::new(&Vec3::new(0.0, 0.0, 0.0), 1.0, &material_black);

        // ray comming from the righ toward right (wrong direction)
        let ray = Ray::new(&Vec3::new(100.0, 0.0, 0.0), &Vec3::new(1.0, 0.0, 0.0));

        // hit result
        let hit_record = sphere.hit(&ray, 0.000, f64::INFINITY);

        assert!(hit_record.is_none());
    }
}
