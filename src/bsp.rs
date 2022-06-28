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
        let hyperplane_normal = p0p1.cross(&p0p2);

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
        } else {
            println!("No children infront?");
        }

        if let Some(child_behind) = &mut parent.behind {
            BSPTree::build(child_behind, depth - 1);
        } else {
            println!("No children behind?");
        }
    }

    pub fn new() -> BSPTree {
        BSPTree {
            faces: vec![],
            infront: None,
            behind: None,
        }
    }

    pub fn ray_hit(ray: &Ray) -> bool {
        // check if ray insects faces plane...
        // if yes
            // check faces list
            // if yes
                // return result
            // else
                // look through behind list
        // else
            // look through infront list
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

fn find_intersection_plane_and_line(p_normal: &Vec3, point_a: &Vec3, point_b: &Vec3) -> Vec3 {
    let l0 = point_b - point_a;
    let d = -(p_normal.x * point_a.x
        + p_normal.y * point_a.y
        + p_normal.z * point_a.z);
    let t = -(p_normal.x *point_a.x
        + p_normal.y *point_a.y
        + p_normal.z *point_a.z
        + d)
        / (p_normal.x * l0.x
            + p_normal.y * l0.y
            + p_normal.z * l0.z);
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
