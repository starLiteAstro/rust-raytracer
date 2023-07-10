mod camera;
mod material;
mod object;
mod ray;
mod vector;

use camera::Camera;
use material::{Dielectric, Lambertian, Metal};
use object::{Scene, Sphere};
use vector::Vec3;

use image::RgbImage;
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressFinish, ProgressStyle};
use rand::prelude::*;
use rayon::prelude::*;

fn main() {
    // Image properties
    let aspect_ratio = 1.5;
    let width: u32 = 1200;
    let height = (width as f64 / aspect_ratio) as u32;
    let samples = 100; // Number of samples per pixel
    let max_depth = 50; // Maximum number of bounces

    // Camera
    let look_from = v!(13, 2, 3);
    let look_at = v!(0, 0, 0);
    let camera = Camera::new(
        look_from,
        look_at,
        v!(0, 1, 0),
        20.0,
        aspect_ratio,
        0.1,
        10.0,
    );

    // Scene
    let scene = random_scene();

    // Progress bar
    let bar = ProgressBar::new((width * height) as u64);
    bar.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.yellow} [{wide_bar:.cyan/blue}] {percent}% - {elapsed_precise} elapsed {msg}",
            )
            .progress_chars("#💀-")
            .on_finish(ProgressFinish::WithMessage("-- Done!".into())),
    );
    bar.set_draw_rate(5);

    // Create image buffer
    let mut buffer = RgbImage::new(width, height);
    // Render loop
    buffer
        .enumerate_pixels_mut() // Iterate over pixels
        .par_bridge() // Bridge to parallel iterator
        .progress_with(bar)
        .for_each(|(x, y, px)| {
            let mut colour = v!(0);
            for _ in 0..samples {
                let u = (x as f64 + random::<f64>()) / (width - 1) as f64;
                let v = (y as f64 + random::<f64>()) / (height - 1) as f64;
                let ray = camera::Camera::get_ray(&camera, u, v);
                colour = colour + ray::colour(&scene, &ray, max_depth);
            }
            // Save pixel colour to buffer
            *px = (colour / samples as f64).into();
        });
    buffer.save("render.png").expect("Failed to save image");
}

fn random_scene() -> Scene {
    let mut objects: Scene = vec![];

    let ground = Box::new(Sphere::new(
        v!(0, -1000, 0),
        1000.0,
        Lambertian::new(v!(0.5, 0.5, 0.5)),
    ));
    objects.push(ground);

    for a in -11..11 {
        for b in -11..11 {
            let a = a as f64;
            let b = b as f64;
            let material_choice: f64 = rand::random();
            let center = v!(
                a + 0.9 * rand::random::<f64>(),
                0.2,
                b + 0.9 * rand::random::<f64>()
            );

            if material_choice < 0.8 {
                //diffuse
                let material = Lambertian::new(v!(rand::random::<f64>()));
                objects.push(Box::new(Sphere::new(center, 0.2, material)));
            } else if material_choice < 0.95 {
                //metal
                let colour = v!(rand::random::<f64>() / 2.0 + 0.5);
                let fuzz = rand::random::<f64>() / 2.0;
                let material = Metal::new(colour, fuzz);
                objects.push(Box::new(Sphere::new(center, 0.2, material)));
            } else {
                //glass
                objects.push(Box::new(Sphere::new(center, 0.2, Dielectric::new(1.5))));
            }
        }
    }

    objects.push(Box::new(Sphere::new(
        v!(0, 1, 0),
        1.0,
        Dielectric::new(1.5),
    )));
    objects.push(Box::new(Sphere::new(
        v!(-4, 1, 0),
        1.0,
        Lambertian::new(v!(0.4, 0.2, 0.1)),
    )));
    objects.push(Box::new(Sphere::new(
        v!(4, 1, 0),
        1.0,
        Metal::new(v!(0.7, 0.6, 0.5), 0.0),
    )));
    objects
}
