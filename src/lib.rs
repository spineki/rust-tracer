pub mod hittable;
pub mod material;

mod camera;
pub use camera::Camera;

mod ray;
pub use ray::Ray;

mod vec3;
pub use vec3::{Color3, Point3, Vec3};
