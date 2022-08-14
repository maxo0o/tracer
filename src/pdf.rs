use rand::Rng;
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
    fn generate(&self) -> Option<Vec3>;
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
        _camera: &Camera,
        _pixel: Option<(usize, usize)>,
        _zbuffer: Arc<Mutex<Vec<Vec<f64>>>>,
    ) -> f64 {
        let cosine = direction.unit().dot(&self.uvw.w());

        if cosine <= 0.0 {
            0.0
        } else {
            cosine / PI
        }
    }

    fn generate(&self) -> Option<Vec3> {
        Some(self.uvw.local_vec(&random_cosine_direction()))
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
        self.hittable
            .pdf_value(&self.origin, direction, camera, pixel, Arc::clone(&zbuffer))
    }

    fn generate(&self) -> Option<Vec3> {
        self.hittable.random(&self.origin)
    }
}

pub struct MixturePDF {
    pdfs: Vec<Box<dyn ProbabilityDensityFunction>>,
}

impl MixturePDF {
    pub fn new(pdfs: Vec<Box<dyn ProbabilityDensityFunction>>) -> MixturePDF {
        MixturePDF { pdfs }
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
        self.pdfs
            .iter()
            .map(|pdf| {
                (1.0 / self.pdfs.len() as f64)
                    * pdf.value(direction, camera, pixel, Arc::clone(&zbuffer)) as f64
            })
            .fold(0.0, |acc, x| acc + x)
    }

    fn generate(&self) -> Option<Vec3> {
        let mut pdf: Option<Vec3> = None;
        while pdf.is_none() {
            let choice = rand::thread_rng().gen_range(0..self.pdfs.len());
            pdf = self.pdfs[choice].generate();
        }
        pdf
    }
}
