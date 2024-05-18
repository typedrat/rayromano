use crate::geometry::ray::{Hit, Ray};
use crate::materials::{Material, Scattered};
use crate::util::{color, reflect_vector};
use na::Vector3;

#[derive(Copy, Clone, Debug)]
pub struct Dielectric {
    refractive_index: f64,
    tint: Vector3<f64>,
}

impl Dielectric {
    #[allow(unused)]
    pub fn new_with_tint(refractive_index: f64, tint: Vector3<f64>) -> Self {
        Dielectric {
            refractive_index,
            tint,
        }
    }

    #[allow(unused)]
    pub fn new(refractive_index: f64) -> Self {
        Self::new_with_tint(refractive_index, color(1., 1., 1.))
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<Scattered> {
        let attenuation = self.tint;
        let refractive_index = if hit.front_face {
            1. / self.refractive_index
        } else {
            self.refractive_index
        };

        let unit_direction = ray.direction().normalize();
        let cos_theta = (-unit_direction).dot(&hit.normal).min(1.0);
        let sin_theta = (1. - cos_theta * cos_theta).sqrt();

        let ray_direction = if refractive_index * sin_theta > 1.0 {
            // Must reflect
            reflect_vector(&unit_direction, &hit.normal)
        } else {
            // Can refract
            refract_vector(&unit_direction, &hit.normal, refractive_index)
        };

        let scatter_ray = Ray::new(hit.point, ray_direction);

        Some(Scattered {
            attenuation,
            scatter_ray,
        })
    }
}

fn refract_vector(
    direction: &Vector3<f64>,
    normal: &Vector3<f64>,
    refractive_index: f64,
) -> Vector3<f64> {
    let cos_theta = (-direction).dot(normal).min(1.0);
    let perp = refractive_index * (direction + cos_theta * normal);
    let parallel = -(1.0 - perp.magnitude_squared()).abs().sqrt() * normal;

    perp + parallel
}
