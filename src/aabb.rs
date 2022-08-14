use crate::ray::Ray;
use crate::vector::Vec3;

#[derive(Debug)]
pub struct AxisAlignedBoundingBox {
    pub minimum: Vec3,
    pub maximum: Vec3,
    pub bounds: [Vec3; 2],
}

#[allow(dead_code)]
impl AxisAlignedBoundingBox {
    pub fn new(point_a: Vec3, point_b: Vec3) -> AxisAlignedBoundingBox {
        AxisAlignedBoundingBox {
            minimum: point_a,
            maximum: point_b,
            bounds: [point_a, point_b],
        }
    }

    pub fn hit3(&self, ray: &Ray, t_min: f64, t_max: f64) -> bool {
        let mut tmin = (self.bounds[ray.sign[0]].x - ray.origin.x) * ray.inverse_direction.x;
        let mut tmax = (self.bounds[1 - ray.sign[0]].x - ray.origin.x) * ray.inverse_direction.x;
        let tymin = (self.bounds[ray.sign[1]].y - ray.origin.y) * ray.inverse_direction.y;
        let tymax = (self.bounds[1 - ray.sign[1]].y - ray.origin.y) * ray.inverse_direction.y;

        if tmin > tymax || tymin > tmax {
            return false;
        }

        if tymin > tmin {
            tmin = tymin;
        }
        if tymax < tmax {
            tmax = tymax;
        }

        let tzmin = (self.bounds[ray.sign[2]].z - ray.origin.z) * ray.inverse_direction.z;
        let tzmax = (self.bounds[1 - ray.sign[2]].z - ray.origin.z) * ray.inverse_direction.z;

        if tmin > tzmax || tzmin > tmax {
            return false;
        }

        true
    }

    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> bool {
        let mut tmin = (self.minimum.x - ray.origin.x) / ray.direction.x;
        let mut tmax = (self.maximum.x - ray.origin.x) / ray.direction.x;

        use std::mem;
        if tmin > tmax {
            mem::swap(&mut tmin, &mut tmax);
        }

        let mut tymin = (self.minimum.y - ray.origin.y) / ray.direction.y;
        let mut tymax = (self.maximum.y - ray.origin.y) / ray.direction.y;

        if tymin > tymax {
            mem::swap(&mut tymin, &mut tymax);
        }

        if (tmin > tymax) || (tymin > tmax) {
            return false;
        }

        if tymin > tmin {
            tmin = tymin;
        }

        if tymax < tmax {
            tmax = tymax;
        }

        let mut tzmin = (self.minimum.z - ray.origin.z) / ray.direction.z;
        let mut tzmax = (self.maximum.z - ray.origin.z) / ray.direction.z;

        if tzmin > tzmax {
            mem::swap(&mut tzmin, &mut tzmax);
        }

        if (tmin > tzmax) || (tzmin > tmax) {
            return false;
        }

        if tzmin > tmin {
            tmin = tzmin;
        }

        if tzmax < tmax {
            tmax = tzmax;
        }

        return true;
    }

    pub fn hit2(&self, ray: &Ray, t_min: f64, t_max: f64) -> bool {
        let t0 = ((self.minimum.x - ray.origin.x) / ray.direction.x)
            .min((self.maximum.x - ray.origin.x) / ray.direction.x);
        let t1 = ((self.minimum.x - ray.origin.x) / ray.direction.x)
            .max((self.maximum.x - ray.origin.x) / ray.direction.x);
        let tmin = t0.max(t_min);
        let tmax = t1.min(t_max);
        if tmax <= tmin {
            return false;
        }

        let t0 = ((self.minimum.y - ray.origin.y) / ray.direction.y)
            .min((self.maximum.y - ray.origin.y) / ray.direction.y);
        let t1 = ((self.minimum.y - ray.origin.y) / ray.direction.y)
            .max((self.maximum.y - ray.origin.y) / ray.direction.y);
        let tmin = t0.max(t_min);
        let tmax = t1.min(t_max);
        if tmax <= tmin {
            return false;
        }

        let t0 = ((self.minimum.z - ray.origin.z) / ray.direction.z)
            .min((self.maximum.z - ray.origin.z) / ray.direction.z);
        let t1 = ((self.minimum.z - ray.origin.z) / ray.direction.z)
            .max((self.maximum.z - ray.origin.z) / ray.direction.z);
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
