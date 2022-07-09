use crate::ray::Ray;
use crate::vector::Vec3;

#[derive(Debug)]
pub struct AxisAlignedBoundingBox {
    pub minimum: Vec3,
    pub maximum: Vec3,
}

impl AxisAlignedBoundingBox {
    pub fn new(point_a: Vec3, point_b: Vec3) -> AxisAlignedBoundingBox {
        AxisAlignedBoundingBox {
            minimum: point_a,
            maximum: point_b,
        }
    }

    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> bool {
        let t0 = (self.minimum.x - ray.origin.x / ray.direction.x)
            .min(self.maximum.x - ray.origin.x / ray.direction.x);
        let t1 = (self.minimum.x - ray.origin.x / ray.direction.x)
            .max(self.maximum.x - ray.origin.x / ray.direction.x);
        let tmin = t0.max(t_min);
        let tmax = t1.min(t_max);
        if tmax <= tmin {
            return false;
        }

        let t0 = (self.minimum.y - ray.origin.y / ray.direction.y)
            .min(self.maximum.y - ray.origin.y / ray.direction.y);
        let t1 = (self.minimum.y - ray.origin.y / ray.direction.y)
            .max(self.maximum.y - ray.origin.y / ray.direction.y);
        let tmin = t0.max(t_min);
        let tmax = t1.min(t_max);
        if tmax <= tmin {
            return false;
        }

        let t0 = (self.minimum.z - ray.origin.z / ray.direction.z)
            .min(self.maximum.z - ray.origin.z / ray.direction.z);
        let t1 = (self.minimum.z - ray.origin.z / ray.direction.z)
            .max(self.maximum.z - ray.origin.z / ray.direction.z);
        let tmin = t0.max(t_min);
        let tmax = t1.min(t_max);
        if tmax <= tmin {
            return false;
        }

        true
    }
}

pub fn surrounding_box(
    box_a: &Option<AxisAlignedBoundingBox>,
    box_b: &Option<AxisAlignedBoundingBox>,
) -> Option<AxisAlignedBoundingBox> {
    if let (Some(box_a), Some(box_b)) = (box_a, box_b) {
        let small = Vec3::new(
            box_a.minimum.x.min(box_b.minimum.x),
            box_a.minimum.y.min(box_b.minimum.y),
            box_a.minimum.z.min(box_b.minimum.z),
        );

        let big = Vec3::new(
            box_a.maximum.x.min(box_b.maximum.x),
            box_a.maximum.y.min(box_b.maximum.y),
            box_a.maximum.z.min(box_b.maximum.z),
        );

        return Some(AxisAlignedBoundingBox::new(small, big));
    }
    None
}
