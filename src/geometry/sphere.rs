use crate::geometry::interval::Interval;
use crate::geometry::ray::{Hit, Hittable, Ray};
use crate::materials::Material;
use na::{Point3, Unit};

#[derive(Clone)]
pub struct Sphere {
    pub center: Point3<f64>,
    pub radius: f64,
    pub material: Box<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3<f64>, radius: f64, material: impl Material + 'static) -> Self {
        Self {
            center,
            radius,
            material: Box::new(material),
        }
    }
}

impl Hittable for Sphere {
    fn hits(&self, ray: &Ray, t_interval: Interval) -> Option<Hit> {
        let relative_center = self.center - ray.origin();
        let direction = ray.direction();

        let a = direction.magnitude_squared();
        let h = direction.dot(&relative_center);
        let c = relative_center.magnitude_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;

        if discriminant < 0. {
            None
        } else {
            let discriminant_sqrt = discriminant.sqrt();

            let mut root = (h - discriminant_sqrt) / a;
            if !t_interval.surrounds(root) {
                root = (h + discriminant_sqrt) / a;

                if !t_interval.surrounds(root) {
                    return None;
                }
            }

            let t = root;
            let direction = ray.direction();
            let point = ray.at(t);
            let normal = Unit::new_normalize((point - self.center) / self.radius);

            Some(Hit::new(direction, point, t, normal, self.material.clone()))
        }
    }
}
