use {Intersection, Ray};
use geometry::Geometry;
use matrix::Matrix4x4;
use transform::Transform;
use vec3::Vec3;
use vec4::Vec4;

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct Sphere {
    center: Vec3<f64>,
    radius: f64,
}

impl Geometry for Sphere {
    fn intersection(&self, ray: &Ray<f64>) -> Option<Intersection> {
        let oc = ray.origin() - self.center;

        let a = ray.direction().dot(ray.direction());
        let b = 2.0 * oc.dot(ray.direction());
        let c = oc.dot(&oc) - self.radius.powi(2);

        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            return None
        }

        let sqrt = discriminant.sqrt();
        let denominator = 2.0 * a;

        let x1 = (-b + sqrt) / denominator;
        let x2 = (-b - sqrt) / denominator;

        let t = if x1 < x2 {
            x1
        } else {
            x2
        };

        let intersection = ray.offset(t);
        let normal = (intersection - self.center).unit();

        return Some(Intersection::new(t, intersection, normal));
    }
}

impl Transform<f64> for Sphere {
    fn transform(&mut self, transformation: &Matrix4x4<f64>) {
        let vec4 = Vec4::new(self.center.x, self.center.y, self.center.z, 1.0);
        let vec4 = transformation * vec4;
        self.center.x = *vec4.x();
        self.center.y = *vec4.y();
        self.center.z = *vec4.z();
    }
}
