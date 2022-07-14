use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::vector::Vec3;

#[derive(Debug)]
pub struct Volume<T: Material> {
    boundary: Box<dyn Hittable>,
    pub material: T,
    neg_inv_density: f64,
}

impl<T: Material> Volume<T> {
    pub fn new(boundary: Box<dyn Hittable>, d: f64, material: T) -> Volume<T> {
        Volume {
            boundary,
            neg_inv_density: -1.0 / d,
            material,
        }
    }
}

impl<T: Material> Hittable for Volume<T> {
    fn hit(
        &self,
        ray: &crate::ray::Ray,
        camera: &crate::camera::Camera,
        t_min: f64,
        t_max: f64,
    ) -> Option<crate::hittable::HitRecord> {
        if let Some(hit1) = &mut self
            .boundary
            .hit(ray, camera, -f64::INFINITY, f64::INFINITY)
        {
            if let Some(hit2) = &mut self
                .boundary
                .hit(ray, camera, hit1.t + 0.0001, f64::INFINITY)
            {
                if hit1.t < t_min {
                    hit1.t = t_min;
                }
                if hit2.t > t_max {
                    hit2.t = t_max;
                }

                if hit1.t >= hit2.t {
                    return None;
                }

                if hit1.t < 0.0 {
                    hit1.t = 0.0;
                }

                let ray_length = ray.direction.length();
                let distance_inside_boundary = (hit2.t - hit1.t) * ray_length;
                let hit_distance = self.neg_inv_density * rand::random::<f64>().log10();

                if hit_distance > distance_inside_boundary {
                    return None;
                }

                let t = hit1.t + hit_distance / ray_length;
                let p = ray.at(t);

                return Some(HitRecord {
                    p,
                    t,
                    normal: Vec3::new(1.0, 0.0, 0.0),
                    front_face: true,
                    material: &self.material,
                    u: 0.0,
                    v: 0.0,
                });
            } else {
                return None;
            }
        }
        None
    }
}
