use buildstructor::buildstructor;
use image::{Rgb, RgbImage};
use indicatif::ParallelProgressIterator;
use na::{Point3, Unit, Vector3};
use rayon::prelude::*;

use crate::geometry::interval::Interval;
use crate::geometry::ray::{Hit, Hittable, Ray};
use crate::materials::Scattered;
use crate::util::random_in_unit_disk;

#[allow(clippy::struct_field_names)]
#[derive(Debug)]
pub struct Camera {
    focus_dist: f64,
    image_width: u32,
    image_height: u32,
    viewport_width: f64,
    viewport_height: f64,
    look_from: Point3<f64>,
    defocus_angle: f64,
    defocus_disk_u: Vector3<f64>,
    defocus_disk_v: Vector3<f64>,
    samples_per_pixel: usize,
    max_depth: usize,

    camera_u: Unit<Vector3<f64>>,
    camera_v: Unit<Vector3<f64>>,
    camera_w: Unit<Vector3<f64>>,
}

#[buildstructor]
impl Camera {
    #[builder]
    pub fn new(
        image_size: (u32, u32),
        vertical_fov: Option<f64>,
        look_from: Option<Point3<f64>>,
        look_at: Option<Point3<f64>>,
        up_vector: Option<Vector3<f64>>,
        focus_dist: Option<f64>,
        defocus_angle: Option<f64>,
        samples_per_pixel: Option<usize>,
        max_depth: Option<usize>,
    ) -> Self {
        let (image_width, image_height) = image_size;
        // The vertical equivalent of 90 deg. FOV
        let vertical_fov = vertical_fov.unwrap_or(60.);
        let theta = vertical_fov.to_radians();
        let h = (theta / 2.).tan();

        let look_from = look_from.unwrap_or(Point3::new(0., 0., 0.));
        let look_at = look_at.unwrap_or(Point3::new(0., 0., -1.));
        let up_vector = up_vector.unwrap_or(Vector3::new(0., 1., 0.));
        let focus_dist = focus_dist.unwrap_or(10.);
        let defocus_angle = defocus_angle.unwrap_or(0.);
        let defocus_angle = (defocus_angle / 2.).to_radians();

        let camera_w = Unit::new_normalize(look_from - look_at);
        let camera_u = Unit::new_normalize(up_vector.cross(&camera_w));
        let camera_v = Unit::new_normalize(camera_w.cross(&camera_u));

        let defocus_radius = focus_dist * f64::tan(defocus_angle);
        let defocus_disk_u = defocus_radius * camera_u.into_inner();
        let defocus_disk_v = defocus_radius * camera_v.into_inner();

        let viewport_height = 2. * h * focus_dist;
        let viewport_width = viewport_height * (f64::from(image_width) / f64::from(image_height));
        let samples_per_pixel = samples_per_pixel.unwrap_or(100);
        let max_depth = max_depth.unwrap_or(10);
        Self {
            focus_dist,
            image_width,
            image_height,
            viewport_width,
            viewport_height,
            look_from,
            defocus_angle,
            defocus_disk_u,
            defocus_disk_v,
            samples_per_pixel,
            max_depth,
            camera_u,
            camera_v,
            camera_w,
        }
    }
}

impl Camera {
    fn viewport_u(&self) -> Vector3<f64> {
        self.viewport_width * self.camera_u.into_inner()
    }

    fn viewport_v(&self) -> Vector3<f64> {
        self.viewport_height * -self.camera_v.into_inner()
    }

    fn viewport_delta_u(&self) -> Vector3<f64> {
        self.viewport_u() / f64::from(self.image_width)
    }

    fn viewport_delta_v(&self) -> Vector3<f64> {
        self.viewport_v() / f64::from(self.image_height)
    }

    fn pixel_0_0_at(&self) -> Point3<f64> {
        let viewport_upper_left = self.look_from
            - self.focus_dist * self.camera_w.into_inner()
            - self.viewport_u() / 2.
            - self.viewport_v() / 2.;
        viewport_upper_left + 0.5 * (self.viewport_delta_u() + self.viewport_delta_v())
    }

    pub fn render<T: Hittable>(&self, world: &T) -> RgbImage {
        let mut output = RgbImage::new(self.image_width, self.image_height);

        output
            .par_enumerate_pixels_mut()
            .progress()
            .for_each(|(x_pos, y_pos, pixel)| {
                let x = f64::from(x_pos);
                let y = f64::from(y_pos);

                let mut color_vector = Vector3::new(0., 0., 0.);

                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(x, y);
                    color_vector += Self::ray_color(world, &ray, self.max_depth);
                }

                #[allow(clippy::cast_precision_loss)]
                color_vector.unscale_mut(self.samples_per_pixel as f64);
                *pixel = vector_to_color(&color_vector);
            });

        output
    }

    fn ray_color(world: &impl Hittable, ray: &Ray, max_depth: usize) -> Vector3<f64> {
        if max_depth == 0 {
            return Vector3::new(0., 0., 0.);
        }

        let intersection = world.hits(ray, Interval::new(0.001, f64::INFINITY));
        if let Some(hit) = &intersection {
            let Hit { material, .. } = &hit;

            return if let Some(Scattered {
                attenuation,
                scatter_ray,
            }) = material.scatter(ray, hit)
            {
                let color: Vector3<f64> = Self::ray_color(world, &scatter_ray, max_depth - 1);
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
        let pixel_center = self.pixel_0_0_at()
            + (x + offset.x) * self.viewport_delta_u()
            + (y + offset.y) * self.viewport_delta_v();

        let ray_origin = if self.defocus_angle <= 0. {
            self.look_from
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_center - ray_origin;

        Ray::new(self.look_from, ray_direction)
    }

    fn defocus_disk_sample(&self) -> Point3<f64> {
        let p = random_in_unit_disk();
        self.look_from + (p.x * self.defocus_disk_u) + (p.y * self.defocus_disk_v)
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

    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    Rgb([
        (256. * valid_intensity.clamp(r)) as u8,
        (256. * valid_intensity.clamp(g)) as u8,
        (256. * valid_intensity.clamp(b)) as u8,
    ])
}
