use crate::{
    object::Object,
    v,
    vector::{Colour, Point, Vec3},
};
use derive_more::Constructor;

#[derive(Debug, PartialEq, PartialOrd, Clone, Constructor)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vec3,
}

impl Ray {
    /// Returns point at distance t along ray
    pub fn at(&self, t: f64) -> Point {
        self.origin + t * self.direction
    }
}

/// Returns colour of ray
pub fn colour(scene: &impl Object, ray: &Ray, depth: u8) -> Colour {
    if depth == 0 {
        return v!(0);
    }
    if let Some(hit) = scene.hit(ray, (0.00001, f64::INFINITY)) {
        if let Some(reflection) = hit.reflection {
            let attenuation = reflection.attenuation;
            attenuation * colour(scene, &reflection.reflected_ray, depth - 1)
        } else {
            v!(0)
        }
    } else {
        let direction = ray.direction.normalise();
        let t = 0.5 * (direction.normalise().y + 1.0); // Scale from (-1; 1) to (0; 1)
        let white: Colour = v!(1);
        let blue: Colour = v!(0.5, 0.7, 1);
        (1.0 - t) * white + t * blue // Linear interpolation
    }
}
