use crate::hittable::{HitRecord, Hittable, HittableList};
use std::ops::Bound;

use crate::aabb::AxisAlignedBoundingBox;

pub struct BoundingVolumeHierarchy {
    pub bounding_box: AxisAlignedBoundingBox,
    pub left: Box<dyn Hittable>,
    pub right: Box<dyn Hittable>,
}

impl BoundingVolumeHierarchy {
    pub fn build(&mut self, list: HittableList, start: usize, end: usize, depth: u32) {
        let object_span = end - start;
        let axis = (depth % 3) as usize;

        match object_span {
            1 => {
                self.left = list.objects[start];
                self.right = list.objects[start];
            }
            2 => {
                if box_compare(list.objects[start], list.objects[start + 1], axis) {
                    self.left =list.objects[start];
                    self.right = list.objects[start + 1];
                } else {
                    self.left = list.objects[start + 1];
                    self.right = list.objects[start];
                }
            }
            _ => {}
        }
    }
}

impl Hittable for BoundingVolumeHierarchy {
    fn hit(
        &self,
        ray: &crate::ray::Ray,
        camera: &crate::camera::Camera,
        t_min: f64,
        t_max: f64,
    ) -> Option<HitRecord> {
        if self.bounding_box.hit(ray, t_min, t_max) == false {
            return None;
        }

        if let Some(hit_left) = self.left.hit(ray, camera, t_min, t_max) {
            return Some(hit_left);
        }

        if let Some(hit_right) = self.right.hit(ray, camera, t_min, t_max) {
            return Some(hit_right);
        }

        None
    }

    fn bounding_box(&self) -> Option<AxisAlignedBoundingBox> {
        self.bounding_box()
    }
}

fn box_compare(hittable_a: Box<dyn Hittable>, hittable_b: Box<dyn Hittable>, axis: usize) -> bool {
    if let (Some(bound_box_a), Some(bound_box_b)) =
        (hittable_a.bounding_box(), hittable_b.bounding_box())
    {
        match axis {
            0 => return bound_box_a.minimum.x < bound_box_b.minimum.x,
            1 => return bound_box_a.minimum.y < bound_box_b.minimum.y,
            2 => return bound_box_a.minimum.z < bound_box_b.minimum.z,
        }
    }

    panic!("Objects don't have bounding boxes?");
}
