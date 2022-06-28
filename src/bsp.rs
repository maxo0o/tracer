use crate::vector::Vec3;

pub struct BSPTree<'a> {
    pub faces: Vec<[[f64; 3]; 3]>,
    pub infront: Option<&'a mut BSPTree<'a>>,
    pub behind:  Option<&'a mut BSPTree<'a>>,
}

impl BSPTree<'_> {
    pub fn add_face(&mut self, camera_origin: &Vec3, hyperplane_normal: &Vec3, face: [[f64; 3]; 3]) {
        let d = -(hyperplane_normal.x * self.faces[0][0][0] +  hyperplane_normal.y * self.faces[0][0][1] +  hyperplane_normal.z * self.faces[0][0][2]);

        let side_f0 = plane_equation(hyperplane_normal, d, face[0]);
        let side_f1 = plane_equation(hyperplane_normal, d, face[1]);
        let side_f2 = plane_equation(hyperplane_normal, d, face[2]);

        if side_f0 > 0.0 && side_f1 > 0.0 && side_f2 > 0.0 {
            // behind?
            match self.infront {
                Some(infront) => {
                    infront.faces.push(face);
                },
                None => {
                    println!("Hi");
                }
            }
        } else if side_f0 < 0.0 && side_f1 < 0.0 && side_f2 < 0.0 {
            // infront? 
        } else {
            // intersection ?
        }
    }
}

fn plane_equation(normal: &Vec3, d: f64, point: [f64; 3]) -> f64 {
    normal.x * point[0] + normal.y * point[1] + normal.z * point[2] + d
}
