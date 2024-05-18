extern crate nalgebra as na;
use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

mod camera;
use crate::camera::Camera;

mod geometry;
use crate::geometry::ray::Hittable;
use crate::geometry::sphere::Sphere;

mod materials;
use crate::materials::{Dielectric, Lambertian, Metal};

mod util;
use crate::util::{color, random_color};

use anyhow::Result;
use na::Point3;

fn main() -> Result<()> {
    let width = 1200;
    let height = 675;

    println!(
        "Resolution = {}x{}, aspect ratio = {}",
        width,
        height,
        readable_aspect_ratio(width, height)
    );

    let camera = Camera::builder()
        .image_size((width, height))
        .samples_per_pixel(500)
        .look_from(Point3::new(13., 2., 3.))
        .look_at(Point3::new(0., 0., 0.))
        .vertical_fov(20.)
        .defocus_angle(0.3)
        .focus_dist(10.)
        .build();

    let mut world: Vec<Box<dyn Hittable>> = Vec::new();

    let material_ground = Lambertian::new(color(0.5, 0.5, 0.5));
    world.push(Box::new(Sphere::new(
        Point3::new(0., -1000., 0.),
        1000.,
        material_ground,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let a = f64::from(a);
            let b = f64::from(b);

            let choose_mat = rand::random::<f64>();
            let center = Point3::new(
                a + 0.9 * rand::random::<f64>(),
                0.2,
                b + 0.9 * rand::random::<f64>(),
            );

            if (center - Point3::new(4., 0.2, 0.)).magnitude() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = random_color();
                    let material = Lambertian::new(albedo);

                    world.push(Box::new(Sphere::new(center, 0.2, material)));
                } else if choose_mat < 0.95 {
                    let albedo = color(
                        0.5 * rand::random::<f64>() + 0.5,
                        0.5 * rand::random::<f64>() + 0.5,
                        0.5 * rand::random::<f64>() + 0.5,
                    );
                    let fuzz = 0.5 * rand::random::<f64>() + 0.5;
                    let material = Metal::new(albedo, fuzz);

                    world.push(Box::new(Sphere::new(center, 0.2, material)));
                } else {
                    // Glass
                    let material = Dielectric::new(1.5);

                    world.push(Box::new(Sphere::new(center, 0.2, material)));
                }
            }
        }
    }

    let material_1 = Dielectric::new(1.5);
    world.push(Box::new(Sphere::new(
        Point3::new(0., 1., 0.),
        1.,
        material_1,
    )));

    let material_2 = Lambertian::new(color(0.4, 0.2, 0.1));
    world.push(Box::new(Sphere::new(
        Point3::new(-4., 1., 0.),
        1.,
        material_2,
    )));

    let material_3 = Metal::new(color(0.7, 0.6, 0.5), 0.0);
    world.push(Box::new(Sphere::new(
        Point3::new(4., 1., 0.),
        1.,
        material_3,
    )));

    let output = camera.render(&world);
    output.save("output.png")?;

    Ok(())
}

fn readable_aspect_ratio(width: u32, height: u32) -> String {
    let mut out = format!("{:.4}", f64::from(width) / f64::from(height));
    let len = out.trim_end_matches('0').trim_end_matches('.').len();
    out.truncate(len);
    out
}
