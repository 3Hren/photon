use std::ops::{Add, Index};

use vec3::Vec3;

#[derive(Copy, Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Vec4<T>([T; 4]);

impl<T> Vec4<T> {
    #[inline]
    pub fn new(x: T, y: T, z: T, w: T) -> Self {
        Vec4([x, y, z, w])
    }

    #[inline]
    pub fn x(&self) -> &T {
        self.index(0)
    }

    #[inline]
    pub fn y(&self) -> &T {
        self.index(1)
    }

    #[inline]
    pub fn z(&self) -> &T {
        self.index(2)
    }

    #[inline]
    pub fn w(&self) -> &T {
        self.index(3)
    }
}

impl<T: Copy> From<[T; 4]> for Vec4<T> {
    #[inline]
    fn from(v: [T; 4]) -> Self {
        Vec4::new(v[0], v[1], v[2], v[3])
    }
}

impl From<Vec3<f64>> for Vec4<f64> {
    #[inline]
    fn from(v: Vec3<f64>) -> Self {
        Vec4::new(v.x, v.y, v.z, 1.0)
    }
}

impl Into<Vec3<f64>> for Vec4<f64> {
    fn into(self) -> Vec3<f64> {
        Vec3::new(*self.x(), *self.y(), *self.z())
    }
}

impl<T: Copy + Add<Output = T> + Default> Vec4<T> {
    #[inline]
    pub fn sum(&self) -> T {
        self.0.iter().fold(Default::default(), |s, &v| s + v)
    }
}

impl<T> Index<usize> for Vec4<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
