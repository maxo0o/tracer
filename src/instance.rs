use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vector::Vec3;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct Translate {
    object: Box<dyn Hittable>,
    offset: Vec3,
}

impl Translate {
    pub fn new(object: Box<dyn Hittable>, offset: Vec3) -> Translate {
        Translate { object, offset }
    }
}

impl Hittable for Translate {
    fn hit(
        &self,
        ray: &crate::ray::Ray,
        camera: &crate::camera::Camera,
        t_min: f64,
        t_max: f64,
        pixel: Option<(usize, usize)>,
        zbuffer: Arc<Mutex<Vec<Vec<f64>>>>,
    ) -> Option<HitRecord> {
        let ray_moved = Ray::new(ray.origin - self.offset, ray.direction);

        if let Some(hit) = self.object.hit(
            &ray_moved,
            camera,
            t_min,
            t_max,
            pixel,
            Arc::clone(&zbuffer),
        ) {
            let mut hit_record = HitRecord {
                p: hit.p + self.offset,
                normal: hit.normal.clone(),
                material: hit.material,
                t: hit.t,
                front_face: hit.front_face,
                u: hit.u,
                v: hit.v,
            };
            hit_record.set_face_normal(&ray_moved, &hit.normal);

            return Some(hit_record);
        }

        None
    }
}

#[derive(Debug)]
pub struct RotateY {
    object: Box<dyn Hittable>,
    theta: f64,
}

impl RotateY {
    pub fn new(object: Box<dyn Hittable>, theta: f64) -> RotateY {
        RotateY {
            object,
            theta: theta.to_radians(),
        }
    }
}

impl Hittable for RotateY {
    fn hit(
        &self,
        ray: &Ray,
        camera: &crate::camera::Camera,
        t_min: f64,
        t_max: f64,
        pixel: Option<(usize, usize)>,
        zbuffer: Arc<Mutex<Vec<Vec<f64>>>>,
    ) -> Option<HitRecord> {
        let mut origin = ray.origin.clone();
        let mut direction = ray.direction.clone();

        origin.x = self.theta.cos() * ray.origin.x - self.theta.sin() * ray.origin.z;
        origin.z = self.theta.sin() * ray.origin.x + self.theta.cos() * ray.origin.z;

        direction.x = self.theta.cos() * ray.direction.x - self.theta.sin() * ray.direction.z;
        direction.z = self.theta.sin() * ray.direction.x + self.theta.cos() * ray.direction.z;

        let ray_rotated = Ray::new(origin, direction);

        if let Some(hit) = self.object.hit(
            &ray_rotated,
            camera,
            t_min,
            t_max,
            pixel,
            Arc::clone(&zbuffer),
        ) {
            let mut p = hit.p;

            p.x = self.theta.cos() * hit.p.x + self.theta.sin() * hit.p.z;
            p.z = -self.theta.sin() * hit.p.x + self.theta.cos() * hit.p.z;

            let mut normal = hit.normal;

            normal.x = self.theta.cos() * hit.normal.x + self.theta.sin() * hit.normal.z;
            normal.z = -self.theta.sin() * hit.normal.x + self.theta.cos() * hit.normal.z;

            let mut hit_record = HitRecord {
                p,
                normal,
                material: hit.material,
                t: hit.t,
                front_face: hit.front_face,
                u: hit.u,
                v: hit.v,
            };
            hit_record.set_face_normal(&ray_rotated, &normal);

            return Some(hit_record);
        }

        None
    }
}
