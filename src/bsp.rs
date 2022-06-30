use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vector::Vec3;
use obj::Obj;
use std::rc::Rc;

#[derive(Debug)]
pub struct BSPTree {
    pub faces: Vec<[Vec3; 3]>,
    pub infront: Option<Box<BSPTree>>,
    pub behind: Option<Box<BSPTree>>,
}

impl BSPTree {
    pub fn build_new(object: Obj) -> BSPTree {
        let mut faces = Vec::new();

        for indices in object.indices.chunks(3) {
            let v0 = Vec3::new(
                object.vertices[indices[0] as usize].position[0].into(),
                object.vertices[indices[0] as usize].position[1].into(),
                object.vertices[indices[0] as usize].position[2].into(),
            );
            let v1 = Vec3::new(
                object.vertices[indices[1] as usize].position[0].into(),
                object.vertices[indices[1] as usize].position[1].into(),
                object.vertices[indices[1] as usize].position[2].into(),
            );
            let v2 = Vec3::new(
                object.vertices[indices[2] as usize].position[0].into(),
                object.vertices[indices[2] as usize].position[1].into(),
                object.vertices[indices[2] as usize].position[2].into(),
            );

            faces.push([v0, v1, v2]);
        }

        let tree = BSPTree {
            faces,
            infront: None,
            behind: None,
        };
        tree
    }

    pub fn build(parent: &mut Box<BSPTree>, depth: i32) {
        if depth < 0 {
            return;
        }

        let parent_v0 = Vec3::copy(&parent.faces[0][0]);
        let parent_v1 = Vec3::copy(&parent.faces[0][1]);
        let parent_v2 = Vec3::copy(&parent.faces[0][2]);
        let p0p1 = &parent_v1 - &parent_v0;
        let p0p2 = &parent_v2 - &parent_v0;
        let hyperplane_normal = p0p1.cross(&p0p2).unit();

        if parent.faces.len() < 2 {
            return;
        };
        for i in 1..(parent.faces.len() - 1) {
            let v0 = Vec3::new(
                parent.faces[i][0].x,
                parent.faces[i][0].y,
                parent.faces[i][0].z,
            );
            let v1 = Vec3::new(
                parent.faces[i][1].x,
                parent.faces[i][1].y,
                parent.faces[i][1].z,
            );
            let v2 = Vec3::new(
                parent.faces[i][2].x,
                parent.faces[i][2].y,
                parent.faces[i][2].z,
            );

            parent.add_face(&hyperplane_normal, [v0, v1, v2]);
        }
        // TODO need to remove items / keep items based on whether they were added in add_face - shouldn't remove
        // items on the same plane
        parent.faces = vec![[
            Vec3::copy(&parent.faces[0][0]),
            Vec3::copy(&parent.faces[0][1]),
            Vec3::copy(&parent.faces[0][2]),
        ]];

        if let Some(child_infront) = &mut parent.infront {
            BSPTree::build(child_infront, depth - 1);
        }

        if let Some(child_behind) = &mut parent.behind {
            BSPTree::build(child_behind, depth - 1);
        }
    }

    pub fn new() -> BSPTree {
        BSPTree {
            faces: vec![],
            infront: None,
            behind: None,
        }
    }

    pub fn ray_hit(&self, ray: &Ray) -> Option<BSPHit> {
        // check if ray insects faces plane...

        if self.faces.len() == 0 {
            return None;
        }

        let sample_face = &self.faces[0];
        let p0 = Vec3::copy(&sample_face[0]);
        let p1 = Vec3::copy(&sample_face[1]);
        let p2 = Vec3::copy(&sample_face[2]);

        if let (true, Some(p), Some(n)) = ray_hit_plane(ray, &p0, &p1, &p2) {
            let edge0 = &p1 - &p0;
            let v_p1 = &p - &p0;
            let c0 = edge0.cross(&v_p1);
            // if n.dot(&c0) < 0.0 {
            //     return None;
            // }

            let edge1 = &p2 - &p1;
            let v_p2 = &p - &p1;
            let c1 = edge1.cross(&v_p2);
            // if n.dot(&c1) < 0.0 {
            //     return None;
            // }

            let edge2 = p0 - &p2;
            let v_p3 = &p - &p2;
            let c2 = edge2.cross(&v_p3);
            // if n.dot(&c2) < 0.0 {
            //     return None;
            // }

            if n.dot(&c2) >= 0.0 && n.dot(&c1) >= 0.0 && n.dot(&c0) >= 0.0 {
                let n_norm = n.unit();
                let mut _front_face = true;
                if ray.direction.dot(&n_norm) > 0.0 {
                    _front_face = false;
                    return None;
                }

                return Some(BSPHit {
                    p,
                    normal: n_norm,
                    front_face: _front_face,
                });
            }
        }

        if let Some(infront) = &self.infront {
            return infront.ray_hit(ray);
        }

        if let Some(behind) = &self.behind {
            return behind.ray_hit(ray);
        }

        None
    }

    pub fn add_face(&mut self, hyperplane_normal: &Vec3, face: [Vec3; 3]) {
        let behind_plane = is_behind(hyperplane_normal, &self.faces[0][0], &face);
        let infront_plane = is_infront(hyperplane_normal, &self.faces[0][0], &face);

        let d = -(hyperplane_normal.x * &self.faces[0][0].x
            + hyperplane_normal.y * &self.faces[0][0].y
            + hyperplane_normal.z * &self.faces[0][0].z);

        let side_f0 = plane_equation(hyperplane_normal, d, &face[0]);
        let side_f1 = plane_equation(hyperplane_normal, d, &face[1]);
        let side_f2 = plane_equation(hyperplane_normal, d, &face[2]);

        if infront_plane {
            // Behind hyperplane
            eprintln!("INFRONT");
            if let Some(infront) = &mut self.infront {
                infront.faces.push(face);
                return;
            }
            let mut infront_branch = Box::new(BSPTree::new());
            infront_branch.faces.push(face);
            self.infront = Some(infront_branch);
        } else if behind_plane {
            // Infront of hyperplane
            eprintln!("BEHIND");
            if let Some(behind) = &mut self.behind {
                behind.faces.push(face);
                return;
            }
            let mut behind_branch = Box::new(BSPTree::new());
            behind_branch.faces.push(face);
            self.behind = Some(behind_branch);
        } else if side_f0 == 0.0 && side_f1 == 0.0 && side_f2 == 0.0 {
            // On the plane
            // self.faces.push(face);
            // Do nothing because already in right place?
            eprintln!("On the same plane");
        } else {
            // Intersection

            eprintln!("Intersection!");
            // let mut p0: Option<Vec3> = None;
            // let mut p1: Option<Vec3> = None;
            // let mut p2: Option<Vec3> = None;

            // if side_f0.is_sign_negative() != side_f1.is_sign_negative() {
            //     p0 = Some(find_intersection_plane_and_line(hyperplane_normal, &face[0], &face[1]));
            //     // eprintln!("Intersect at p0 {:?}", p0);
            //     // eprintln!("f0, f1 {}, {}", side_f0, side_f1);
            // }

            // if side_f0.is_sign_negative() != side_f2.is_sign_negative() {
            //     p1 = Some(find_intersection_plane_and_line(hyperplane_normal, &face[0], &face[2]));
            //     // eprintln!("Intersect at p1 {:?}", p1);
            //     // eprintln!("f0, f1 {}, {}", side_f0, side_f2);
            // }

            // if side_f1.is_sign_negative() != side_f2.is_sign_negative() {
            //     p2 = Some(find_intersection_plane_and_line(hyperplane_normal, &face[1], &face[2]));
            //     // eprintln!("Intersect at p2 {:?}", p2);
            //     // eprintln!("f0, f1 {}, {}", side_f1, side_f2);
            // }

            // // if p0 && p1
            // if let (Some(new_point0), Some(new_point1)) = (p0, p1) {
            //     let triangle0_new = [Vec3::copy(&face[0]), Vec3::copy(&new_point0), Vec3::copy(&new_point1)];
            //     let triangle1_new = [Vec3::copy(&new_point0), Vec3::copy(&new_point1), Vec3::copy(&face[1])];
            //     let triangle2_new = [Vec3::copy(&new_point1), Vec3::copy(&face[1]), Vec3::copy(&face[2])];

            //     if is_behind(hyperplane_normal, &self.faces[0][0], &triangle0_new) {
            //         if let Some(infront) = &mut self.infront {
            //             infront.faces.push(triangle0_new);
            //             eprintln!("????");
            //             return;
            //         }
            //         let mut infront_branch = Box::new(BSPTree::new());
            //         infront_branch.faces.push(triangle0_new);
            //         self.infront = Some(infront_branch);
            //     } else if is_infront(hyperplane_normal, &self.faces[0][0], &triangle0_new) {
            //         if let Some(behind) = &mut self.behind {
            //             behind.faces.push(face);
            //             eprintln!("????");
            //             return;
            //         }
            //         let mut behind_branch = Box::new(BSPTree::new());
            //         behind_branch.faces.push(face);
            //         self.infront = Some(behind_branch);
            //     } else {
            //         println!("NOPE");
            //     }

            //     if is_behind(hyperplane_normal, &self.faces[0][0], &triangle1_new) {
            //         if let Some(infront) = &mut self.infront {
            //             infront.faces.push(triangle1_new);
            //             eprintln!("????");
            //             return;
            //         }
            //         let mut infront_branch = Box::new(BSPTree::new());
            //         infront_branch.faces.push(triangle1_new);
            //         self.infront = Some(infront_branch);
            //     } else if is_infront(hyperplane_normal, &self.faces[0][0], &triangle1_new) {
            //         if let Some(behind) = &mut self.behind {
            //             behind.faces.push(triangle1_new);
            //             eprintln!("????");
            //             return;
            //         }
            //         let mut behind_branch = Box::new(BSPTree::new());
            //         behind_branch.faces.push(triangle1_new);
            //         self.infront = Some(behind_branch);
            //     } else {
            //         println!("NOPE");
            //     }

            //     if is_behind(hyperplane_normal, &self.faces[0][0], &triangle2_new) {
            //         if let Some(infront) = &mut self.infront {
            //             infront.faces.push(triangle2_new);
            //             eprintln!("????");
            //             return;
            //         }
            //         let mut infront_branch = Box::new(BSPTree::new());
            //         infront_branch.faces.push(triangle2_new);
            //         self.infront = Some(infront_branch);
            //     } else if is_infront(hyperplane_normal, &self.faces[0][0], &triangle2_new) {
            //         if let Some(behind) = &mut self.behind {
            //             behind.faces.push(triangle2_new);
            //             return;
            //         }
            //         let mut behind_branch = Box::new(BSPTree::new());
            //         behind_branch.faces.push(triangle2_new);
            //         self.infront = Some(behind_branch);
            //     } else {
            //         println!("NOPE");
            //     }

            // }
        }
    }

    // pub fn ray_insection_face(&self) -> [[f64; 3]; 3] {

    // }
}

pub struct BSPHit {
    pub p: Vec3,
    pub normal: Vec3,
    pub front_face: bool,
}

fn ray_hit_plane(ray: &Ray, p0: &Vec3, p1: &Vec3, p2: &Vec3) -> (bool, Option<Vec3>, Option<Vec3>) {
    let p0p1 = p1 - p0;
    let p0p2 = p2 - p0;
    let plane_normal = p0p1.cross(&p0p2).unit();

    let triangle_ray_dot_product = plane_normal.dot(&ray.direction);
    if triangle_ray_dot_product.abs() == 0.0 {
        return (false, None, None);
    }

    let d = -plane_normal.dot(&p1);

    let t = -(plane_normal.dot(&ray.origin) + d) / triangle_ray_dot_product;
    if t < 0.0 {
        return (false, None, None);
    }

    let p = ray.at(t);

    (true, Some(p), Some(plane_normal))
}

fn find_intersection_plane_and_line(p_normal: &Vec3, point_a: &Vec3, point_b: &Vec3) -> Vec3 {
    let l0 = point_b - point_a;
    let d = -(p_normal.dot(&point_a));
    let t = -(p_normal.dot(&point_a) + d) / (p_normal.dot(&l0));
    point_a + t * &l0
}

fn plane_equation(normal: &Vec3, d: f64, point: &Vec3) -> f64 {
    normal.x * point.x + normal.y * point.y + normal.z * point.z + d
}

fn is_infront(plane_normal: &Vec3, point_on_plane: &Vec3, face: &[Vec3; 3]) -> bool {
    let d = -(plane_normal.x * point_on_plane.x
        + plane_normal.y * point_on_plane.y
        + plane_normal.z * point_on_plane.z);

    let side_f0 = plane_equation(plane_normal, d, &face[0]);
    let side_f1 = plane_equation(plane_normal, d, &face[1]);
    let side_f2 = plane_equation(plane_normal, d, &face[2]);

    side_f0 <= 0.0 && side_f1 <= 0.0 && side_f2 <= 0.0
}

fn is_behind(plane_normal: &Vec3, point_on_plane: &Vec3, face: &[Vec3; 3]) -> bool {
    let d = -(plane_normal.x * point_on_plane.x
        + plane_normal.y * point_on_plane.y
        + plane_normal.z * point_on_plane.z);

    let side_f0 = plane_equation(plane_normal, d, &face[0]);
    let side_f1 = plane_equation(plane_normal, d, &face[1]);
    let side_f2 = plane_equation(plane_normal, d, &face[2]);

    side_f0 >= 0.0 && side_f1 >= 0.0 && side_f2 >= 0.0
}
