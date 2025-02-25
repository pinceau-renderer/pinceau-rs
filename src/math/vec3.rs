use std::{
    fmt,
    ops::{Add, Div, Mul, Sub},
};

use super::utils::Scalar;

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Vec3<T: Scalar> {
    pub x: T,
    pub y: T,
    pub z: T,
}

/**
 * General
 */
// Static method
impl<T: Scalar> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
    pub fn cross_vectors(a: &Self, b: &Self) -> Self {
        Self::new(
            a.y * b.z - a.z * b.y,
            a.z * b.x - a.x * b.z,
            a.x * b.y - a.y * b.x,
        )
    }
}
// Instance method
impl<T: Scalar> Vec3<T> {
    pub fn dot(&self, other: &Self) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    pub fn cross(&self, other: &Self) -> Self {
        Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
}

impl<T: Scalar> Add for Vec3<T> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}
impl<T: Scalar> Add<T> for Vec3<T> {
    type Output = Self;
    fn add(self, scalar: T) -> Self {
        Self::new(self.x + scalar, self.y + scalar, self.z + scalar)
    }
}

impl<T: Scalar> Sub for Vec3<T> {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}
impl<T: Scalar> Sub<T> for Vec3<T> {
    type Output = Self;
    fn sub(self, scalar: T) -> Self {
        Self::new(self.x - scalar, self.y - scalar, self.z - scalar)
    }
}

impl<T: Scalar> Mul<T> for Vec3<T> {
    type Output = Self;
    fn mul(self, scalar: T) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl<T: Scalar> Div<T> for Vec3<T> {
    type Output = Self;
    fn div(self, scalar: T) -> Self::Output {
        if scalar == T::zero() {
            panic!("Divid by 0")
        }
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

impl<T: Scalar + fmt::Display> fmt::Display for Vec3<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vec3({}, {}, {})", self.x, self.y, self.z)
    }
}

/**
 * Float
 */
impl Vec3<f32> {
    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    pub fn normalize(self) -> Self {
        let magnitude = self.magnitude();
        if magnitude == 0. {
            return Self::new(0., 0., 0.);
        }
        self / magnitude
    }
}
