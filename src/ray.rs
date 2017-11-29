use std::ops::Range;

use {Point, Vec3};

#[derive(Debug)]
pub struct Ray<T> {
    origin: Vec3<T>,
    direction: Vec3<T>,
    range: Range<T>,
}

impl Ray<f64> {
    pub fn new(origin: Vec3<f64>, direction: Vec3<f64>, range: Range<f64>) -> Self {
        Self { origin, direction, range }
    }

    #[inline]
    pub fn origin(&self) -> Vec3<f64> {
        self.origin
    }

    #[inline]
    pub fn direction(&self) -> &Vec3<f64> {
        &self.direction
    }
}

impl<T: PartialOrd> Ray<T> {
    #[inline]
    pub fn contains(&self, t: T) -> bool {
        self.range.contains(t)
    }
}
