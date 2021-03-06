use std::f64::INFINITY;

use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vector::Vec3;
use crate::utils::{distance};
use obj::Obj;
use std::sync::{Arc, Mutex};

pub struct Object<M: Material> {
    object: Obj,
    material: M,
}

impl<M: Material> Object<M> {
    pub fn new(object: Obj, material: M) -> Object<M> {
        Object { object, material }
    }
}

impl<T: Material> Hittable for Object<T> {
    fn hit(&self, ray: &Ray, _t_min: f64, _t_max: f64, p_0: u32, p_1: u32, zbuffer: Arc<Mutex<Vec<Vec<f64>>>>) -> Option<HitRecord> {
        let mut potential_hit: Option<HitRecord> = None;
        let mut _position = Vec3::new(INFINITY, INFINITY, INFINITY);
        let mut min_distance = INFINITY;

        for indices in self.object.indices.chunks(3) {
            let v_0 = &self.object.vertices[indices[0] as usize];
            let v_1 = &self.object.vertices[indices[1] as usize];
            let v_2 = &self.object.vertices[indices[2] as usize];

            let p1 = Vec3::new(
                v_0.position[0].into(),
                v_0.position[1].into(),
                v_0.position[2].into(),
            );
            let p2 = Vec3::new(
                v_1.position[0].into(),
                v_1.position[1].into(),
                v_1.position[2].into(),
            );
            let p3 = Vec3::new(
                v_2.position[0].into(),
                v_2.position[1].into(),
                v_2.position[2].into(),
            );

            let p0p1 = &p2 - &p1;
            let p0p2 = &p3 - &p1;

            let pvec = ray.direction.cross(&p0p2);
            let det = p0p1.dot(&pvec);

            let _front_face = true;
            if det < 0.00001 {
                continue;
            }

            let inv_det = 1.0 / det;

            let tvec = &ray.origin - p1;
            let u = tvec.dot(&pvec) * inv_det;
            if u < 0.0 || u > 1.0 {
                continue;
            }

            let qvec = tvec.cross(&p0p1);
            let v = ray.direction.dot(&qvec) * inv_det;
            if v < 0.0 || u + v > 1.0 {
                continue;
            }

            let t = p0p2.dot(&qvec) * inv_det;
            let p = ray.at(t);

            let cam_look_from = Vec3::new(8.0, 2.0, 2.0);
            let z_distance = distance(&p, &cam_look_from).abs();

            // if z_distance < min_distance {
            //     min_distance = z_distance;
            // } else {
            //     continue;
            // }

            {
                let mut zbuff = zbuffer.lock().unwrap();
                
                if z_distance < zbuff[p_0 as usize][p_1 as usize] {
                    zbuff[p_0 as usize][p_1 as usize] = z_distance;
                } else {
                    continue;
                }
            }

            let n = p0p1.cross(&p0p2);

            potential_hit = Some(HitRecord {
                p,
                t,
                normal: n.unit(),
                material: &self.material,
                front_face: _front_face,
            });

        //     let p1p2 = &p2 - &p1;
        //     let p1p3 = &p3 - &p1;
        //     let n = p1p2.cross(&p1p3);

        //     let triangle_ray_dot_product = n.dot(&ray.direction);
        //     if triangle_ray_dot_product.abs() == 0.0 {
        //         continue;
        //     }

        //     let d = -n.dot(&p1);

        //     let t = -(n.dot(&ray.origin) + d) / triangle_ray_dot_product;
        //     if t < 0.0 {
        //         continue;
        //     }

        //     let p = ray.at(t);

        //     let edge0 = &p2 - &p1;
        //     let v_p1 = &p - &p1;
        //     let c0 = edge0.cross(&v_p1);
        //     if n.dot(&c0) < 0.0 {
        //         continue;
        //     }

        //     let edge1 = &p3 - &p2;
        //     let v_p2 = &p - &p2;
        //     let c1 = edge1.cross(&v_p2);
        //     if n.dot(&c1) < 0.0 {
        //         continue;
        //     }

        //     let edge2 = p1 - &p3;
        //     let v_p3 = &p - &p3;
        //     let c2 = edge2.cross(&v_p3);
        //     if n.dot(&c2) < 0.0 {
        //         continue;
        //     }

        //     let n_norm = n.unit();
        //     let mut _front_face = true;
        //     if ray.direction.dot(&n_norm) > 0.0 {
        //         _front_face = false;
        //         continue;
        //     }

        //     let cam_look_from = Vec3::new(8.0, 2.0, 2.0);
        //     let z_distance = distance(&p, &cam_look_from).abs();
        //     let mut zbuff = zbuffer.lock().unwrap();
            
        //     if z_distance < zbuff[p_0 as usize][p_1 as usize] {
        //         zbuff[p_0 as usize][p_1 as usize] = z_distance;
        //     } else {
        //         continue;
        //     }

        //     potential_hit = Some(HitRecord {
        //         p,
        //         t,
        //         normal: n_norm,
        //         material: &self.material,
        //         front_face: _front_face,
        //     });
        }

        // reset so sampling works correctly
        let mut zbuff = zbuffer.lock().unwrap();
        zbuff[p_0 as usize][p_1 as usize] = f64::INFINITY;

        potential_hit
    }
}

fn _signed_volume(a: &Vec3, b: &Vec3, c: &Vec3, d: &Vec3) -> f64 {
    (1.0 / 6.0) * &((b - a).cross(&(c - a))).dot(&(d-a))
}