use std::f64::INFINITY;

use crate::hittable::{HitRecord, Hittable};
use crate::kdtree::{build, build_from_obj, KDTree, KDTreeHitRecord};
use crate::material::Material;
use crate::ray::Ray;
use crate::utils::distance;
use crate::vector::Vec3;
use obj::Obj;
use std::sync::{Arc, Mutex};

pub struct Object<M: Material> {
    pub tree: Box<KDTree>,
    material: M,
}

impl<M: Material> Object<M> {
    pub fn new(object: Obj, material: M) -> Object<M> {
        let mut faces = build_from_obj(object);

        if let Some(tree) = build(&mut faces[..], 20, 0) {
            return Object { tree, material };
        } else {
            panic!("Problem building kdtree");
        }
    }
}

impl<T: Material> Hittable for Object<T> {
    fn hit(
        &self,
        ray: &Ray,
        _t_min: f64,
        _t_max: f64,
        p_0: u32,
        p_1: u32,
        zbuffer: Arc<Mutex<Vec<Vec<f64>>>>,
    ) -> Option<HitRecord> {
        let mut potential_hit: Option<HitRecord> = None;
        let mut _position = Vec3::new(INFINITY, INFINITY, INFINITY);

        if let Some(KDTreeHitRecord {
            p,
            t,
            normal,
            front_face,
        }) = self.tree.traverse(ray, _t_min, _t_max)
        {
            return Some(HitRecord {
                p: p,
                t: t,
                normal: normal,
                material: &self.material,
                front_face: front_face,
            });
        }

        potential_hit
    }
}
