use crate::vector::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
    pub inverse_direction: Vec3,
    pub sign: [usize; 3],
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        // these are optimizations from scratchapixel for AABB intersection
        let inverse_direction = Vec3::new(1.0 / direction.x, 1.0 / direction.y, 1.0 / direction.z);
        let mut sign = [0, 0, 0];
        sign[0] = if inverse_direction.x < 0.0 { 1 } else { 0 };
        sign[1] = if inverse_direction.y < 0.0 { 1 } else { 0 };
        sign[2] = if inverse_direction.z < 0.0 { 1 } else { 0 };

        Ray {
            origin,
            direction,
            inverse_direction,
            sign,
        }
    }

    pub fn at(&self, t: f64) -> Vec3 {
        let a = Vec3 { ..self.origin };
        let b = Vec3 { ..self.direction };

        a + t * &b
    }
}
