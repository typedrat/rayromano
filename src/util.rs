use na::{Unit, UnitVector3, Vector3};
use palette::{rgb::Rgb, Hsl, IntoColor, Srgb};
use rand::Rng;
use std::f64::consts::TAU;

pub fn random_unit_vector() -> UnitVector3<f64> {
    let mut rng = rand::thread_rng();

    let u: f64 = 2. * rng.gen::<f64>() - 1.;
    let phi: f64 = TAU * rng.gen::<f64>();

    Unit::new_normalize(Vector3::new(
        phi.cos() * (1. - u.powf(2.)).powf(0.5),
        phi.sin() * (1. - u.powf(2.)).powf(0.5),
        u,
    ))
}

#[allow(dead_code)]
pub fn random_in_unit_disk() -> Vector3<f64> {
    let mut rng = rand::thread_rng();

    let theta = rng.gen_range(0. .. TAU);
    let r = rng.gen_range(0f64 ..= 1f64).sqrt();

    Vector3::new(r * theta.cos(), r * theta.sin(), 0.)
}

pub fn reflect_vector(v: &Vector3<f64>, n: &Vector3<f64>) -> Vector3<f64> {
    v - 2. * v.dot(n) * n
}

pub fn color(r: f64, g: f64, b: f64) -> Vector3<f64> {
    Vector3::new(r, g, b)
}

pub fn random_color() -> Vector3<f64> {
    let mut rng = rand::thread_rng();
    let hue = rng.gen_range(0.0..360.0);
    let saturation = rng.gen_range(0.5..=1.0);
    let lightness = rng.gen_range(0.2..=0.8);

    let hsl: Hsl<Srgb, f64> = Hsl::new(hue, saturation, lightness);
    let Rgb {
        red, green, blue, ..
    } = hsl.into_color();

    color(red, green, blue)
}
