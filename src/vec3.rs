use std::ops::{Add, Div, Mul, Sub};

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Vec3<T> {
    x: T,
    y: T,
    z: T,
}

impl<T> Vec3<T> {
    #[inline]
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

impl<T: Copy + Mul<Output = T>> Vec3<T> {
    #[inline]
    pub fn scale(&self, factor: T) -> Vec3<T> {
        Vec3 {
            x: self.x * factor,
            y: self.y * factor,
            z: self.z * factor,
        }
    }
}

impl<T: Copy + Add<Output = T> + Mul<Output = T>> Vec3<T> {
    #[inline]
    pub fn dot(&self, other: &Vec3<T>) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl Vec3<f64> {
    #[inline]
    pub fn len(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    #[inline]
    pub fn unit(&self) -> Vec3<f64> {
        let len = self.len();

        Vec3 {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
        }
    }

    #[inline]
    pub fn inverse(&self) -> Vec3<f64> {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl<T: Add<Output = T>> Add for Vec3<T> {
    type Output = Vec3<T>;

    #[inline]
    fn add(self, other: Vec3<T>) -> Self::Output {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T: Sub<Output = T>> Sub for Vec3<T> {
    type Output = Vec3<T>;

    #[inline]
    fn sub(self, other: Vec3<T>) -> Self::Output {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}
