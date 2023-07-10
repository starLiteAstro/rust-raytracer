use derive_more::{Add, Constructor, Div, Mul, Neg, Sub};
use std::ops::Div;
use std::ops::Mul;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Add, Sub, Mul, Div, Neg, Constructor)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[macro_export]
macro_rules! v {
    ($x: expr, $y: expr, $z: expr) => {
        Vec3::new(f64::from($x), f64::from($y), f64::from($z))
    };
    ($x: expr) => {
        Vec3::new(f64::from($x), f64::from($x), f64::from($x))
    };
}

pub type Colour = Vec3;
pub type Point = Vec3;

impl Vec3 {
    /// Dot product
    pub fn dot(&self, other: &Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Magnitude
    pub fn len(&self) -> f64 {
        self.dot(self).sqrt()
    }

    /// Cross product
    pub fn cross(&self, other: &Vec3) -> Self {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    /// Applies function to each element of vector
    pub fn map<F>(&self, mut f: F) -> Vec3
    where
        F: FnMut(f64) -> f64,
    {
        Vec3 {
            x: f(self.x),
            y: f(self.y),
            z: f(self.z),
        }
    }

    /// Scales vector to unit length
    pub fn normalise(&self) -> Self {
        self.map(|x| x / self.len())
    }

    /// Returns random vector in unit sphere
    pub fn rand_unit() -> Self {
        loop {
            // Random range [0; 1], scale to [-1; 1]
            let v = v!(rand::random::<f64>() * 2.0 - 1.0);
            if v.len() < 1.0 {
                break v.normalise(); // If vector lies on sphere, normalise and return
            }
        }
    }

    /// Checks if vector is near zero
    pub fn near_zero(&self) -> bool {
        let s = 1e-8; // Tolerance
        self.x.abs() < s && self.y.abs() < s && self.z.abs() < s
    }
}

impl From<Vec3> for image::Rgb<u8> {
    fn from(v: Vec3) -> Self {
        image::Rgb(
            [v.x, v.y, v.z]
                .map(|c| c.sqrt())
                .map(|c| (c * 255.999) as u8),
        )
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Self::Output {
        v.map(|x| x * self)
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Self::Output {
        Vec3 {
            x: self.x * v.x,
            y: self.y * v.y,
            z: self.z * v.z,
        }
    }
}

impl Div for Vec3 {
    type Output = Vec3;

    fn div(self, v: Vec3) -> Self::Output {
        Vec3 {
            x: self.x / v.x,
            y: self.y / v.y,
            z: self.z / v.z,
        }
    }
}
