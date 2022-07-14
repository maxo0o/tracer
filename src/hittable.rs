use crate::aabb::{surrounding_box, AxisAlignedBoundingBox};
use crate::ray::Ray;
use crate::vector::Vec3;
use crate::{camera::Camera, material::Material};

pub trait Hittable: Send + Sync + std::fmt::Debug {
    fn hit(&self, ray: &Ray, camera: &Camera, t_min: f64, t_max: f64) -> Option<HitRecord>;

    fn bounding_box(&self) -> Option<AxisAlignedBoundingBox> {
        None
    }
}

#[derive(Debug)]
pub struct HitRecord<'a> {
    pub p: Vec3,
    pub normal: Vec3,
    pub material: &'a dyn Material,
    pub t: f64,
    pub front_face: bool,
    pub u: f64,
    pub v: f64,
}

impl HitRecord<'_> {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = ray.direction.dot(&outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal.clone()
        } else {
            -outward_normal.clone()
        };
    }
}

#[derive(Debug)]
pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn hit_something(&self, ray: &Ray, t_min: f64, t_max: f64) -> bool {
        for object in &self.objects {
            if let Some(bounding_box) = object.bounding_box() {
                if bounding_box.hit(ray, t_min, t_max) {
                    return true;
                }
            }
        }

        false
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, camera: &Camera, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit_anything: Option<HitRecord> = None;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if let Some(hit_record) = object.hit(ray, camera, t_min, closest_so_far) {
                closest_so_far = hit_record.t;
                hit_anything = Some(hit_record);
            }
        }

        hit_anything
    }

    fn bounding_box(&self) -> Option<AxisAlignedBoundingBox> {
        if self.objects.len() == 0 {
            return None;
        }

        let mut bounding_box: Option<AxisAlignedBoundingBox> = None;

        let mut first_box = true;
        for object in &self.objects {
            let temp_box = object.bounding_box();
            bounding_box = if first_box {
                temp_box
            } else {
                surrounding_box(&bounding_box, &temp_box)
            };
            first_box = false;
        }

        bounding_box
    }
}
