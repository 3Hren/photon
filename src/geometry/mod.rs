use crate::{ray::Ray, Intersection, Material};

pub use self::{
    mesh::{Mesh, Triangle},
    plane::Plane,
    sphere::Sphere,
};
use crate::transform::Transform;

mod mesh;
mod plane;
mod sphere;

pub trait Geometry: Transform<f64> {
    fn intersection(&self, ray: &Ray<f64>) -> Option<Intersection>;
}

pub struct Model<G> {
    pub geometry: G,
    pub material: Material,
}
