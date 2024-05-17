use crate::geometry::ray::{Hit, Ray};
use dyn_clone::DynClone;
use na::Vector3;

pub trait Material: Send + Sync + DynClone {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<Scattered>;
}

dyn_clone::clone_trait_object!(Material);

#[derive(Copy, Clone, Debug)]
pub struct Scattered {
    pub attenuation: Vector3<f64>,
    pub scatter_ray: Ray,
}
