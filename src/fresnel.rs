use crate::colour::Colour;

pub trait Fresnel {
    fn evaulate(&self, cos_i: f64) -> f64;
}

pub struct FresnelConductor {
    index_of_refraction_i: f64,
    index_of_refraction_t: f64,
    absorption_coefficient: f64,
}
impl FresnelConductor {
    pub fn new(eta_i: f64, eta_t: f64, k: f64) -> FresnelConductor {
        FresnelConductor {
            index_of_refraction_i: eta_i,
            index_of_refraction_t: eta_t,
            absorption_coefficient: k,
        }
    }
}
impl Fresnel for FresnelConductor {
    fn evaulate(&self, cos_i: f64) -> f64 {
        let cos_theta_i = cos_i.clamp(-1.0, 1.0);
        let eta = self.index_of_refraction_t / self.index_of_refraction_i;
        let eta_k = self.absorption_coefficient / self.index_of_refraction_i;

        let cos_theta_i_2 = cos_theta_i * cos_theta_i;
        let sin_theta_i_2 = 1.0 - cos_theta_i_2;
        let eta_2 = eta * eta;
        let eta_k_2 = eta_k * eta_k;

        let t0 = eta_2 - eta_k_2 - sin_theta_i_2;
        let a2_plus_b2 = (t0 * t0 + 4.0 * eta_2 * eta_k_2).sqrt();
        let t1 = a2_plus_b2 + cos_theta_i_2;
        let a = (0.5 * (a2_plus_b2 + t0)).sqrt();
        let t2 = 2.0 * cos_theta_i * a;
        let rs = (t1 - t2) / (t1 + t2);

        let t3 = cos_theta_i_2 * a2_plus_b2 + sin_theta_i_2 * sin_theta_i_2;
        let t4 = t2 * sin_theta_i_2;
        let rp = rs * (t3 - t4) / (t3 + t4);

        0.5 * (rp + rs)
    }
}

struct FresnelDielectric {
    index_of_refraction_i: f64,
    index_of_refraction_t: f64,
}
impl Fresnel for FresnelDielectric {
    fn evaulate(&self, cos_i: f64) -> f64 {
        1.0
    }
}

struct FresnelNoOp {}
impl Fresnel for FresnelNoOp {
    fn evaulate(&self, cos_i: f64) -> f64 {
        1.0
    }
}
