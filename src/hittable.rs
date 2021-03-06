use crate::material::Material;
use crate::ray::Ray;
use crate::vector::Vec3;
use std::sync::{Arc, Mutex};

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, p_0: u32, p_1: u32, zbuffer: Arc<Mutex<Vec<Vec<f64>>>>) -> Option<HitRecord>;
}

pub struct HitRecord<'a> {
    pub p: Vec3,
    pub normal: Vec3,
    pub material: &'a dyn Material,
    pub t: f64,
    pub front_face: bool,
}

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: Vec::new(),
        }
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, p_0: u32, p_1: u32, zbuffer: Arc<Mutex<Vec<Vec<f64>>>>) -> Option<HitRecord> {
        let mut hit_anything: Option<HitRecord> = None;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if let Some(hit_record) = object.hit(ray, t_min, closest_so_far, p_0, p_1, Arc::clone(&zbuffer)) {
                closest_so_far = hit_record.t;
                hit_anything = Some(hit_record);
            }
        }

        hit_anything
    }
}
