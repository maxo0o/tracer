use std::f64::consts::PI;

use crate::utils::*;
use crate::vector::Vec3;

pub trait Microfacet {
    fn distribution(&self, wh: &Vec3) -> f64;

    fn lambda(&self, w: &Vec3) -> f64;

    fn g1(&self, w: &Vec3) -> f64;

    fn g(&self, wo: &Vec3, wi: &Vec3) -> f64;

    fn sample_wh(&self, wo: &Vec3, u: (f64, f64)) -> Vec3;

    fn pdf(&self, wo: &Vec3, wh: &Vec3) -> f64;
}

pub struct BeckmannDistribution {
    alpha_x: f64,
    alpha_y: f64,
}
impl BeckmannDistribution {
    pub fn new(alpha_x: f64, alpha_y: f64) -> BeckmannDistribution {
        BeckmannDistribution { alpha_x, alpha_y }
    }

    pub fn roughness(value: f64) -> f64 {
        let roughness = value.max(0.0001);
        let x = roughness.ln();
        1.62142
            + 0.819955 * x
            + 0.1734 * x * x
            + 0.0171201 * x * x * x
            + 0.000640711 * x * x * x * x
    }
}
impl Microfacet for BeckmannDistribution {
    fn distribution(&self, wh: &Vec3) -> f64 {
        let tan_2_theta = tan_2_theta(wh);
        if tan_2_theta.is_infinite() {
            return 0.0;
        }

        let cos_4_theta = cos_2_theta(wh) * cos_2_theta(wh);
        f64::exp(
            -tan_2_theta
                * (cos_2_phi(wh) / (self.alpha_x * self.alpha_x)
                    + sin_2_phi(wh) / (self.alpha_y * self.alpha_y)),
        ) / (PI * self.alpha_x * self.alpha_y * cos_4_theta)
    }

    fn lambda(&self, w: &Vec3) -> f64 {
        let abs_tan_theta = tan_theta(w).abs();
        if abs_tan_theta.is_infinite() {
            return 0.0;
        }

        let alpha = (cos_2_phi(w) * self.alpha_x * self.alpha_x
            + sin_2_phi(w) * self.alpha_y * self.alpha_y)
            .sqrt();
        let a = 1.0 / (alpha * abs_tan_theta);
        if a >= 1.6 {
            return 0.0;
        }

        (1.0 - 1.259 * a + 0.396 * a * a) / (3.535 * a + 2.181 * a * a)
    }

    fn g1(&self, w: &Vec3) -> f64 {
        1.0 / (1.0 + self.lambda(w))
    }

    fn g(&self, wo: &Vec3, wi: &Vec3) -> f64 {
        1.0 / (1.0 + self.lambda(wo) + self.lambda(wi))
    }

    fn sample_wh(&self, wo: &Vec3, u: (f64, f64)) -> Vec3 {
        let mut tan_2_theta = 0.0;
        let mut phi = 0.0;

        if self.alpha_x == self.alpha_y {
            let mut log_sample = f64::ln(1.0 - u.0);
            if log_sample.is_infinite() {
                log_sample = 0.0;
            }
            tan_2_theta = -self.alpha_x * self.alpha_x * log_sample;
            phi = u.1 * 2.0 * PI;
        } else {
            let log_sample = f64::ln(u.0);
            phi = f64::atan(self.alpha_y / self.alpha_x * f64::tan(2.0 * PI * u.1 + 0.5 * PI));

            if u.1 > 0.5 {
                phi += PI;
            }

            let sin_phi = phi.sin();
            let cos_phi = phi.cos();
            let alpha_x2 = self.alpha_x * self.alpha_x;
            let alpha_y2 = self.alpha_y * self.alpha_y;

            tan_2_theta =
                -log_sample / (cos_phi * cos_phi / alpha_x2 + sin_phi * sin_phi / alpha_y2);
        }

        let cos_theta = 1.0 / (1.0 + tan_2_theta).sqrt();
        let sin_theta = f64::sqrt(f64::max(0.0, 1.0 - cos_theta * cos_theta));

        let mut wh = spherical_direction(sin_theta, cos_theta, phi);
        if !same_hemisphere(wo, &wh) {
            wh = -wh;
        }

        wh
    }

    fn pdf(&self, wo: &Vec3, wh: &Vec3) -> f64 {
        self.distribution(wh) * abs_cos_theta(wh)
    }
}
