use crate::ray::Ray;
use crate::vector::Vec3;

#[derive(Debug, Clone)]
pub struct AxisAlignedBoundingBox {
    pub minimum: Vec3,
    pub maximum: Vec3,
    pub bounds: [Vec3; 2],
    pub centroid: Vec3,
}

#[allow(dead_code)]
impl AxisAlignedBoundingBox {
    pub fn new(point_a: Vec3, point_b: Vec3) -> AxisAlignedBoundingBox {
        AxisAlignedBoundingBox {
            minimum: point_a,
            maximum: point_b,
            bounds: [point_a, point_b],
            centroid: 0.5 * &point_a + 0.5 * &point_b,
        }
    }

    pub fn offset(&self, p: Vec3) -> Vec3 {
        let mut o = p - &self.minimum;
        if self.maximum.x > self.minimum.x {
            o.x /= self.maximum.x - self.minimum.x;
        }
        if self.maximum.y > self.minimum.y {
            o.y /= self.maximum.y - self.minimum.y;
        }
        if self.maximum.z > self.minimum.z {
            o.z /= self.maximum.z - self.minimum.z;
        }

        o
    }

    pub fn surface_area(&self) -> f64 {
        let d = self.maximum - self.minimum;
        2.0 * (d.x * d.y + d.x * d.z + d.y * d.z)
    }

    pub fn maximum_extent(&self) -> u32 {
        let d = self.maximum - self.minimum;
        if d.x > d.y && d.x > d.z {
            0
        } else if d.y > d.z {
            1
        } else {
            2
        }
    }

    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> (bool, f64, f64) {
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
            return (false, 0.0, 0.0);
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
            return (false, 0.0, 0.0);
        }

        if tzmin > tmin {
            tmin = tzmin;
        }

        if tzmax < tmax {
            tmax = tzmax;
        }

        (true, tmin, tmax)
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
            box_a.maximum.x.max(box_b.maximum.x),
            box_a.maximum.y.max(box_b.maximum.y),
            box_a.maximum.z.max(box_b.maximum.z),
        );

        return Some(AxisAlignedBoundingBox::new(small, big));
    }
    eprintln!("Empty bounding boxes OH NO");
    None
}
