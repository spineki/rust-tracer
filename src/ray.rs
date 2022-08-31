use crate::{Point3, Vec3};

/// A ray used for ray tracing
pub struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    /// create a new ray of light
    pub fn new(origin: &Point3, direction: &Vec3) -> Self {
        Self {
            origin: *origin,
            direction: *direction,
        }
    }

    /// get the origin, the source of the light
    pub fn origin(&self) -> Point3 {
        self.origin
    }

    /// get the direction vector of the ray
    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    /// return the point reached by the ray at time t
    pub fn at(&self, t: f64) -> Point3 {
        self.origin + self.direction * t
    }
}
