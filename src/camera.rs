use crate::{
    ray::Ray,
    v,
    vector::{Point, Vec3},
};
use rand::distributions::{Distribution, Uniform};

pub struct Camera {
    pub origin: Point,
    pub top_left: Point,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub lens_radius: f64,
}

impl Camera {
    pub fn new(
        look_from: Point,
        look_at: Point,
        vup: Vec3,
        fov: f64,
        aspect_ratio: f64,
        aperture_width: f64,
        focus_distance: f64,
    ) -> Self {
        let h = (fov.to_radians() / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        // Orthonormal basis
        let w = (look_from - look_at).normalise();
        let u = vup.cross(&w).normalise();
        let v = w.cross(&u);

        let origin: Point = look_from;
        let horizontal = u * viewport_width * focus_distance;
        let vertical = v * -viewport_height * focus_distance;
        let top_left = origin - horizontal / 2.0 - vertical / 2.0 - (w * focus_distance);

        Self {
            origin,
            top_left,
            horizontal,
            vertical,
            u,
            v,
            lens_radius: aperture_width / 2.0,
        }
    }

    /// Returns ray from camera to pixel at (i, j)
    pub fn get_ray(&self, i: f64, j: f64) -> Ray {
        let rand = rand_in_unit_circle() * self.lens_radius;
        let origin = self.origin + self.u * rand.x + self.v * rand.y;
        let px_pos = self.top_left + i * self.horizontal + j * self.vertical;
        Ray::new(origin, px_pos - origin)
    }
}

/// Returns random vector in unit circle
fn rand_in_unit_circle() -> Vec3 {
    let dist = Uniform::new_inclusive(-1.0, 1.0);
    let mut rng = rand::thread_rng();
    loop {
        // Random range [0; 1], scale to [-1; 1]
        let v = v!(dist.sample(&mut rng), dist.sample(&mut rng), 0);
        if v.len() < 1.0 {
            break v.normalise(); // If vector lies on sphere, normalise and return
        }
    }
}
