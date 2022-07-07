use crate::{material::Material, camera::Camera};
use crate::ray::Ray;
use crate::vector::Vec3;
use crate::aabb::AxisAlignedBoundingBox;

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, camera: &Camera, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self) -> Option<AxisAlignedBoundingBox>;
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
            bounding_box = if first_box { temp_box } else { surrounding_box(&bounding_box, &temp_box) };
            first_box = false;
        }

        bounding_box
    }
}

fn surrounding_box(box_a: &Option<AxisAlignedBoundingBox>, box_b: &Option<AxisAlignedBoundingBox>) -> Option<AxisAlignedBoundingBox> {
    
    if let (Some(box_a), Some(box_b)) = (box_a, box_b) {
        let small = Vec3::new(
            box_a.minimum.x.min(box_b.minimum.x),
            box_a.minimum.y.min(box_b.minimum.y),
            box_a.minimum.z.min(box_b.minimum.z),
        );
    
        let big = Vec3::new(
            box_a.maxmimum.x.min(box_b.maxmimum.x),
            box_a.maxmimum.y.min(box_b.maxmimum.y),
            box_a.maxmimum.z.min(box_b.maxmimum.z),
        );

        return Some(AxisAlignedBoundingBox::new(small, big));
    }
    None
}
