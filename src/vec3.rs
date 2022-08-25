use std::ops;

use rand::Rng;

/// A 3 dimension vector type
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

pub type Point3 = Vec3;
pub type Color3 = Vec3;

impl Color3 {
    pub fn write(&self, samples_per_pixel: u32) {
        let scale = 1.0 / samples_per_pixel as f64;

        // scale the color per sample and add a gamma correct of 2 (sqrrt = power 1/2)
        let (r, g, b) = (
            (self.x() * scale).sqrt(),
            (self.y() * scale).sqrt(),
            (self.z() * scale).sqrt(),
        );

        let ir = (256.0 * r.clamp(0.0, 0.999)) as u32;
        let ig = (256.0 * g.clamp(0.0, 0.999)) as u32;
        let ib = (256.0 * b.clamp(0.0, 0.999)) as u32;

        println!("{ir} {ig} {ib}");
    }

    /// a black Color3 with all chanels at 0
    pub fn black() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    // returns a new random vector with coordinates in the specified range
    pub fn new_clamped_random(min: f64, max: f64, rng: &mut impl Rng) -> Self {
        Self {
            x: rng.gen_range(min..=max),
            y: rng.gen_range(min..=max),
            z: rng.gen_range(min..=max),
        }
    }

    /// create a vector in the unit sphere. Creating a random vector in the unit cube until it's in the sphere
    /// Probability of success per iteration. 4/3 pi / 8 ~= 0.52... not so great but will converge eventually
    /// ugly
    pub fn new_randow_in_unit_sphere(rng: &mut impl Rng) -> Self {
        loop {
            let vector = Self::new_clamped_random(-1.0, 1.0, rng);
            if vector.mag_squared() < 1.0 {
                return vector;
            }
        }
    }

    pub fn new_randow_unit_vector(rng: &mut impl Rng) -> Self {
        Self::new_randow_in_unit_sphere(rng).normalize()
    }

    pub fn x(&self) -> f64 {
        self.x
    }
    pub fn y(&self) -> f64 {
        self.y
    }
    pub fn z(&self) -> f64 {
        self.z
    }

    /// Return true if close to zero in all dimension (normen 1)
    pub fn is_near_zero(&self) -> bool {
        let epsilon = 10e-8;

        self.x.abs() < epsilon && self.y.abs() < epsilon && self.z.abs() < epsilon
    }

    // reflect the current vector along a given normal
    pub fn reflect(&self, n: &Vec3) -> Vec3 {
        *self - *n * 2.0 * self.dot(n)
    }

    /// magnitude of the vector (careful, a sqrt is used)
    pub fn mag(&self) -> f64 {
        self.mag_squared().sqrt()
    }

    /// squared magnitude of the vector (no sqrt)
    pub fn mag_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// Dot product between vectors
    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// cross product
    pub fn cross(&self, other: &Self) -> Self {
        Vec3::new(
            self.y() * other.z() - self.z() * other.y(),
            self.z() * other.x() - self.x() * other.z(),
            self.x() * other.y() - self.y() * other.x(),
        )
    }

    /// create a vector with coordinates multiplied term by term
    /// e.g. (1, 2, 3).hadamar((2, 3, 4)) = (2, 6, 12)
    pub fn hadamar(&self, other: &Self) -> Self {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }

    pub fn normalize(&self) -> Self {
        self / self.mag()
    }
}

// Overloading basic operators for convenience

impl ops::Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x() + rhs.x(),
            y: self.y() + rhs.y(),
            z: self.z() + rhs.z(),
        }
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x() - rhs.x(),
            y: self.y() - rhs.y(),
            z: self.z() - rhs.z(),
        }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Self {
        Self {
            x: self.x() * rhs,
            y: self.y() * rhs,
            z: self.z() * rhs,
        }
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Self {
        self * (1.0 / rhs)
    }
}
// Reusing owned code for references
impl ops::Div<f64> for &Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Self::Output {
        (*self) / rhs
    }
}

// Overloading assign operators for convenience

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x();
        self.y += rhs.y();
        self.z += rhs.z();
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x();
        self.y -= rhs.y();
        self.z -= rhs.z();
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_do_basic_arithmetic_operations() {
        assert_eq!(
            Vec3::new(1.0, 2.0, 3.0) + Vec3::new(2.0, 1.0, 0.0),
            Vec3::new(3.0, 3.0, 3.0)
        );

        assert_eq!(
            Vec3::new(1.0, 2.0, 3.0) - Vec3::new(1.0, 1.0, 1.0),
            Vec3::new(0.0, 1.0, 2.0)
        );

        assert_eq!(Vec3::new(1.0, 2.0, 3.0) * 2.0, Vec3::new(2.0, 4.0, 6.0));
        assert_eq!(Vec3::new(1.0, 2.0, 3.0) * -1.0, Vec3::new(-1.0, -2.0, -3.0));
        assert_eq!(Vec3::new(2.0, 4.0, 6.0) / 2.0, Vec3::new(1.0, 2.0, 3.0));
    }
}
