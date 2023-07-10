use crate::{
    material::{Material, Reflection},
    ray::Ray,
    vector::{Point, Vec3},
};
use derive_more::Constructor;

#[derive(Debug, Constructor)]
pub struct Sphere<M: Material> {
    pub centre: Point,
    pub radius: f64,
    pub material: M,
}

pub struct Hit {
    pub point: Point,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub reflection: Option<Reflection>,
}

/// Represents objects within the scene
pub trait Object {
    /// Determines if an object has been hit by a ray
    fn hit(&self, ray: &Ray, bounds: (f64, f64)) -> Option<Hit>;
}

pub type Scene = Vec<Box<dyn Object + Sync>>;

impl<M: Material> Object for Sphere<M> {
    fn hit(&self, ray: &Ray, bounds: (f64, f64)) -> Option<Hit> {
        let oc = ray.origin - self.centre;
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * ray.direction.dot(&oc);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let d = b * b - 4.0 * a * c;
        if d >= 0.0 {
            let mut root = (-b - d.sqrt()) / (2.0 * a); // Negative root closer
            if !(bounds.0..bounds.1).contains(&root) {
                root = (-b + d.sqrt()) / (2.0 * a); // If not in bounds, try positive root
                if !(bounds.0..bounds.1).contains(&root) {
                    return None; // If not in bounds, return None
                }
            }
            let point = ray.at(root);
            let normal = (point - self.centre).normalise(); // Normalise normal
            let (normal, front_face) = if ray.direction.dot(&normal) > 0.0 {
                (-normal, false) // Negative normal if ray inside sphere
            } else {
                (normal, true)
            };
            let mut h = Hit {
                point,
                normal,
                t: root,
                front_face,
                reflection: None,
            };
            h.reflection = self.material.scatter(ray, &h);
            Some(h)
        } else {
            return None;
        }
    }
}

impl Object for Scene {
    /// Returns the hit closest to the camera
    fn hit(&self, ray: &Ray, bounds: (f64, f64)) -> Option<Hit> {
        self.iter()
            .filter_map(|object| object.hit(ray, bounds))
            .min_by(|a, b| a.t.partial_cmp(&b.t).unwrap())
    }
}
