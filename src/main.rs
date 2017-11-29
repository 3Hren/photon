#![feature(range_contains)]

//extern crate serde_json;
extern crate image;

use std::f64;
use std::fs::File;
use std::time::SystemTime;

use image::{ImageBuffer, ImageRgb8, Pixel, Rgb};

type Point = Vec3<f64>;

mod geometry;
mod ray;
mod vec3;

use geometry::{Geometry, Mesh, Model, Triangle};
use ray::Ray;
use vec3::Vec3;

#[derive(Copy, Clone, Debug)]
pub struct Intersection {
    t: f64,
    point: Vec3<f64>,
    normal: Vec3<f64>,
}

impl Intersection {
    pub fn new(t: f64, point: Vec3<f64>, normal: Vec3<f64>) -> Self {
        Self { t, point, normal }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Material {
    color: Rgb<u8>,
    reflective: f64,
}

///
///
/// A plane can be defined as a point representing how far the plane is from the world origin and a
/// normal (defining the orientation of the plane).
#[derive(Copy, Clone, Debug)]
struct Plane {
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

#[derive(Copy, Clone, Debug)]
struct Sphere {
    center: Point,
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

        let intersection = ray.origin() + ray.direction().scale(t);
        let normal = (intersection - self.center).unit();

        return Some(Intersection::new(t, intersection, normal));
    }
}

trait Light {
    fn pos(&self) -> Vec3<f64>;
    fn intensity(&self, intersection: &Intersection) -> f64;
}

#[derive(Copy, Clone, Debug)]
struct PointLight {
    intensity: f64,
    position: Vec3<f64>,
}

impl Light for PointLight {
    fn pos(&self) -> Vec3<f64> {
        self.position
    }

    fn intensity(&self, intersection: &Intersection) -> f64 {
        let l = self.position - intersection.point;
        let r = intersection.normal.dot(&l);
        if r > 0.0 {
            self.intensity * r / (intersection.normal.len() * l.len())
        } else {
            0.0
        }
    }
}

struct Scene {
    lights: Vec<Box<Light>>,
    objects: Vec<Model<Box<Geometry>>>,

    depth: u16,
    background: Rgb<u8>,
}

impl Scene {
    pub fn new(background: Rgb<u8>) -> Self {
        Self {
            lights: Vec::new(),
            objects: Vec::new(),
            depth: 1,
            background,
        }
    }

//    pub fn load<P: AsRef<Path>>(path: &P) -> Result<Self, Box<Error>> {
//        unimplemented!()
//    }

    pub fn trace(&self, ray: &Ray<f64>) -> Rgb<u8> {
        self.trace_limited(ray, self.depth)
    }

    fn trace_limited(&self, ray: &Ray<f64>, depth: u16) -> Rgb<u8> {
        self.closest_intersection(ray).map(|(m, i)| {
            let intensity = self.lightning(&i);

            let reflective = m.material.reflective;

            let color = m.material.color.map(|c| {
                let color = c as f64 * intensity;

                if color > 255.0 {
                    255
                } else {
                    color as u8
                }
            });

            if depth <= 0 || reflective <= 0.0 {
                return color;
            }

            let n = i.normal.unit();
            let d = ray.direction().inverse();

            let direction = n.scale(2.0 * n.dot(&d)) - d;
            let ray = Ray::new(i.point, direction, 1.0e-6..1.0e20);
            let reflected_color = self.trace_limited(&ray, depth - 1);

            let cr = color.map(|c| (c as f64 * (1.0 - reflective)) as u8);
            let cl = reflected_color.map(|c| (c as f64 * reflective) as u8);


            Rgb([cr[0] + cl[0], cr[1] + cl[1], cr[2] + cl[2]])
        }).unwrap_or(self.background)
    }

    fn closest_intersection(&self, ray: &Ray<f64>) -> Option<(&Model<Box<Geometry>>, Intersection)> {
        let mut t = f64::INFINITY;
        let mut closest = None;

        for model in &self.objects {
            if let Some(intersection) = model.geometry.intersection(ray) {
                if intersection.t < t && ray.contains(intersection.t) {
                    t = intersection.t;
                    closest = Some((model, intersection));
                }
            }
        }

        closest
    }

    fn lightning(&self, intersection: &Intersection) -> f64 {
        let mut intensity = 0.0;
        for light in &self.lights {
            // Shadows.
            let direction = light.pos() - intersection.point;
            let ray = Ray::new(intersection.point, direction, 1.0e-6..1.0e20);
            if self.closest_intersection(&ray).is_some() {
                continue
            }

            intensity += light.intensity(&intersection);
        }

        intensity
    }
}

struct Viewport {
    width: f64,
    height: f64,
}

fn main() {
    let width = 800;
    let height = 800;

    let mut buf = ImageBuffer::new(width, height);

    let viewport = Viewport { width: 1.0, height: 1.0 };

    let mesh = Mesh::load("teapot.obj.txt").unwrap();

    let mut scene = Scene::new(Rgb([30, 30, 30]));

    scene.objects.push(Model {
        geometry: Box::new(Plane {
            point: Point::new(0.0, -1.0, 0.0),
            normal: Vec3::new(0.0, 1.0, 0.0).unit(),
        }),
        material: Material {
            color: Rgb([127, 127, 127]),
            reflective: 0.0,
        },
    });

    for t in mesh.triangles {
        scene.objects.push(Model {
            geometry: Box::new(t),
            material: Material {
                color: Rgb([127, 127, 127]),
                reflective: 0.2,
            },
        });
    }

    scene.objects.push(Model {
        geometry: Box::new(Sphere {
            center: Point::new(0.0, 0.0, 10.0),
            radius: 1.0,
        }),
        material: Material {
            color: Rgb([255, 140, 0]),
            reflective: 0.0,
        },
    });
    scene.objects.push(Model {
        geometry: Box::new(Sphere {
            center: Point::new(0.0, -1.0, 3.0),
            radius: 1.0,
        }),
        material: Material {
            color: Rgb([255, 99, 71]),
            reflective: 0.5,
        },
    });
    scene.objects.push(Model {
        geometry: Box::new(Sphere {
            center: Point::new(2.0, 0.0, 4.0),
            radius: 1.0,
        }),
        material: Material {
            color: Rgb([85, 107, 47]),
            reflective: 0.2,
        },
    });
    scene.objects.push(Model {
        geometry: Box::new(Sphere {
            center: Point::new(-2.0, 0.0, 4.0),
            radius: 1.0,
        }),
        material: Material {
            color: Rgb([135, 206, 250]),
            reflective: 0.9,
        },
    });

    let lights = 1;
    for id in 0..lights {
        let phi = 6.2830 * id as f64 / lights as f64;
        let radius = 0.5;
        scene.lights.push(Box::new(PointLight {
            intensity: 0.8 / lights as f64,
            position: Vec3::new(-10.5, 5.0, -2.0) + Vec3::new(radius * phi.cos(), 0.0, radius * phi.sin()),
        }));
    }

    let origin = Point::new(0.0, 0.0, -2.0);

    let now = SystemTime::now();
    println!("Start rendering ...");
    println!("  - models: {}", scene.objects.len());
    println!("  - lights: {}", scene.lights.len());
    for (n, (x, y, pixel)) in buf.enumerate_pixels_mut().enumerate() {
        let sx = x as f64 + width as f64 / -2.0;
        let sy = height as f64 / 2.0 - y as f64;

        let vx = sx * viewport.width / width as f64;
        let vy = sy * viewport.height / height as f64;
        let vz = 1.0;

        let ray = Ray::new(origin, Vec3::new(vx, vy, vz), 1.0..1.0e20);

        *pixel = scene.trace(&ray);
        if n as u32 % ((width * height) / 100) == 0 {
            println!("Progress: {}%", n as u32 / ((width * height) / 100));
        }
    }

    let elapsed = now.elapsed().unwrap();
    println!("Finished, elapsed: {} ms", (elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 * 1e-9) * 1e3);
    let file = &mut File::create("photon.png").unwrap();

    ImageRgb8(buf).save(file, image::PNG).unwrap();
}
