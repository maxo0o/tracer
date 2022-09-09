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

#[allow(dead_code)]
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

pub fn random_to_sphere(radius: f64, distance_squared: f64) -> Vec3 {
    let r1: f64 = rand::thread_rng().gen();
    let r2: f64 = rand::thread_rng().gen();
    let z = 1.0 + r2 * ((1.0 - radius * radius / distance_squared).sqrt() - 1.0);

    let phi = 2.0 * PI * r1;
    let x = phi.cos() * (1.0 - z * z).sqrt();
    let y = phi.sin() * (1.0 - z * z).sqrt();

    Vec3::new(x, y, z)
}

pub fn cos_theta(w: &Vec3) -> f64 {
    w.z
}

pub fn cos_2_theta(w: &Vec3) -> f64 {
    w.z * w.z
}

pub fn abs_cos_theta(w: &Vec3) -> f64 {
    w.z.abs()
}

pub fn sin_2_theta(w: &Vec3) -> f64 {
    (0.0 as f64).max(1.0 - cos_2_theta(w))
}

pub fn sin_theta(w: &Vec3) -> f64 {
    sin_2_theta(w).sqrt()
}

pub fn tan_theta(w: &Vec3) -> f64 {
    sin_theta(w) / cos_theta(w)
}

pub fn tan_2_theta(w: &Vec3) -> f64 {
    sin_2_theta(w) / cos_2_theta(w)
}

pub fn cos_phi(w: &Vec3) -> f64 {
    let sin_theta = sin_theta(w);
    if sin_theta == 0.0 {
        1.0
    } else {
        (w.x / sin_theta).clamp(-1.0, 1.0)
    }
}

pub fn sin_phi(w: &Vec3) -> f64 {
    let sin_theta = sin_theta(w);
    if sin_theta == 0.0 {
        0.0
    } else {
        (w.y / sin_theta).clamp(-1.0, 1.0)
    }
}

pub fn cos_2_phi(w: &Vec3) -> f64 {
    cos_phi(w) * cos_phi(w)
}

pub fn sin_2_phi(w: &Vec3) -> f64 {
    sin_phi(w) * sin_phi(w)
}

pub fn cos_d_phi(wa: &Vec3, wb: &Vec3) -> f64 {
    ((wa.x * wb.x + wa.y * wb.y)
        / ((wa.x * wa.x + wa.y * wa.y) * (wb.x * wb.x + wb.y * wb.y)).sqrt())
    .clamp(-1.0, 1.0)
}

pub fn spherical_direction(sin_theta: f64, cos_theta: f64, phi: f64) -> Vec3 {
    Vec3::new(sin_theta * phi.cos(), sin_theta * phi.sin(), cos_theta)
}

pub fn same_hemisphere(w: &Vec3, wp: &Vec3) -> bool {
    w.z * wp.z > 0.0
}
