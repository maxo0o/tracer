use crate::aabb::AxisAlignedBoundingBox;
use crate::camera::Camera;
use crate::hittable::{HitRecord, Hittable};
use crate::kdtree::{build_from_obj, KDTree, KDTreeHitRecord};
use crate::material::Material;
use crate::material::UnitMaterial;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::utils::distance;
use crate::vector::Vec3;

use obj::{Obj, TexturedVertex};
use std::fmt;
use std::sync::{Arc, Mutex};

pub struct Object {
    pub tree: Box<KDTree>,
    material: Box<dyn Material>,
    bounding_box: AxisAlignedBoundingBox,
    render: bool,
}

impl fmt::Debug for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Object",)
    }
}

impl Object {
    pub fn new(
        object: Obj<TexturedVertex, u32>,
        material: Box<dyn Material>,
        shade_smooth: bool,
        render: bool,
    ) -> Object {
        let (mut faces, bounding_box) = build_from_obj(object, shade_smooth);

        let faces_len = faces.len();
        let depth = 8 + (1.3 * (faces_len as f64).log2()) as u32;
        if let Some(tree) =
            KDTree::build_sah(&mut faces[..], faces_len, depth, bounding_box.clone(), 0)
        {
            Object {
                tree,
                material,
                bounding_box,
                render,
            }
        } else {
            panic!("Problem building kdtree");
        }
    }
}

impl Hittable for Object {
    fn hit(
        &self,
        ray: &Ray,
        camera: &Camera,
        t_min: f64,
        t_max: f64,
        pixel: Option<(usize, usize)>,
        zbuffer: Arc<Mutex<Vec<Vec<f64>>>>,
        _first_ray: bool,
    ) -> Option<HitRecord> {
        //eprintln!("Search object");
        if let Some(KDTreeHitRecord {
            p,
            t,
            normal,
            tangent,
            bitangent,
            front_face,
            text_coord,
        }) = self.tree.traverse(ray, camera, t_min, t_max)
        {
            // only check the zbuffer at first ray level
            if let Some(pixel) = pixel {
                let z_distance = distance(&p, &camera.origin);
                let mut zbuff = zbuffer.lock().unwrap();

                if z_distance < zbuff[pixel.0][pixel.1] {
                    zbuff[pixel.0][pixel.1] = z_distance;
                } else {
                    return None;
                }
            }

            return Some(HitRecord {
                p,
                t,
                normal,
                tangent: Some(tangent),
                bitangent: Some(bitangent),
                material: &self.material,
                front_face,
                u: text_coord.u,
                v: text_coord.v,
            });
        }

        None
    }

    fn get_light_sampler_sphere(&self) -> Sphere {
        let mut center = Vec3::new(
            (self.bounding_box.minimum.x + self.bounding_box.maximum.x) / 2.0,
            (self.bounding_box.minimum.y + self.bounding_box.maximum.y) / 2.0,
            (self.bounding_box.minimum.z + self.bounding_box.maximum.z) / 2.0,
        );

        let r1 = self.bounding_box.maximum.x - self.bounding_box.minimum.x;
        let r2 = self.bounding_box.maximum.y - self.bounding_box.minimum.y;
        let r3 = self.bounding_box.maximum.z - self.bounding_box.minimum.z;

        let radius = r1.abs().max(r2.abs()).max(r3.abs()) / 2.0;

        // small offset
        center.y += radius / 4.0;

        Sphere {
            center,
            radius,
            material: Box::new(UnitMaterial {}),
        }
    }

    fn bounding_box(&self) -> Option<AxisAlignedBoundingBox> {
        Some(AxisAlignedBoundingBox::new(
            self.bounding_box.minimum,
            self.bounding_box.maximum,
        ))
    }

    fn should_render(&self) -> bool {
        self.render
    }
}
