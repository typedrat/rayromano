extern crate nalgebra as na;
use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

mod camera;

use crate::camera::Camera;
use std::f64::consts::PI;

mod geometry;
mod materials;
mod util;

use geometry::ray::Hittable;
use geometry::sphere::Sphere;
use util::color;

use crate::materials::*;
use anyhow::Result;
use na::Point3;

fn main() -> Result<()> {
    let width = 640;
    let height = 360;

    println!(
        "Resolution = {}x{}, aspect ratio = {}",
        width,
        height,
        readable_aspect_ratio(width, height)
    );

    let camera_center = Point3::new(0f64, 0f64, 0f64);
    let camera = Camera::from_image_size_and_camera(width, height, 1.0, camera_center, 90., 100);

    let r = (PI / 4.).cos();

    let material_left = Lambertian::new(color(0., 0., 1.));
    let material_right = Lambertian::new(color(1., 0., 0.));

    let world: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere::new(Point3::new(-r, 0., -1.), r, material_left)),
        Box::new(Sphere::new(Point3::new(r, 0., -1.), r, material_right)),
    ];

    let output = camera.render(world);
    output.save("output.png")?;

    Ok(())
}

fn readable_aspect_ratio(width: u32, height: u32) -> String {
    let mut out = format!("{:.4}", (width as f32) / (height as f32));
    let len = out.trim_end_matches('0').trim_end_matches('.').len();
    out.truncate(len);
    out
}
