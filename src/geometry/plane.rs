use {Intersection, Ray};
use geometry::Geometry;
use vec3::Vec3;

///
///
/// A plane can be defined as a point representing how far the plane is from the world origin and a
/// normal (defining the orientation of the plane).
#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub struct Plane {
    point: Vec3<f64>,
    normal: Vec3<f64>,
}

impl Geometry for Plane {
    fn intersection(&self, ray: &Ray<f64>) -> Option<Intersection> {
        let denominator = self.normal.dot(ray.direction());

        if denominator.abs() >= 1e-6 {
            let p0r0 = self.point - ray.origin();
            let t = p0r0.dot(&self.normal) / denominator;
            Some(Intersection::new(t, ray.origin() + ray.direction().scale(t), self.normal))
        } else {
            None
        }
    }
}
