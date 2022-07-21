use std::f64::consts::PI;
use std::sync::{Arc, Mutex};

use crate::camera::Camera;
use crate::hittable::Hittable;
use crate::onb::OrthonormalBasis;
use crate::utils::random_cosine_direction;
use crate::vector::Vec3;

pub trait ProbabilityDensityFunction {
    fn value(
        &self,
        direction: &Vec3,
        camera: &Camera,
        pixel: Option<(usize, usize)>,
        zbuffer: Arc<Mutex<Vec<Vec<f64>>>>,
    ) -> f64;
    fn generate(&self) -> Vec3;
}

pub struct CosinePDF {
    uvw: OrthonormalBasis,
}

impl CosinePDF {
    pub fn new(w: &Vec3) -> CosinePDF {
        CosinePDF {
            uvw: OrthonormalBasis::build_from_w(w),
        }
    }
}

impl ProbabilityDensityFunction for CosinePDF {
    fn value(
        &self,
        direction: &Vec3,
        camera: &Camera,
        pixel: Option<(usize, usize)>,
        zbuffer: Arc<Mutex<Vec<Vec<f64>>>>,
    ) -> f64 {
        let cosine = direction.unit().dot(&self.uvw.w());

        return if cosine <= 0.0 { 0.0 } else { cosine / PI };
    }

    fn generate(&self) -> Vec3 {
        self.uvw.local_vec(&random_cosine_direction())
    }
}

pub struct HittablePDF {
    origin: Vec3,
    hittable: Arc<Box<dyn Hittable>>,
}

impl HittablePDF {
    pub fn new(origin: &Vec3, hittable: Arc<Box<dyn Hittable>>) -> HittablePDF {
        HittablePDF {
            origin: Vec3::copy(origin),
            hittable: Arc::clone(&hittable),
        }
    }
}

impl ProbabilityDensityFunction for HittablePDF {
    fn value(
        &self,
        direction: &Vec3,
        camera: &Camera,
        pixel: Option<(usize, usize)>,
        zbuffer: Arc<Mutex<Vec<Vec<f64>>>>,
    ) -> f64 {
        return self.hittable.pdf_value(
            &self.origin,
            direction,
            camera,
            pixel,
            Arc::clone(&zbuffer),
        );
    }

    fn generate(&self) -> Vec3 {
        return self.hittable.random(&self.origin);
    }
}

pub struct MixturePDF {
    pdf: [Box<dyn ProbabilityDensityFunction>; 2],
}

impl MixturePDF {
    pub fn new(
        p0: Box<dyn ProbabilityDensityFunction>,
        p1: Box<dyn ProbabilityDensityFunction>,
    ) -> MixturePDF {
        MixturePDF { pdf: [p0, p1] }
    }
}

impl ProbabilityDensityFunction for MixturePDF {
    fn value(
        &self,
        direction: &Vec3,
        camera: &Camera,
        pixel: Option<(usize, usize)>,
        zbuffer: Arc<Mutex<Vec<Vec<f64>>>>,
    ) -> f64 {
        0.5 * self.pdf[0].value(direction, camera, pixel, Arc::clone(&zbuffer))
            + 0.5 * self.pdf[1].value(direction, camera, pixel, Arc::clone(&zbuffer))
    }

    fn generate(&self) -> Vec3 {
        return if rand::random::<f64>() < 0.5 {
            self.pdf[0].generate()
        } else {
            self.pdf[1].generate()
        };
    }
}
