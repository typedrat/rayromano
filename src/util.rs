use na::{Unit, UnitVector3, Vector3};
use rand::{thread_rng, Rng};
use std::f64::consts::TAU;

pub fn random_unit_vector() -> UnitVector3<f64> {
    let u: f64 = 2. * rand::random::<f64>() - 1.;
    let phi: f64 = TAU * rand::random::<f64>();

    Unit::new_normalize(Vector3::new(
        phi.cos() * (1. - u.powf(2.)).powf(0.5),
        phi.sin() * (1. - u.powf(2.)).powf(0.5),
        u,
    ))
}

#[allow(dead_code)]
pub fn random_in_unit_disk() -> Vector3<f64> {
    let mut rng = thread_rng();

    let theta = rng.gen_range(0. ..TAU);
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
