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
    fn hit(&self, ray: &crate::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if let Some(hit_record) = object.hit(ray, t_min, t_max) {
                closest_so_far = hit_record.t;

                // todo change this to only take into account the closest element
                return Some(hit_record);
            }
        }

        None
    }
}
