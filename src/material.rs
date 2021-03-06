use crate::colour::Colour;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::utils::{random_in_unit_sphere, random_in_unit_vector, reflect, refract};
use crate::vector::Vec3;

use rand::Rng;

pub trait Material: Send + Sync {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> (Ray, Colour, bool, bool);
}

pub struct Lambertian {
    pub albedo: Colour,
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, hit_record: &HitRecord) -> (Ray, Colour, bool, bool) {
        let mut scatter_direction = &hit_record.normal + random_in_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = Vec3::copy(&hit_record.normal);
        }

        let scattered = Ray::new(Vec3::copy(&hit_record.p), scatter_direction);
        let is_light = false;
        (scattered, Colour::copy(&self.albedo), true, is_light)
    }
}

pub struct Metal {
    pub albedo: Colour,
    pub f: f64,
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> (Ray, Colour, bool, bool) {
        let reflected = reflect(&ray_in.direction.unit(), &hit_record.normal);
        let scattered_ray = Ray::new(
            Vec3::copy(&hit_record.p),
            reflected + self.f.clamp(0.0, 1.0) * &random_in_unit_sphere(),
        );
        let scattered = scattered_ray.direction.dot(&hit_record.normal) > 0.0;
        let is_light = false;
        (scattered_ray, Colour::copy(&self.albedo), scattered, is_light)
    }
}

pub struct Dialectric {
    pub index_of_refraction: f64,
}

impl Material for Dialectric {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> (Ray, Colour, bool, bool) {
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

        let is_light = false;
        (
            Ray::new(Vec3::copy(&hit_record.p), direction),
            attenuation,
            true,
            is_light,
        )
    }
}

pub struct Light {
    pub intensity: f64,
    pub colour: Colour,
}

impl Material for Light {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> (Ray, Colour, bool, bool) {
        let ray = Ray::new(Vec3::copy(&ray_in.origin), Vec3::copy(&ray_in.direction));
        let is_light = true;
        (ray, self.intensity * Colour::copy(&self.colour), false, is_light)
    }
}

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}
