use std::f64::consts::PI;

use crate::vector::Vec3;
use rand::Rng;

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    v - 2.0 * v.dot(n) * n
}

pub fn refract(uv: &Vec3, n: &Vec3, etail_over_etat: f64) -> Vec3 {
    let cos_theta = -uv.dot(&n).min(1.0);
    let r_out_perp = etail_over_etat * &(uv + cos_theta * n);
    let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * n;
    r_out_perp + r_out_parallel
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3::new(
            rand::thread_rng().gen_range(-1.0..1.0),
            rand::thread_rng().gen_range(-1.0..1.0),
            rand::thread_rng().gen_range(-1.0..1.0),
        );

        if p.length_squared() >= 1.0 {
            continue;
        }

        return p;
    }
}

pub fn distance(a: &Vec3, b: &Vec3) -> f64 {
    ((b.x - a.x).powf(2.0) + (b.y - a.y).powf(2.0) + (b.z - a.z).powf(2.0)).sqrt()
}

pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = Vec3::new(
            rand::thread_rng().gen_range(-1.0..1.0),
            rand::thread_rng().gen_range(-1.0..1.0),
            0.0,
        );

        if p.length_squared() >= 1.0 {
            continue;
        }

        return p;
    }
}

pub fn random_in_unit_vector() -> Vec3 {
    random_in_unit_sphere().unit()
}

pub fn _random_in_hemisphere(normal: &Vec3) -> Vec3 {
    let in_unit_sphere = random_in_unit_sphere();
    if in_unit_sphere.dot(normal) > 0.0 {
        return in_unit_sphere;
    }

    -in_unit_sphere
}

pub fn random_cosine_direction() -> Vec3 {
    let r1: f64 = rand::thread_rng().gen();
    let r2: f64 = rand::thread_rng().gen();
    let z = (1.0 - r2).sqrt();

    let phi = 2.0 * PI * r1;
    let x = phi.cos() * r2.sqrt();
    let y = phi.sin() * r2.sqrt();

    Vec3::new(x, y, z)
}
