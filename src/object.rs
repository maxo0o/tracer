use crate::aabb::AxisAlignedBoundingBox;
use crate::camera::Camera;
use crate::hittable::{HitRecord, Hittable};
use crate::kdtree::{build, build_from_obj, KDTree, KDTreeHitRecord};
use crate::material::Material;
use crate::ray::Ray;
use obj::Obj;

#[derive(Clone)]
pub struct Object<M: Material> {
    pub tree: Box<KDTree>,
    material: M,
}

impl<M: Material> Object<M> {
    pub fn new(object: Obj, material: M) -> Object<M> {
        let mut faces = build_from_obj(object);

        if let Some(tree) = build(&mut faces[..], 15, 0) {
            return Object { tree, material };
        } else {
            panic!("Problem building kdtree");
        }
    }
}

impl<T: Material> Hittable for Object<T> {
    fn hit(&self, ray: &Ray, camera: &Camera, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if let Some(KDTreeHitRecord {
            p,
            t,
            normal,
            front_face,
        }) = self.tree.traverse(ray, camera, t_min, t_max)
        {
            return Some(HitRecord {
                p,
                t,
                normal,
                material: &self.material,
                front_face,
            });
        }

        None
    }

    fn bounding_box(&self) -> Option<AxisAlignedBoundingBox> {
        None
    }
}
