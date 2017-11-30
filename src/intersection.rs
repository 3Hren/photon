use vec3::Vec3;

#[derive(Copy, Clone, Debug)]
pub struct Intersection {
    pub t: f64,
    pub point: Vec3<f64>,
    pub normal: Vec3<f64>,
}

impl Intersection {
    pub fn new(t: f64, point: Vec3<f64>, normal: Vec3<f64>) -> Self {
        Self { t, point, normal }
    }
}
