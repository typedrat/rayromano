use na::{Unit, UnitVector3, Vector3};
use rand::{thread_rng, Rng};
use std::f64::consts::PI;

pub fn random_unit_vector() -> UnitVector3<f64> {
    Unit::new_normalize(Vector3::new(
        rand::random::<f64>(),
        rand::random::<f64>(),
        rand::random::<f64>(),
    ))
}

pub fn random_in_unit_disk() -> Vector3<f64> {
    let mut rng = thread_rng();

    let theta = rng.gen_range(0. ..2.0 * PI);
    let r = rng.gen::<f64>().sqrt();

    let x = r * theta.cos();
    let y = r * theta.sin();

    Vector3::new(x, y, 0.)
}

pub fn reflect_vector(v: &Vector3<f64>, n: &Vector3<f64>) -> Vector3<f64> {
    v - 2. * v.dot(n) * n
}

pub fn color(r: f64, g: f64, b: f64) -> Vector3<f64> {
    Vector3::new(r, g, b)
}
