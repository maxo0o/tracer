use std::f64::consts::PI;

use crate::colour::Colour;
use crate::hittable::HitRecord;
use crate::onb::OrthonormalBasis;
use crate::ray::Ray;
use crate::texture::Texture;
use crate::utils::random_cosine_direction;
use crate::utils::{random_in_unit_sphere, random_in_unit_vector, reflect, refract};
use crate::vector::Vec3;

use rand::Rng;

pub trait Material: Send + Sync + std::fmt::Debug {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> (Ray, Colour, bool) {
        (
            Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0)),
            Colour::new(0.0, 0.0, 0.0),
            false,
        )
    }

    fn emitted(&self, u: f64, v: f64, p: &Vec3) -> Colour {
        Colour::new(0.0, 0.0, 0.0)
    }

    fn scattering_pdf(&self, ray_in: &Ray, hit_record: &HitRecord, scattered: &Ray) -> f64 {
        1.0
    }

    fn use_pdfs(&self) -> bool {
        false
    }
}

#[derive(Debug)]
pub struct UnitMaterial {}
impl Material for UnitMaterial {}

#[derive(Debug)]
pub struct Lambertian {
    pub albedo: Box<dyn Texture + Send + Sync>,
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, hit_record: &HitRecord) -> (Ray, Colour, bool) {
        let onb = OrthonormalBasis::build_from_w(&hit_record.normal);
        //let mut scatter_direction = &hit_record.normal + random_in_unit_vector();
        let mut scatter_direction = onb.local_vec(&random_cosine_direction());

        if scatter_direction.near_zero() {
            scatter_direction = Vec3::copy(&hit_record.normal);
        }

        let scattered = Ray::new(Vec3::copy(&hit_record.p), scatter_direction);

        (
            scattered,
            Colour::copy(&self.albedo.value(hit_record.u, hit_record.v, &hit_record.p)),
            true,
        )
    }

    fn scattering_pdf(&self, ray_in: &Ray, hit_record: &HitRecord, scattered: &Ray) -> f64 {
        let cosine = hit_record.normal.dot(&scattered.direction.unit());

        return if cosine < 0.0 { 0.0 } else { cosine / PI };
    }

    fn use_pdfs(&self) -> bool {
        true
    }
}

#[derive(Debug)]
pub struct Metal {
    pub albedo: Colour,
    pub f: f64,
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> (Ray, Colour, bool) {
        let reflected = reflect(&ray_in.direction.unit(), &hit_record.normal);
        let scattered_ray = Ray::new(
            Vec3::copy(&hit_record.p),
            reflected + self.f.clamp(0.0, 1.0) * &random_in_unit_sphere(),
        );
        let scattered = scattered_ray.direction.dot(&hit_record.normal) > 0.0;
        (scattered_ray, Colour::copy(&self.albedo), scattered)
    }
}

#[derive(Debug)]
pub struct Dialectric {
    pub index_of_refraction: f64,
}

impl Material for Dialectric {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> (Ray, Colour, bool) {
        let attenuation = Colour::new(1.0, 1.0, 1.0);
        let refraction_ratio = if hit_record.front_face {
            1.0 / self.index_of_refraction
        } else {
            self.index_of_refraction
        };

        let unit_direction = ray_in.direction.unit();
        let cos_theta = -unit_direction.dot(&hit_record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let direction = if cannot_refract
            || reflectance(cos_theta, refraction_ratio) > rand::thread_rng().gen_range(0.0..1.0)
        {
            reflect(&unit_direction, &hit_record.normal)
        } else {
            refract(&unit_direction, &hit_record.normal, refraction_ratio)
        };

        (
            Ray::new(Vec3::copy(&hit_record.p), direction),
            attenuation,
            true,
        )
    }
}

#[derive(Debug)]
pub struct Light {
    pub intensity: f64,
    pub albedo: Box<dyn Texture + Send + Sync>,
}

impl Material for Light {
    fn scatter(&self, ray_in: &Ray, _hit_record: &HitRecord) -> (Ray, Colour, bool) {
        let ray = Ray::new(Vec3::copy(&ray_in.origin), Vec3::copy(&ray_in.direction));
        (ray, Colour::new(0.0, 0.0, 0.0), false)
    }

    fn emitted(&self, u: f64, v: f64, p: &Vec3) -> Colour {
        self.intensity * self.albedo.value(u, v, p)
    }
}

#[derive(Debug)]
pub struct Isotropic {
    pub albedo: Box<dyn Texture + Send + Sync>,
}

impl Material for Isotropic {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> (Ray, Colour, bool) {
        let ray = Ray::new(Vec3::copy(&hit_record.p), random_in_unit_sphere());
        (
            ray,
            self.albedo.value(hit_record.u, hit_record.v, &hit_record.p),
            true,
        )
    }
}

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}
