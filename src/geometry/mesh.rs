//! Model that contains one or more triangles.

use std::{
    error::Error,
    f64,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use crate::{geometry::Geometry, matrix::Matrix4x4, transform::Transform, vec3::Vec3, vec4::Vec4, Intersection, Ray};

#[derive(Copy, Clone, Debug)]
pub struct Triangle<T> {
    ///
    vertices: [Vec3<T>; 3],

    ///
    /// All the same if our triangle is *flat*.
    /// Values differ when we want interpolation. e.g. round things like teapot.
    normals: [Vec3<T>; 3],
}

impl Triangle<f64> {
    pub fn new(vertices: [Vec3<f64>; 3]) -> Self {
        let n = vertices[0].cross(&vertices[1]).unit();

        Self {
            vertices,
            normals: [n, n, n],
        }
    }

    pub fn with_normals(mut self, normals: [Vec3<f64>; 3]) -> Self {
        self.normals = normals;
        self
    }
}

impl Geometry for Triangle<f64> {
    fn intersection(&self, ray: &Ray<f64>) -> Option<Intersection> {
        let e1 = self.vertices[1] - self.vertices[0];
        let e2 = self.vertices[2] - self.vertices[0];
        let p = ray.direction().cross(&e2);
        let determinant = e1.dot(&p);

        // If determinant is near zero, ray lies in the plane of triangle.
        if determinant.abs() < f64::EPSILON {
            return None;
        }

        let inv_det = 1.0 / determinant;
        let s = ray.origin() - self.vertices[0];
        let beta = inv_det * s.dot(&p);
        if beta < 0.0 || beta > 1.0 {
            return None;
        }

        let q = s.cross(&e1);
        let gamma = inv_det * ray.direction().dot(&q);
        if gamma < 0.0 || beta + gamma > 1.0 {
            return None;
        }

        let t = inv_det * e2.dot(&q);

        if ray.contains(t) {
            let alpha = 1.0 - beta - gamma;

            // Interpolate normals at vertices to get normal
            let n = self.normals[0].scale(alpha) + self.normals[1].scale(beta) + self.normals[2].scale(gamma);

            Some(Intersection {
                t,
                normal: n,
                point: ray.offset(t),
            })
        } else {
            None
        }
    }
}

impl Transform<f64> for Triangle<f64> {
    fn transform(&mut self, transformation: &Matrix4x4<f64>) {
        self.vertices[0] = (transformation * Vec4::from(self.vertices[0])).into();
        self.vertices[1] = (transformation * Vec4::from(self.vertices[1])).into();
        self.vertices[2] = (transformation * Vec4::from(self.vertices[2])).into();

        let inverse = transformation.inverse();
        self.normals[0] = Matrix4x4::transform_normal(&self.normals[0], inverse);
        self.normals[1] = Matrix4x4::transform_normal(&self.normals[1], inverse);
        self.normals[2] = Matrix4x4::transform_normal(&self.normals[2], inverse);
    }
}

#[derive(Clone, Debug)]
pub struct Mesh {
    pub triangles: Vec<Triangle<f64>>,
}

impl Mesh {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, Box<Error>> {
        let file = File::open(path)?;
        let file = BufReader::new(file);

        let mut vertices: Vec<Vec3<f64>> = Vec::new();
        let mut normals: Vec<Vec3<f64>> = Vec::new();
        let mut triangles = Vec::new();

        for line in file.lines() {
            let line = line?;
            let tokens: Vec<&str> = line[..].split_whitespace().collect();
            if tokens.is_empty() {
                continue;
            }

            println!("{:?}", tokens.get(0));
            match tokens.get(0) {
                // Vertexes.
                Some(&"v") => match (tokens.get(1), tokens.get(2), tokens.get(3)) {
                    (Some(x), Some(y), Some(z)) => {
                        vertices.push(Vec3::new(x.parse()?, y.parse()?, z.parse()?));
                    }
                    (..) => return Err("invalid `v` token".into()),
                },
                Some(&"vn") => match (tokens.get(1), tokens.get(2), tokens.get(3)) {
                    (Some(x), Some(y), Some(z)) => {
                        normals.push(Vec3::new(x.parse()?, y.parse()?, z.parse()?));
                    }
                    (..) => return Err("invalid `vn` token".into()),
                },
                // Faces
                Some(&"f") => {
                    let tail = match tokens.split_first() {
                        Some((.., tail)) => tail,
                        None => {
                            return Err("face syntax of `obj` not supported or malformed".into());
                        }
                    };

                    let pairs: Vec<Vec<usize>> = tail
                        .iter()
                        .map(|token| {
                            let str_tokens: Vec<&str> = token.split('/').collect();
                            str_tokens
                                .iter()
                                .map(|str_tok| {
                                    match str_tok.parse::<usize>().ok() {
                                        Some(usize_tok) => usize_tok - 1, // Have to offset as OBJ is 1-indexed
                                        None => !0,                       // No data available/not supplied (eg. `//` as a token)
                                    }
                                })
                                .collect()
                        })
                        .collect();

                    triangles.push(
                        Triangle::new([vertices[pairs[0][0]], vertices[pairs[1][0]], vertices[pairs[2][0]]]).with_normals([
                            normals[pairs[0][2]],
                            normals[pairs[1][2]],
                            normals[pairs[2][2]],
                        ]),
                    );
                }
                Some(..) => {}
                None => {}
            }
        }

        let mesh = Self { triangles };

        Ok(mesh)
    }
}

impl Geometry for Mesh {
    fn intersection(&self, ray: &Ray<f64>) -> Option<Intersection> {
        let mut t = f64::INFINITY;
        let mut closest = None;

        for triangle in &self.triangles {
            if let Some(intersection) = triangle.intersection(ray) {
                if intersection.t < t && ray.contains(intersection.t) {
                    t = intersection.t;
                    closest = Some(intersection);
                }
            }
        }

        closest
    }
}

impl Transform<f64> for Mesh {
    fn transform(&mut self, transformation: &Matrix4x4<f64>) {
        for triangle in &mut self.triangles {
            triangle.transform(transformation);
        }
    }
}
