use crate::geometry::ray::{Hit, Ray};
use crate::materials::{Material, Scattered};
use crate::util::{random_unit_vector, reflect_vector};
use na::Vector3;

#[derive(Copy, Clone, Debug)]
pub struct Metal {
    albedo: Vector3<f64>,
    fuzz: f64,
}

impl Metal {
    #[allow(unused)]
    pub fn new(albedo: Vector3<f64>, fuzz: f64) -> Self {
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, Hit { point, normal, .. }: &Hit) -> Option<Scattered> {
        let reflected = reflect_vector(ray.direction(), normal);
        let fuzzy_reflected =
            reflected.normalize() + (self.fuzz * random_unit_vector().into_inner());
        let scatter_ray = Ray::new(point.clone(), fuzzy_reflected);

        if fuzzy_reflected.dot(normal) > 0. {
            Some(Scattered {
                attenuation: self.albedo,
                scatter_ray,
            })
        } else {
            None
        }
    }
}
