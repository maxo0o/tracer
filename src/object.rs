use crate::aabb::AxisAlignedBoundingBox;
use crate::camera::Camera;
use crate::hittable::{HitRecord, Hittable};
use crate::kdtree::{build_from_obj, KDTree, KDTreeHitRecord};
use crate::material::Material;
use crate::ray::Ray;
use obj::{Obj, TexturedVertex};

#[derive(Debug)]
pub struct Object<M: Material> {
    pub tree: Box<KDTree>,
    material: M,
    bounding_box: AxisAlignedBoundingBox,
}

impl<M: Material> Object<M> {
    pub fn new(object: Obj<TexturedVertex, u32>, material: M) -> Object<M> {
        let (mut faces, bounding_box) = build_from_obj(object);

        if let Some(tree) = KDTree::build(&mut faces[..], 15, 0) {
            return Object {
                tree,
                material,
                bounding_box,
            };
        } else {
            panic!("Problem building kdtree");
        }
    }
}

impl<T: Material + std::fmt::Debug> Hittable for Object<T> {
    fn hit(&self, ray: &Ray, camera: &Camera, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if let Some(KDTreeHitRecord {
            p,
            t,
            normal,
            front_face,
            text_coord,
        }) = self.tree.traverse(ray, camera, t_min, t_max)
        {
            return Some(HitRecord {
                p,
                t,
                normal,
                material: &self.material,
                front_face,
                u: text_coord.u,
                v: text_coord.v,
            });
        }

        None
    }

    fn bounding_box(&self) -> Option<AxisAlignedBoundingBox> {
        Some(AxisAlignedBoundingBox::new(
            self.bounding_box.minimum,
            self.bounding_box.maximum,
        ))
    }
}
