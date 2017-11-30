use {Intersection, Material};
use ray::Ray;

mod mesh;
mod plane;
mod sphere;

pub use self::mesh::{Mesh, Triangle};
pub use self::plane::Plane;
pub use self::sphere::Sphere;

pub trait Geometry {
    fn intersection(&self, ray: &Ray<f64>) -> Option<Intersection>;
}

pub struct Model<G> {
    pub geometry: G,
    pub material: Material,
}

