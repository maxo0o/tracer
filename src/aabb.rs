use crate::ray::Ray;
use crate::vector::Vec3;

pub struct AxisAlignedBoundingBox {
    pub minimum: Vec3,
    pub maxmimum: Vec3,
}

impl AxisAlignedBoundingBox {
    pub fn new(point_a: Vec3, point_b: Vec3) -> AxisAlignedBoundingBox {
        AxisAlignedBoundingBox {
            minimum: point_a,
            maxmimum: point_b,
        }
    }

    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> bool {
        let t0 = (self.minimum.x - ray.origin.x / ray.direction.x)
            .min(self.maxmimum.x - ray.origin.x / ray.direction.x);
        let t1 = (self.minimum.x - ray.origin.x / ray.direction.x)
            .max(self.maxmimum.x - ray.origin.x / ray.direction.x);
        let tmin = t0.max(t_min);
        let tmax = t1.min(t_max);
        if tmax <= tmin {
            return false;
        }

        let t0 = (self.minimum.y - ray.origin.y / ray.direction.y)
            .min(self.maxmimum.y - ray.origin.y / ray.direction.y);
        let t1 = (self.minimum.y - ray.origin.y / ray.direction.y)
            .max(self.maxmimum.y - ray.origin.y / ray.direction.y);
        let tmin = t0.max(t_min);
        let tmax = t1.min(t_max);
        if tmax <= tmin {
            return false;
        }

        let t0 = (self.minimum.z - ray.origin.z / ray.direction.z)
            .min(self.maxmimum.z - ray.origin.z / ray.direction.z);
        let t1 = (self.minimum.z - ray.origin.z / ray.direction.z)
            .max(self.maxmimum.z - ray.origin.z / ray.direction.z);
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
            box_a.maxmimum.x.min(box_b.maxmimum.x),
            box_a.maxmimum.y.min(box_b.maxmimum.y),
            box_a.maxmimum.z.min(box_b.maxmimum.z),
        );

        return Some(AxisAlignedBoundingBox::new(small, big));
    }
    None
}