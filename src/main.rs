extern crate nalgebra as na;
use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

mod camera;
use crate::camera::Camera;

mod geometry;
use geometry::ray::Hittable;
use geometry::sphere::Sphere;

mod materials;
use crate::materials::*;

mod util;
use util::color;

use anyhow::Result;
use na::Point3;
use std::f64::consts::PI;

fn main() -> Result<()> {
    let width = 640;
    let height = 360;

    println!(
        "Resolution = {}x{}, aspect ratio = {}",
        width,
        height,
        readable_aspect_ratio(width, height)
    );

    let camera = Camera::builder()
        .image_size((width, height))
        .samples_per_pixel(500)
        .build();

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
