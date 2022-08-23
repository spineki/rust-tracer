pub mod hittable;
pub use hittable::{HitRecord, Hittable};

pub mod hittable_list;
pub use hittable_list::HittableList;

mod ray;
pub mod sphere;
mod vec3;
pub use ray::Ray;
pub use vec3::{Color3, Point3, Vec3};
