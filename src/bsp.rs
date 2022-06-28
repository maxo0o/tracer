use obj::Obj;
use std::rc::Rc;
use crate::vector::Vec3;

#[derive(Debug)]
pub struct BSPTree {
    pub faces: Vec<[Vec3; 3]>,
    pub infront: Option<Box<BSPTree>>,
    pub behind:  Option<Box<BSPTree>>,
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

        let tree = BSPTree {faces, infront: None, behind: None };
        tree
    }

    pub fn build(parent: &mut Box<BSPTree>, depth: i32) {
        if depth < 0 {
            return;
        }

        let parent_v0 = Vec3::copy(&parent.faces[0][0]);
        let parent_v1 = Vec3::copy(& parent.faces[0][1]);
        let parent_v2 = Vec3::copy(&parent.faces[0][2]);
        let p0p1 = &parent_v1 - &parent_v0;
        let p0p2 = &parent_v2 - &parent_v0;
        let hyperplane_normal = p0p1.cross(&p0p2);

        if parent.faces.len() < 2 { return; };
        for i in 1..(parent.faces.len()-1) {
            let v0 = Vec3::new(parent.faces[i][0].x, parent.faces[i][0].y, parent.faces[i][0].z);
            let v1 = Vec3::new(parent.faces[i][1].x, parent.faces[i][1].y, parent.faces[i][1].z);
            let v2 = Vec3::new(parent.faces[i][2].x, parent.faces[i][2].y, parent.faces[i][2].z);

            parent.add_face(&hyperplane_normal, [v0, v1, v2]);
        }
        // TODO need to remove items / keep items based on whether they were added in add_face - shouldn't remove
        // items on the same plane
        parent.faces = vec![[Vec3::copy(&parent.faces[0][0]), Vec3::copy(&parent.faces[0][1]), Vec3::copy(&parent.faces[0][2])]];

        if let Some(child_infront) = &mut parent.infront {
            BSPTree::build(child_infront, depth-1);
        } else {
            println!("No children infront?");
        }

        if let Some(child_behind) = &mut parent.behind {
            BSPTree::build(child_behind, depth-1);
        } else {
            println!("No children behind?");
        }
    }

    pub fn new() -> BSPTree {
        BSPTree { faces: vec![], infront: None, behind: None }
    }

    pub fn add_face(&mut self, hyperplane_normal: &Vec3, face: [Vec3; 3]) {
        let d = -(hyperplane_normal.x * self.faces[0][0].x +  hyperplane_normal.y * self.faces[0][0].y +  hyperplane_normal.z * self.faces[0][0].z);

        let side_f0 = plane_equation(hyperplane_normal, d, &face[0]);
        let side_f1 = plane_equation(hyperplane_normal, d, &face[1]);
        let side_f2 = plane_equation(hyperplane_normal, d, &face[2]);

        if side_f0 > 0.0 && side_f1 > 0.0 && side_f2 > 0.0 {
            // Behind hyperplane
            if let Some(infront) = &mut self.infront {
                infront.faces.push(face);
                return;
            }
            let mut infront_branch = Box::new(BSPTree::new());
            infront_branch.faces.push(face);
            self.infront = Some(infront_branch);

        } else if side_f0 < 0.0 && side_f1 < 0.0 && side_f2 < 0.0 {
            // Infront of hyperplane
            if let Some(behind) = &mut self.behind {
                behind.faces.push(face);
                return;
            }
            let mut behind_branch = Box::new(BSPTree::new());
            behind_branch.faces.push(face);
            self.infront = Some(behind_branch);

        } else if side_f0 == 0.0 && side_f1 == 0.0 && side_f2 == 0.0 {
            // On the plane
            // self.faces.push(face);
            // Do nothing because already in right place?
        }  else {
            // Intersection of hyperplane
            eprintln!("Intersection");
        }
    }

    // pub fn ray_insection_face(&self) -> [[f64; 3]; 3] {
        
    // }
}

fn plane_equation(normal: &Vec3, d: f64, point: &Vec3) -> f64 {
    normal.x * point.x + normal.y * point.y + normal.z * point.z + d
}
