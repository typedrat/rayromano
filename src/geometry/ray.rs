use crate::geometry::interval::Interval;
use crate::materials::Material;
use na::{Point3, Unit, Vector3};

#[derive(Copy, Clone, Debug)]
/// A ray to be traced.
pub struct Ray {
    origin: Point3<f64>,
    direction: Vector3<f64>,
}

impl Ray {
    /// Create a new ray, given its origin and direction.
    pub fn new(origin: Point3<f64>, direction: Vector3<f64>) -> Self {
        Self { origin, direction }
    }

    /// Get the ray's origin.
    pub fn origin(&self) -> &Point3<f64> {
        &self.origin
    }

    /// Get the ray's direction.
    pub fn direction(&self) -> &Vector3<f64> {
        &self.direction
    }

    /// Get a point at distance `t` along the ray.
    pub fn at(&self, t: f64) -> Point3<f64> {
        &self.origin + &self.direction * t
    }
}

pub struct Hit {
    pub point: Point3<f64>,
    pub t: f64,
    pub normal: Unit<Vector3<f64>>,
    pub material: Box<dyn Material>,
    pub front_face: bool,
}

impl Hit {
    pub fn new(
        direction: &Vector3<f64>,
        point: Point3<f64>,
        t: f64,
        normal: Unit<Vector3<f64>>,
        material: Box<dyn Material>,
    ) -> Self {
        let front_face = direction.dot(&normal.into_inner()) < 0.;
        let normal = if front_face { normal } else { -normal };

        Self {
            point,
            t,
            normal,
            material,
            front_face,
        }
    }
}

pub trait Hittable: Send + Sync {
    fn hits(&self, ray: &Ray, t_interval: Interval) -> Option<Hit>;
}

impl Hittable for Vec<Box<dyn Hittable>> {
    fn hits(&self, ray: &Ray, t_interval: Interval) -> Option<Hit> {
        let mut best_hit = None;
        let mut closest_so_far = t_interval.max;

        for hittable in self {
            let possible_hit = hittable.hits(ray, Interval::new(t_interval.min, closest_so_far));

            if let Some(Hit { t, .. }) = possible_hit {
                best_hit = possible_hit;
                closest_so_far = t;
            }
        }

        best_hit
    }
}
