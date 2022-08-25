use crate::{HitRecord, Hittable};

pub struct HittableList<'a> {
    objects: Vec<&'a dyn Hittable>,
}

impl<'a> HittableList<'a> {
    pub fn new() -> HittableList<'a> {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: &'a dyn Hittable) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList<'_> {
    fn hit(&self, ray: &crate::Ray, t_min: f64, t_max: f64) -> Option<HitRecord<'_>> {
        let mut closest_hit_record: Option<HitRecord> = None;

        for object in &self.objects {
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
}

#[cfg(test)]

mod test {

    use super::{HitRecord, HittableList};
    use crate::{Color3, Hittable, Lambertian, Point3, Ray, Sphere, Vec3};

    #[test]
    fn it_should_detect_intersection_of_aligned_spheres() {
        let material_black = Lambertian::new(&Color3::black());

        // unit sphere centered on 0
        let sphere1 = Sphere::new(&Vec3::new(0.0, 0.0, 0.0), 1.0, &material_black);
        // centered on 2 (not overlapping)
        let sphere2 = Sphere::new(&Vec3::new(3.0, 0.0, 0.0), 1.0, &material_black);

        let mut world = HittableList::new();
        world.add(&sphere1);
        world.add(&sphere2);

        // ray comming from the left
        let ray = Ray::new(&Vec3::new(-100.0, 0.0, 0.0), &Vec3::new(1.0, 0.0, 0.0));

        // hit result
        let hit_record = world.hit(&ray, 0.000, f64::INFINITY).unwrap();

        // only hitting left sphere
        let expected_record = HitRecord {
            point: Point3::new(-1.0, 0.0, 0.0),
            normal: Vec3::new(-1.0, 0.0, 0.0),
            t: 99.0,
            front_face: true,
            material: &material_black,
        };

        assert_eq!(hit_record.point, expected_record.point);
        assert_eq!(hit_record.normal, expected_record.normal);
        assert_eq!(hit_record.t, expected_record.t);
        assert_eq!(hit_record.front_face, expected_record.front_face);
    }

    #[test]

    fn it_should_detect_intersection_with_one_sphere() {
        let material_black = Lambertian::new(&Color3::black());

        // unit sphere centered on 0
        let sphere1 = Sphere::new(&Vec3::new(0.0, 0.0, 0.0), 1.0, &material_black);
        // centered on 2 (not overlapping)
        let sphere2 = Sphere::new(&Vec3::new(3.0, 0.0, 0.0), 1.0, &material_black);

        let mut world = HittableList::new();
        world.add(&sphere1);
        world.add(&sphere2);

        // ray comming from the left
        let ray = Ray::new(&Point3::new(1.5, 0.0, 0.0), &Vec3::new(1.0, 0.0, 0.0));

        // hit result
        let hit_record = world.hit(&ray, 0.000, f64::INFINITY).unwrap();

        // only hitting left sphere
        let expected_record = HitRecord {
            point: Point3::new(2.0, 0.0, 0.0),
            normal: Vec3::new(-1.0, 0.0, 0.0),
            t: 0.5,
            front_face: true,
            material: &material_black,
        };

        assert_eq!(hit_record.point, expected_record.point);
        assert_eq!(hit_record.normal, expected_record.normal);
        assert_eq!(hit_record.t, expected_record.t);
        assert_eq!(hit_record.front_face, expected_record.front_face);
    }
}
