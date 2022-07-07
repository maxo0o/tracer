use crate::vector::Vec3;
use crate::ray::Ray;

pub struct AxisAlignedBoundingBox {
    pub minimum: Vec3,
    pub maxmimum: Vec3,
}

impl AxisAlignedBoundingBox {
    pub fn new(point_a: Vec3, point_b: Vec3) -> AxisAlignedBoundingBox {
        AxisAlignedBoundingBox { minimum: point_a, maxmimum: point_b }
    }

    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> bool {
        let t0 = (self.minimum.x - ray.origin.x / ray.direction.x).min(self.maxmimum.x - ray.origin.x / ray.direction.x);
        let t1 = (self.minimum.x - ray.origin.x / ray.direction.x).max(self.maxmimum.x - ray.origin.x / ray.direction.x);
        let tmin = t0.max(t_min);
        let tmax = t1.min(t_max);
        if tmax <= tmin {
            return false;
        }

        let t0 = (self.minimum.y - ray.origin.y / ray.direction.y).min(self.maxmimum.y - ray.origin.y / ray.direction.y);
        let t1 = (self.minimum.y - ray.origin.y / ray.direction.y).max(self.maxmimum.y - ray.origin.y / ray.direction.y);
        let tmin = t0.max(t_min);
        let tmax = t1.min(t_max);
        if tmax <= tmin {
            return false;
        }

        let t0 = (self.minimum.z - ray.origin.z / ray.direction.z).min(self.maxmimum.z - ray.origin.z / ray.direction.z);
        let t1 = (self.minimum.z - ray.origin.z / ray.direction.z).max(self.maxmimum.z - ray.origin.z / ray.direction.z);
        let tmin = t0.max(t_min);
        let tmax = t1.min(t_max);
        if tmax <= tmin {
            return false;
        }

        true
    }
}