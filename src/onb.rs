use crate::vector::Vec3;

pub struct OrthonormalBasis {
    axis: [Vec3; 3],
}

#[allow(dead_code)]
impl OrthonormalBasis {
    pub fn u(&self) -> Vec3 {
        self.axis[0]
    }

    pub fn v(&self) -> Vec3 {
        self.axis[1]
    }

    pub fn w(&self) -> Vec3 {
        self.axis[2]
    }

    pub fn local(&self, a: f64, b: f64, c: f64) -> Vec3 {
        a * &self.u() + b * &self.v() + c * &self.w()
    }

    pub fn local_vec(&self, a: &Vec3) -> Vec3 {
        a.x * &self.u() + a.y * &self.v() + a.z * &self.w()
    }

    pub fn build_from_w(n: &Vec3) -> OrthonormalBasis {
        let w = n.unit();
        let a = if w.x.abs() > 0.9 {
            Vec3::new(0.0, 1.0, 0.0)
        } else {
            Vec3::new(1.0, 0.0, 0.0)
        };
        let v = w.cross(&a).unit();
        let u = w.cross(&v);

        OrthonormalBasis { axis: [u, v, w] }
    }
}
