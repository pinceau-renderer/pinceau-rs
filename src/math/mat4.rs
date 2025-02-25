use std::{
    fmt,
    ops::{Add, Mul, Sub},
};

use crate::math::utils::deg_to_rad;

use super::{utils::Scalar, vec3::Vec3};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mat4<T> {
    pub data: [T; 16],
}

impl<T: Scalar + fmt::Display> fmt::Display for Mat4<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Mat3(\n\t{}, {}, {}, {},\n\t{}, {}, {}, {},\n\t{}, {}, {}, {},\n\t{}, {}, {}, {},\n)",
            self.data[0],
            self.data[4],
            self.data[8],
            self.data[12],
            self.data[1],
            self.data[5],
            self.data[9],
            self.data[13],
            self.data[2],
            self.data[6],
            self.data[10],
            self.data[14],
            self.data[3],
            self.data[7],
            self.data[11],
            self.data[15],
        )
    }
}

// Static
impl<T: Scalar> Mat4<T> {
    pub fn new(
        m00: T,
        m01: T,
        m02: T,
        m03: T,
        m10: T,
        m11: T,
        m12: T,
        m13: T,
        m20: T,
        m21: T,
        m22: T,
        m23: T,
        m30: T,
        m31: T,
        m32: T,
        m33: T,
    ) -> Self {
        Self {
            data: [
                m00, m01, m02, m03, m10, m11, m12, m13, m20, m21, m22, m23, m30, m31, m32, m33,
            ],
        }
    }

    pub fn zero() -> Self {
        Self::new(
            T::zero(),
            T::zero(),
            T::zero(),
            T::zero(),
            T::zero(),
            T::zero(),
            T::zero(),
            T::zero(),
            T::zero(),
            T::zero(),
            T::zero(),
            T::zero(),
            T::zero(),
            T::zero(),
            T::zero(),
            T::zero(),
        )
    }

    pub fn identity() -> Self {
        Self::new(
            T::one(),
            T::zero(),
            T::zero(),
            T::zero(),
            T::zero(),
            T::one(),
            T::zero(),
            T::zero(),
            T::zero(),
            T::zero(),
            T::one(),
            T::zero(),
            T::zero(),
            T::zero(),
            T::zero(),
            T::one(),
        )
    }

    fn index(row: usize, col: usize) -> usize {
        row * 4 + col
    }
}

// Instance
impl<T: Scalar> Mat4<T> {
    pub fn determinant(&self) -> T {
        let [m00, m01, m02, m03, m10, m11, m12, m13, m20, m21, m22, m23, m30, m31, m32, m33] =
            self.data;

        let det0 = m11 * (m22 * m33 - m23 * m32) - m12 * (m21 * m33 - m23 * m31)
            + m13 * (m21 * m32 - m22 * m31);
        let det1 = m10 * (m22 * m33 - m23 * m32) - m12 * (m20 * m33 - m23 * m30)
            + m13 * (m20 * m32 - m22 * m30);
        let det2 = m10 * (m21 * m33 - m23 * m31) - m11 * (m20 * m33 - m23 * m30)
            + m13 * (m20 * m31 - m21 * m30);
        let det3 = m10 * (m21 * m32 - m22 * m31) - m11 * (m20 * m32 - m22 * m30)
            + m12 * (m20 * m31 - m21 * m30);

        m00 * det0 - m01 * det1 + m02 * det2 - m03 * det3
    }

    pub fn transpose(self) -> Self {
        Self::new(
            self.data[0],
            self.data[4],
            self.data[8],
            self.data[12],
            self.data[1],
            self.data[5],
            self.data[9],
            self.data[13],
            self.data[2],
            self.data[6],
            self.data[10],
            self.data[14],
            self.data[3],
            self.data[7],
            self.data[11],
            self.data[15],
        )
    }

    pub fn extract_basis(&self) -> (Vec3<T>, Vec3<T>, Vec3<T>) {
        let [x1, y1, z1, _, x2, y2, z2, _, x3, y3, z3, ..] = self.data;
        (
            Vec3::new(x1, x2, x3),
            Vec3::new(y1, y2, y3),
            Vec3::new(z1, z2, z3),
        )
    }
}

impl Mat4<f32> {
    pub fn make_rotation_axis(axis: Vec3<f32>, angle: f32) -> Self {
        // seen https://www.gamedev.net/articles/programming/math-and-physics/do-we-really-need-quaternions-r1199/ while browsing threejs codebase
        let c = angle.cos();
        let s = angle.sin();
        let t = 1. - c;
        let Vec3 { x, y, z } = axis;
        Self::new(
            t * x * x - c,
            t * x * y - s * z,
            t * x * z - x * y,
            0.,
            t * x * y - s * z,
            t * y * y + c,
            t * y * z + s * x,
            0.,
            t * x * z + s * y,
            t * y * z - x * s,
            t * z * z + c,
            0.,
            0.,
            0.,
            0.,
            1.,
        )
    }
    pub fn rotate(self, axis: Vec3<f32>, angle: f32) -> Self {
        self * Self::make_rotation_axis(axis, angle)
    }
    pub fn make_scale(x: f32, y: f32, z: f32) -> Self {
        Self::new(x, 0., 0., 0., 0., y, 0., 0., 0., 0., z, 0., 0., 0., 0., 1.)
    }
    pub fn scale(self, x: f32, y: f32, z: f32) -> Self {
        self * Self::make_scale(x, y, z)
    }
    pub fn make_translation(x: f32, y: f32, z: f32) -> Self {
        Self::new(1., 0., 0., x, 0., 1., 0., y, 0., 0., 1., z, 0., 0., 0., 1.)
    }
    pub fn translate(self, x: f32, y: f32, z: f32) -> Self {
        self * Self::make_translation(x, y, z)
    }
    pub fn perspective(fov: f32, aspect: f32, near: f32, far: f32) -> Self {
        let fov_rad = deg_to_rad(fov);
        let f = 1. / (fov_rad / 2.).tan();
        let depth = 1. / (near - far);

        Self::new(
            f / aspect,
            0.,
            0.,
            0.,
            0.,
            f,
            0.,
            0.,
            0.,
            0.,
            (far + near) * depth,
            -1.,
            0.,
            0.,
            2. * far * near * depth,
            0.,
        )
    }
    pub fn orthogonal(top: f32, right: f32, bottom: f32, left: f32, near: f32, far: f32) -> Self {
        let horizontal = 1. / (left - right);
        let vertical = 1. / (bottom - top);
        let depth = 1. / (near - far);

        Self::new(
            -2. * horizontal,
            0.,
            0.,
            0.,
            0.,
            -2. * vertical,
            0.,
            0.,
            0.,
            0.,
            2. * depth,
            0.,
            (left + right) * horizontal,
            (top + bottom) * vertical,
            (far + near) * depth,
            1.,
        )
    }
    pub fn invert(self) -> Option<Self> {
        let [m00, m01, m02, m03, m10, m11, m12, m13, m20, m21, m22, m23, m30, m31, m32, m33] =
            self.data;

        let c00 = m11 * (m22 * m33 - m23 * m32) - m12 * (m21 * m33 - m23 * m31)
            + m13 * (m21 * m32 - m22 * m31);
        let c01 = -(m10 * (m22 * m33 - m23 * m32) - m12 * (m20 * m33 - m23 * m30)
            + m13 * (m20 * m32 - m22 * m30));
        let c02 = m10 * (m21 * m33 - m23 * m31) - m11 * (m20 * m33 - m23 * m30)
            + m13 * (m20 * m31 - m21 * m30);
        let c03 = -(m10 * (m21 * m32 - m22 * m31) - m11 * (m20 * m32 - m22 * m30)
            + m12 * (m20 * m31 - m21 * m30));

        let det = m00 * c00 + m01 * c01 + m02 * c02 + m03 * c03;

        if det == 0. {
            return None;
        }

        let c10 = -(m01 * (m22 * m33 - m23 * m32) - m02 * (m21 * m33 - m23 * m31)
            + m03 * (m21 * m32 - m22 * m31));
        let c11 = m00 * (m22 * m33 - m23 * m32) - m02 * (m20 * m33 - m23 * m30)
            + m03 * (m20 * m32 - m22 * m30);
        let c12 = -(m00 * (m21 * m33 - m23 * m31) - m01 * (m20 * m33 - m23 * m30)
            + m03 * (m20 * m31 - m21 * m30));
        let c13 = m00 * (m21 * m32 - m22 * m31) - m01 * (m20 * m32 - m22 * m30)
            + m02 * (m20 * m31 - m21 * m30);

        let c20 = m01 * (m12 * m33 - m13 * m32) - m02 * (m11 * m33 - m13 * m31)
            + m03 * (m11 * m32 - m12 * m31);
        let c21 = -(m00 * (m12 * m33 - m13 * m32) - m02 * (m10 * m33 - m13 * m30)
            + m03 * (m10 * m32 - m12 * m30));
        let c22 = m00 * (m11 * m33 - m13 * m31) - m01 * (m10 * m33 - m13 * m30)
            + m03 * (m10 * m31 - m11 * m30);
        let c23 = -(m00 * (m11 * m32 - m12 * m31) - m01 * (m10 * m32 - m12 * m30)
            + m02 * (m10 * m31 - m11 * m30));

        let c30 = -(m01 * (m12 * m23 - m13 * m22) - m02 * (m11 * m23 - m13 * m21)
            + m03 * (m11 * m22 - m12 * m21));
        let c31 = m00 * (m12 * m23 - m13 * m22) - m02 * (m10 * m23 - m13 * m20)
            + m03 * (m10 * m22 - m12 * m20);
        let c32 = -(m00 * (m11 * m23 - m13 * m21) - m01 * (m10 * m23 - m13 * m20)
            + m03 * (m10 * m21 - m11 * m20));
        let c33 = m00 * (m11 * m22 - m12 * m21) - m01 * (m10 * m22 - m12 * m20)
            + m02 * (m10 * m21 - m11 * m20);

        let adjugate = Mat4::new(
            c00, c10, c20, c30, c01, c11, c21, c31, c02, c12, c22, c32, c03, c13, c23, c33,
        );

        Some(adjugate * (1. / det))
    }
}

impl<T: Scalar> Add for Mat4<T> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let mut result = self;
        for i in 0..16 {
            result.data[i] = self.data[i] + other.data[i];
        }
        result
    }
}

impl<T: Scalar> Add<T> for Mat4<T> {
    type Output = Self;
    fn add(self, scalar: T) -> Self {
        let mut result = self;
        for i in 0..16 {
            result.data[i] = self.data[i] + scalar
        }
        result
    }
}
impl<T: Scalar> Sub for Mat4<T> {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        let mut result = self;
        for i in 0..16 {
            result.data[i] = self.data[i] - other.data[i];
        }
        result
    }
}

impl<T: Scalar> Sub<T> for Mat4<T> {
    type Output = Self;
    fn sub(self, scalar: T) -> Self {
        let mut result = self;
        for i in 0..16 {
            result.data[i] = self.data[i] - scalar;
        }
        result
    }
}

impl<T: Scalar> Mul<T> for Mat4<T> {
    type Output = Self;
    fn mul(self, scalar: T) -> Self {
        let mut result = self;
        for i in 0..16 {
            result.data[i] = self.data[i] * scalar;
        }
        result
    }
}

impl<T: Scalar> Mul for Mat4<T> {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        let mut result = Mat4::zero();
        for i in 0..4 {
            for j in 0..4 {
                result.data[Self::index(i, j)] = self.data[Self::index(i, 0)]
                    * other.data[Self::index(0, j)]
                    + self.data[Self::index(i, 1)] * other.data[Self::index(1, j)]
                    + self.data[Self::index(i, 2)] * other.data[Self::index(2, j)]
                    + self.data[Self::index(i, 3)] * other.data[Self::index(3, j)];
            }
        }
        result
    }
}
