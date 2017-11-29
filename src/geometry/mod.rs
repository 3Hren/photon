use {Intersection, Material};
use ray::Ray;

mod mesh;

pub use self::mesh::{Mesh, Triangle};

pub trait Geometry {
    fn intersection(&self, ray: &Ray<f64>) -> Option<Intersection>;
}

pub struct Model<G> {
    pub geometry: G,
    pub material: Material,
}

