use {Intersection, Material};
use ray::Ray;

mod mesh;

pub trait Model {
    fn material(&self) -> &Material;
    fn intersection(&self, ray: &Ray<f64>) -> Option<Intersection>;
}
