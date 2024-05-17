use image::{Rgb, RgbImage};
use indicatif::ParallelProgressIterator;
use na::{Point3, Vector3};
use rayon::prelude::*;

use crate::geometry::interval::Interval;
use crate::geometry::ray::{Hit, Hittable, Ray};
use crate::materials::Scattered;

#[derive(Debug)]
pub struct Camera {
    focal_length: f64,
    image_width: u32,
    image_height: u32,
    viewport_width: f64,
    viewport_height: f64,
    camera_center: Point3<f64>,
    vertical_fov: f64,
    samples_per_pixel: usize,
    max_depth: isize,
}

impl Camera {
    pub fn from_image_size_and_camera(
        image_width: u32,
        image_height: u32,
        focal_length: f64,
        camera_center: Point3<f64>,
        vertical_fov: f64,
        samples_per_pixel: usize,
    ) -> Self {
        let theta = vertical_fov.to_radians();
        let h = (theta / 2.).tan();

        let viewport_height = 2. * h * focal_length;
        let viewport_width = viewport_height * ((image_width as f64) / (image_height as f64));
        Self {
            focal_length,
            image_width,
            image_height,
            viewport_width,
            viewport_height,
            camera_center,
            vertical_fov,
            samples_per_pixel,
            max_depth: 50,
        }
    }

    fn u(&self) -> Vector3<f64> {
        Vector3::new(self.viewport_width, 0f64, 0f64)
    }

    fn v(&self) -> Vector3<f64> {
        Vector3::new(0f64, -self.viewport_height, 0f64)
    }

    fn delta_u(&self) -> Vector3<f64> {
        self.u() / (self.image_width as f64)
    }

    fn delta_v(&self) -> Vector3<f64> {
        self.v() / (self.image_height as f64)
    }

    fn pixel_0_0_at(&self) -> Point3<f64> {
        let viewport_upper_left = self.camera_center
            - Vector3::new(0f64, 0f64, self.focal_length)
            - self.u() / 2f64
            - self.v() / 2f64;
        viewport_upper_left + 0.5 * (self.delta_u() + self.delta_v())
    }

    pub fn render<T: Hittable>(&self, world: T) -> RgbImage {
        let mut output = RgbImage::new(self.image_width, self.image_height);

        output
            .par_enumerate_pixels_mut()
            .progress()
            .for_each(|(x_pos, y_pos, pixel)| {
                let x = x_pos as f64;
                let y = y_pos as f64;

                let mut color_vector = Vector3::new(0., 0., 0.);

                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(x, y);
                    color_vector += self.ray_color(&world, &ray, self.max_depth)
                }

                color_vector.unscale_mut(self.samples_per_pixel as f64);
                *pixel = vector_to_color(&color_vector);
            });

        output
    }

    fn ray_color(&self, world: &impl Hittable, ray: &Ray, max_depth: isize) -> Vector3<f64> {
        if max_depth <= 0 {
            return Vector3::new(0., 0., 0.);
        }

        let intersection = world.hits(ray, Interval::new(0.001, f64::INFINITY));
        if let Some(hit) = &intersection {
            let Hit { material, .. } = &hit;

            return if let Some(Scattered {
                attenuation,
                scatter_ray,
            }) = material.scatter(ray, &hit)
            {
                let color: Vector3<f64> = self.ray_color(world, &scatter_ray, max_depth - 1);
                attenuation.component_mul(&color)
            } else {
                Vector3::new(0., 0., 0.)
            };
        }

        let unit_direction = ray.direction().normalize();
        let a = 0.5 * (unit_direction.y + 1.);

        Vector3::new(1. - 0.5 * a, 1. - 0.3 * a, 1.)
    }

    fn get_ray(&self, x: f64, y: f64) -> Ray {
        let offset = sample_for_pixel();

        let pixel_center =
            self.pixel_0_0_at() + (x + offset.x) * self.delta_u() + (y + offset.y) * self.delta_v();
        let ray_direction = pixel_center - self.camera_center;
        Ray::new(self.camera_center, ray_direction)
    }
}

fn sample_for_pixel() -> Vector3<f64> {
    Vector3::new(rand::random::<f64>() - 0.5, rand::random::<f64>() - 0.5, 0.)
}

fn vector_to_color(vec: &Vector3<f64>) -> Rgb<u8> {
    let valid_intensity = Interval::new(0., 0.999);

    let r = vec.x.powf(1. / 2.2);
    let g = vec.y.powf(1. / 2.2);
    let b = vec.z.powf(1. / 2.2);

    Rgb([
        (256. * valid_intensity.clamp(r)) as u8,
        (256. * valid_intensity.clamp(g)) as u8,
        (256. * valid_intensity.clamp(b)) as u8,
    ])
}
