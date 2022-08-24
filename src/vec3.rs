use std::ops;

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

        let (r, g, b) = (self.x() * scale, self.y() * scale, self.z() * scale);

        let ir = (256.0 * r.clamp(0.0, 0.999)) as u32;
        let ig = (256.0 * g.clamp(0.0, 0.999)) as u32;
        let ib = (256.0 * b.clamp(0.0, 0.999)) as u32;

        println!("{ir} {ig} {ib}");
    }

    pub fn black() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
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
        assert_eq!(Vec3::new(2.0, 4.0, 6.0) / 2.0, Vec3::new(1.0, 2.0, 3.0));
    }
}
