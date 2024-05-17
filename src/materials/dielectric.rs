use crate::geometry::ray::{Hit, Ray};
use crate::materials::{Material, Scattered};
use na::Vector3;

#[derive(Copy, Clone, Debug)]
pub struct Dialectric {
    refractive_index: f64,
    tint: Vector3<f64>,
}

impl Dialectric {
    #[allow(unused)]
    pub fn new(refractive_index: f64, tint: Vector3<f64>) -> Self {
        Dialectric {
            refractive_index,
            tint,
        }
    }
}

impl Material for Dialectric {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<Scattered> {
        let attenuation = self.tint;
        let refractive_index = if hit.front_face {
            1. / self.refractive_index
        } else {
            self.refractive_index
        };

        let unit_direction = ray.direction().normalize();
        let refracted = refract(&unit_direction, &hit.normal, refractive_index);
        let scatter_ray = Ray::new(hit.point, refracted);

        Some(Scattered {
            attenuation,
            scatter_ray,
        })
    }
}

fn refract(direction: &Vector3<f64>, normal: &Vector3<f64>, refractive_index: f64) -> Vector3<f64> {
    let cos_theta = (-direction).dot(normal).min(1.0);
    let perp = refractive_index * (direction + cos_theta * normal);
    let parallel = -(1.0 - perp.magnitude_squared()).abs().sqrt() * normal;

    perp + parallel
}
