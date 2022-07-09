use crate::aabb::{surrounding_box, AxisAlignedBoundingBox};
use crate::hittable::{HitRecord, Hittable};
use crate::vector::Vec3;
use std::sync::Arc;

#[derive(Debug)]
pub struct BoundingVolumeHierarchy {
    pub bounding_box: AxisAlignedBoundingBox,
    pub left: Arc<Box<dyn Hittable>>,
    pub right: Arc<Box<dyn Hittable>>,
}

impl BoundingVolumeHierarchy {
    pub fn build(list: &mut [Arc<Box<dyn Hittable>>], depth: u32) -> BoundingVolumeHierarchy {
        let axis = (depth % 3) as usize;

        match list.len() {
            1 => {
                let left = Arc::clone(&list[0]);
                let right = Arc::clone(&list[0]);
                let bounding_box = surrounding_box(&left.bounding_box(), &right.bounding_box());

                return BoundingVolumeHierarchy {
                    bounding_box: bounding_box.unwrap(),
                    left,
                    right,
                };
            }
            2 => {
                if box_compare(Arc::clone(&list[0]), Arc::clone(&list[1]), axis) {
                    let left = Arc::clone(&list[0]);
                    let right = Arc::clone(&list[1]);
                    let bounding_box = surrounding_box(&left.bounding_box(), &right.bounding_box());

                    return BoundingVolumeHierarchy {
                        bounding_box: bounding_box.unwrap(),
                        left,
                        right,
                    };
                } else {
                    let left = Arc::clone(&list[1]);
                    let right = Arc::clone(&list[0]);
                    let bounding_box = surrounding_box(&right.bounding_box(), &left.bounding_box());

                    return BoundingVolumeHierarchy {
                        bounding_box: bounding_box.unwrap(),
                        left,
                        right,
                    };
                }
            }
            _ => {
                list.sort_by(|object_a, object_b| {
                    let box_a = object_a.bounding_box().unwrap();
                    let box_b = object_b.bounding_box().unwrap();

                    match axis {
                        0 => box_a.minimum.x.partial_cmp(&box_b.minimum.x).unwrap(),
                        1 => box_a.minimum.y.partial_cmp(&box_b.minimum.y).unwrap(),
                        2 => box_a.minimum.z.partial_cmp(&box_b.minimum.z).unwrap(),
                        _ => panic!("Invalid axis!"),
                    }
                });

                let mid = list.len() / 2;
                let left = BoundingVolumeHierarchy::build(&mut list[..mid], depth + 1);
                let right = BoundingVolumeHierarchy::build(&mut list[mid..], depth + 1);
                let bounding_box = surrounding_box(&left.bounding_box(), &right.bounding_box());

                return BoundingVolumeHierarchy {
                    bounding_box: bounding_box.unwrap(),
                    left: Arc::new(Box::new(left)),
                    right: Arc::new(Box::new(right)),
                };
            }
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

        let mut hit: Option<HitRecord> = None;
        if let Some(hit_left) = self.left.hit(ray, camera, t_min, t_max) {
            hit = Some(hit_left);
        }

        let mut left_hit_t = 0.0;
        if let Some(hit_left) = &hit {
            left_hit_t = hit_left.t;
        } else {
            left_hit_t = t_max;
        }
        if let Some(hit_right) = self.right.hit(ray, camera, t_min, left_hit_t) {
            hit = Some(hit_right);
        }

        hit
    }

    fn bounding_box(&self) -> Option<AxisAlignedBoundingBox> {
        Some(AxisAlignedBoundingBox::new(
            Vec3::copy(&self.bounding_box.minimum),
            Vec3::copy(&self.bounding_box.maximum),
        ))
    }
}

fn box_compare(
    hittable_a: Arc<Box<dyn Hittable>>,
    hittable_b: Arc<Box<dyn Hittable>>,
    axis: usize,
) -> bool {
    if let (Some(bound_box_a), Some(bound_box_b)) =
        (hittable_a.bounding_box(), hittable_b.bounding_box())
    {
        match axis {
            0 => return bound_box_a.minimum.x < bound_box_b.minimum.x,
            1 => return bound_box_a.minimum.y < bound_box_b.minimum.y,
            2 => return bound_box_a.minimum.z < bound_box_b.minimum.z,
            _ => panic!("Invalid value for axis"),
        }
    }

    panic!("Objects don't have bounding boxes?");
}
