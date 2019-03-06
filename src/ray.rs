use std::ops::Range;

use crate::{matrix::Matrix4x4, transform::Transform, vec4::Vec4, Vec3};

#[derive(Debug)]
pub struct Ray<T> {
    origin: Vec3<T>,
    direction: Vec3<T>,
    range: Range<T>,
}

impl Ray<f64> {
    pub fn new(origin: Vec3<f64>, direction: Vec3<f64>, range: Range<f64>) -> Self {
        Self {
            origin,
            direction: direction.unit(),
            range,
        }
    }

    #[inline]
    pub fn origin(&self) -> Vec3<f64> {
        self.origin
    }

    #[inline]
    pub fn direction(&self) -> &Vec3<f64> {
        &self.direction
    }

    #[inline]
    pub fn offset(&self, t: f64) -> Vec3<f64> {
        self.origin + self.direction.scale(t)
    }
}

impl<T: PartialOrd> Ray<T> {
    #[inline]
    pub fn contains(&self, t: T) -> bool {
        self.range.contains(&t)
    }
}

impl Transform<f64> for Ray<f64> {
    fn transform(&mut self, transformation: &Matrix4x4<f64>) {
        self.origin = (transformation * Vec4::from(self.origin)).into();
        self.direction = (transformation * Vec4::from(self.direction)).into();
    }
}
