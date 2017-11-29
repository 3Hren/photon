//! Model that contains one or more triangles.

use {Intersection, Material, Ray};
use geometry::Geometry;
use vec3::Vec3;

#[derive(Copy, Clone, Debug)]
pub struct Triangle<T> {
    ///
    vertices: [Vec3<T>; 3],

    ///
    /// All the same if our triangle is *flat*.
    /// Values differ when we want interpolation. e.g. round things like teapot.
    normals: [Vec3<T>; 3],
}

impl<T> Geometry for Triangle<T> {
    fn intersection(&self, ray: &Ray<f64>) -> Option<Intersection> {
        unimplemented!();
    }
}
