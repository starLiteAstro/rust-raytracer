use derive_more::Constructor;

use crate::{
    object::Hit,
    ray::Ray,
    v,
    vector::{Colour, Vec3},
};

#[derive(Debug, Constructor)]
pub struct Reflection {
    pub reflected_ray: Ray,
    pub attenuation: Colour,
}

#[derive(Debug, Constructor)]
pub struct Lambertian {
    pub attenuation: Colour,
}

#[derive(Debug, Constructor)]
pub struct Metal {
    pub attenuation: Colour,
    pub fuzz: f64,
}

#[derive(Debug, Constructor)]
pub struct Dielectric {
    pub refraction_ratio: f64,
}

pub trait Material {
    fn scatter(&self, incident_ray: &Ray, hit: &Hit) -> Option<Reflection>;
}

impl Material for Lambertian {
    /// Lambertian reflection
    fn scatter(&self, _: &Ray, hit: &Hit) -> Option<Reflection> {
        let mut scatter_direction = hit.normal + Vec3::rand_unit();
        if scatter_direction.near_zero() {
            scatter_direction = hit.normal; // Catch degenerate scatter direction
        }
        let reflected_ray = Ray::new(hit.point, scatter_direction);
        Some(Reflection::new(reflected_ray, self.attenuation))
    }
}

impl Material for Metal {
    /// Metal reflection
    fn scatter(&self, incident_ray: &Ray, hit: &Hit) -> Option<Reflection> {
        let reflected_direction =
            reflect(incident_ray.direction, &hit.normal) + self.fuzz * Vec3::rand_unit();
        let reflected_ray = Ray::new(hit.point, reflected_direction);
        if reflected_ray.direction.dot(&hit.normal) > 0.0 {
            Some(Reflection::new(reflected_ray, self.attenuation))
        } else {
            None
        }
    }
}

impl Material for Dielectric {
    /// Dielectric refraction
    fn scatter(&self, incident_ray: &Ray, hit: &Hit) -> Option<Reflection> {
        let ratio = if hit.front_face {
            1.0 / self.refraction_ratio
        } else {
            self.refraction_ratio
        };
        let unit_direction = incident_ray.direction.normalise();
        let cos_theta = -unit_direction.dot(&hit.normal); // Cosine of angle between ray and normal
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();
        let scatter_direction =
            if (ratio * sin_theta > 1.0) || schlick(cos_theta, ratio) > rand::random() {
                // Total internal reflection
                reflect(unit_direction, &hit.normal)
            } else {
                refract(unit_direction, &hit.normal, ratio)
            };
        let refracted_ray = Ray::new(hit.point, scatter_direction);
        Some(Reflection::new(refracted_ray, v!(1)))
    }
}

/// Reflects vector v about normal
fn reflect(v: Vec3, normal: &Vec3) -> Vec3 {
    v - 2.0 * v.dot(normal) * *normal
}

/// Refracts incident ray about normal
fn refract(incident_ray: Vec3, normal: &Vec3, ratio: f64) -> Vec3 {
    let cos_theta = -incident_ray.dot(normal).min(1.0);
    let r_out_perp = ratio * (incident_ray + cos_theta * *normal);
    let r_out_parallel = -(1.0 - r_out_perp.len().powi(2)).abs().sqrt() * *normal;
    r_out_perp + r_out_parallel
}

/// Schlick approximation
fn schlick(angle: f64, n: f64) -> f64 {
    let r0 = ((1.0 - n) / (1.0 + n)).powi(2);
    r0 + (1.0 - r0) * (1.0 - angle).powi(5)
}
