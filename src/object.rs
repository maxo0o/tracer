use crate::aabb::AxisAlignedBoundingBox;
use crate::camera::Camera;
use crate::colour::Colour;
use crate::hittable::{HitRecord, Hittable};
use crate::kdtree::{build_from_obj, KDTree, KDTreeHitRecord};
use crate::material::Material;
use crate::material::{Lambertian, UnitMaterial};
use crate::onb::OrthonormalBasis;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::texture::SolidColour;
use crate::utils::{distance, random_to_sphere};
use crate::vector::Vec3;

use obj::{Obj, TexturedVertex};
use std::f64::consts::PI;
use std::sync::{Arc, Mutex};

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
    fn hit(
        &self,
        ray: &Ray,
        camera: &Camera,
        t_min: f64,
        t_max: f64,
        pixel: Option<(usize, usize)>,
        zbuffer: Arc<Mutex<Vec<Vec<f64>>>>,
    ) -> Option<HitRecord> {
        if let Some(KDTreeHitRecord {
            p,
            t,
            normal,
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
                material: &self.material,
                front_face,
                u: text_coord.u,
                v: text_coord.v,
            });
        }

        None
    }

    // fn pdf_value(
    //         &self,
    //         origin: &Vec3,
    //         v: &Vec3,
    //         camera: &Camera,
    //         pixel: Option<(usize, usize)>,
    //         zbuffer: Arc<Mutex<Vec<Vec<f64>>>>,
    //     ) -> f64 {
    //         let r1 = self.bounding_box.maximum.x - self.bounding_box.minimum.x;
    //         let r2 = self.bounding_box.maximum.y - self.bounding_box.minimum.y;
    //         let r3 = self.bounding_box.maximum.z - self.bounding_box.minimum.z;

    //         let radius = r1.abs().max(r2.abs()).max(r3.abs()) / 2.0;

    //         let center = Vec3::new(
    //             (self.bounding_box.minimum.x + self.bounding_box.maximum.x) / 2.0,
    //             (self.bounding_box.minimum.y + self.bounding_box.maximum.y) / 2.0,
    //             (self.bounding_box.minimum.z + self.bounding_box.maximum.z) / 2.0,
    //         );

    //         let empty_mat = Lambertian {
    //             albedo: Box::new(SolidColour::new(Colour::new(0.73, 0.73, 0.73))),
    //         };

    //         if let Some(hit) = Sphere::new(center, radius, empty_mat).hit(
    //             &Ray::new(*origin, *v),
    //             camera,
    //             0.0001,
    //             f64::INFINITY,
    //             pixel,
    //             zbuffer,
    //         ) {
    //             let cos_theta_max = (1.0 - radius*radius / (center - origin).length_squared()).sqrt();
    //             let solid_angle = 2.0 * PI * (1.0 - cos_theta_max);

    //             return  1.0 / solid_angle;
    //         }

    //         0.0
    // }

    // fn random(&self, origin: &Vec3) -> Vec3 {
    //     let r1 = self.bounding_box.maximum.x - self.bounding_box.minimum.x;
    //     let r2 = self.bounding_box.maximum.y - self.bounding_box.minimum.y;
    //     let r3 = self.bounding_box.maximum.z - self.bounding_box.minimum.z;

    //     let radius = r1.abs().max(r2.abs()).max(r3.abs()) / 2.0;

    //     let center = Vec3::new(
    //         (self.bounding_box.minimum.x + self.bounding_box.maximum.x) / 2.0,
    //         (self.bounding_box.minimum.y + self.bounding_box.maximum.y) / 2.0,
    //         (self.bounding_box.minimum.z + self.bounding_box.maximum.z) / 2.0,
    //     );
    //     let direction = center - origin;
    //     let distance_squared = direction.length_squared();

    //     let uvw = OrthonormalBasis::build_from_w(&direction);
    //     return uvw.local_vec(&random_to_sphere(radius, distance_squared));
    // }

    fn get_light_sampler_sphere(&self) -> Sphere<UnitMaterial> {
        let center = Vec3::new(
            (self.bounding_box.minimum.x + self.bounding_box.maximum.x) / 2.0,
            (self.bounding_box.minimum.y + self.bounding_box.maximum.y) / 2.0,
            (self.bounding_box.minimum.z + self.bounding_box.maximum.z) / 2.0,
        );

        let r1 = self.bounding_box.maximum.x - self.bounding_box.minimum.x;
        let r2 = self.bounding_box.maximum.y - self.bounding_box.minimum.y;
        let r3 = self.bounding_box.maximum.z - self.bounding_box.minimum.z;

        let radius = r1.abs().max(r2.abs()).max(r3.abs()) / 2.0;

        eprintln!("radius: {:?}", radius);
        eprintln!("center: {:?}", center);

        Sphere {
            center,
            radius,
            material: UnitMaterial {},
        }
    }

    fn bounding_box(&self) -> Option<AxisAlignedBoundingBox> {
        Some(AxisAlignedBoundingBox::new(
            self.bounding_box.minimum,
            self.bounding_box.maximum,
        ))
    }
}
