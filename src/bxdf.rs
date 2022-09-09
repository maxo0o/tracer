use std::f64::consts::PI;

use crate::colour::Colour;
// use crate::fresnel::Fresnel;
// use crate::microfacet::Microfacet;
// use crate::utils::*;
use crate::vector::Vec3;

//pub enum BxDFType {
//    Reflection,
//    Transmission,
//    Diffuse,
//    Glossy,
//    Specular,
//    All,
//}

pub trait BxDF: std::fmt::Debug + Send + Sync {
    fn f(&self, wo: &Vec3, wi: &Vec3, n: &Vec3, colour: &Colour) -> Colour;
}

#[derive(Debug)]
pub struct MicrofacetReflection {
    metallic: f64,
    roughness: f64,
    reflectance: f64,
    include_diffuse: bool,
}
impl MicrofacetReflection {
    pub fn new(
        metallic: f64,
        roughness: f64,
        reflectance: f64,
        include_diffuse: bool,
    ) -> MicrofacetReflection {
        MicrofacetReflection {
            metallic,
            roughness,
            reflectance,
            include_diffuse,
        }
    }
}

impl BxDF for MicrofacetReflection {
    fn f(&self, wo: &Vec3, wi: &Vec3, n: &Vec3, colour: &Colour) -> Colour {
        let h = (wo + wi).unit();

        let no_v = n.dot(wo).clamp(0.0, 1.0);
        let no_l = n.dot(wi).clamp(0.0, 1.0);
        let no_h = n.dot(&h).clamp(0.0, 1.0);
        let vo_h = wo.dot(&h).clamp(0.0, 1.0);

        let r = 0.16 * self.reflectance * self.reflectance;
        let f0 = r;

        let f = fresnel_schlik(vo_h, f0);
        let d = d_ggx(no_h, self.roughness);
        let g = g_smith(no_v, no_l, self.roughness);

        let c = (f * d * g) / (4.0 * no_v.max(0.001) * no_l.max(0.001));
        let spec = Colour::new(c, c, c);

        let rho_d = (1.0 - self.metallic) * colour;
        let diff = (1.0 / PI) * rho_d;

        if self.include_diffuse {
            spec + diff
        } else {
            spec + 1.0 * colour
        }
    }
}

pub fn fresnel_schlik(cos_theta: f64, f: f64) -> f64 {
    f + (1.0 - f) * f64::powf(1.0 - cos_theta, 5.0)
}

pub fn d_ggx(no_h: f64, roughness: f64) -> f64 {
    let alpha = roughness * roughness;
    let alpha2 = alpha * alpha;
    let no_h2 = no_h * no_h;
    let b = no_h2 * (alpha2 - 1.0) + 1.0;
    alpha2 * (1.0 / PI) / (b * b)
}

pub fn g_smith(no_v: f64, no_l: f64, roughness: f64) -> f64 {
    g1_ggx_schlick(no_l, roughness) * g1_ggx_schlick(no_v, roughness)
}

pub fn g1_ggx_schlick(no_v: f64, roughness: f64) -> f64 {
    let alpha = roughness * roughness;
    let k = alpha / 2.0;
    no_v.max(0.001) / (no_v * (1.0 - k) + k)
}
//pub struct SpecularReflection {
//    pub scale: f64,
//    pub fresnel: Box<dyn Fresnel>,
//}
//impl BxDF for SpecularReflection {
//    fn bxdf_type() -> BxDFType {
//     BxDFType::Specular
//    }

//    fn f(&self, _wo: &Vec3, _wi: &Vec3) -> Colour {
//       Colour::new(0.0, 0.0, 0.0)
//    }
//
//    fn sample_f(&self, _wo: &Vec3, wi: &Vec3, u: (f64, f64)) -> Colour {
//        let f = (self.fresnel.evaulate(cos_theta(wi)) * self.scale) / abs_cos_theta(wi);
//        Colour::new(f, f, f)
//    }
//}

//pub struct MicrofacetReflection {
//    pub scale: f64,
//    pub fresnel: Box<dyn Fresnel>,
//    pub distribution: Box<dyn Microfacet>,
//}
//impl BxDF for MicrofacetReflection {
//    fn bxdf_type() -> BxDFType {
//        BxDFType::Reflection
//    }
//
//    fn f(&self, wo: &Vec3, wi: &Vec3) -> Colour {
//        let cos_theta_o = abs_cos_theta(wo);
//        let cos_theta_i = abs_cos_theta(wi);
//        let mut wh = wo + wi;
//
//        if cos_theta_o == 0.0 || cos_theta_i == 0.0 {
//            return Colour::new(0.0, 0.0, 0.0);
//        }
//        if wh.x == 0.0 && wh.y == 0.0 && wh.z == 0.0 {
//            return Colour::new(0.0, 0.0, 0.0);
//        }
//
//        wh = wh.unit();
//
//        if wh.dot(&Vec3::new(0.0, 0.0, 1.0)) < 0.0 {
//            wh = -wh;
//        }
//
//        let f = self.fresnel.evaulate(wi.dot(&wh));
//        let radiance =
//            self.scale * self.distribution.distribution(&wh) * self.distribution.g(wo, wi) * f
//                / (4.0 * cos_theta_i * cos_theta_o);
//        let pdf = self.distribution.pdf(wo, &wh) / (4.0 * wo.dot(&wh));
//        Colour::new(radiance, radiance, radiance)
//    }
//
//    fn sample_f(&self, wo: &Vec3, _wi: &Vec3, u: (f64, f64)) -> Colour {
//        if wo.z == 0.0 {
//            return Colour::new(0.0, 0.0, 0.0);
//        }
//
//        let wh = self.distribution.sample_wh(wo, u);
//        let wi = reflect(wo, &wh);
//
//        //if !same_hemisphere(wo, &wi) {
//        //  return Colour::new(0.0, 0.0, 0.0);
//        //}
//
//        self.f(wo, &wi)
//    }
//}
