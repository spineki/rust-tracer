mod hittable;
mod hittable_list;
mod sphere;
mod tetrahedron;
mod triangle;

pub use hittable::{HitRecord, Hittable};
pub use hittable_list::HittableList;
pub use sphere::Sphere;
pub use tetrahedron::Tetrahedron;
pub use triangle::Triangle;
