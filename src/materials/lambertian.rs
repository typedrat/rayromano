use crate::geometry::ray::{Hit, Ray};
use crate::materials::{Material, Scattered};
use crate::util::random_unit_vector;
use na::Vector3;

#[derive(Copy, Clone, Debug)]
pub struct Lambertian {
    albedo: Vector3<f64>,
}

impl Lambertian {
    #[allow(unused)]
    pub fn new(albedo: Vector3<f64>) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, Hit { point, normal, .. }: &Hit) -> Option<Scattered> {
        let mut scatter_direction = normal.into_inner() + random_unit_vector().into_inner();
        if scatter_direction.magnitude_squared() < 1e-8 {
            scatter_direction = normal.into_inner()
        }

        let scatter_ray = Ray::new(*point, scatter_direction);

        Some(Scattered {
            attenuation: self.albedo,
            scatter_ray,
        })
    }
}
