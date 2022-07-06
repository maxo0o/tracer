use crate::ray::Ray;
use crate::vector::Vec3;
use crate::utils::distance;

use obj::Obj;

#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    points: [[f64; 3]; 3],
}

impl Triangle {
    fn copy(t: &Triangle) -> Triangle {
        Triangle {
            points: [
                [t.points[0][0], t.points[0][1], t.points[0][2]],
                [t.points[1][0], t.points[1][1], t.points[1][2]],
                [t.points[2][0], t.points[2][1], t.points[2][2]],
            ],
        }
    }
}

pub struct KDTreeHitRecord {
    pub p: Vec3,
    pub t: f64,
    pub normal: Vec3,
    pub front_face: bool,
}

#[derive(Debug)]
pub struct KDTree {
    pub split_axis: usize,
    pub left_child: Option<Box<KDTree>>,
    pub right_child: Option<Box<KDTree>>,
    pub split_distance: f64,
    pub location: Box<Triangle>,
    pub is_leaf: bool,
    pub points: Option<Vec<Box<Triangle>>>,
}

impl KDTree {
    pub fn traverse(&self, ray: &Ray, t_start: f64, t_end: f64) -> Option<KDTreeHitRecord> {
        let ray_origin = [ray.origin.x, ray.origin.y, ray.origin.z];
        let ray_dir = [ray.direction.x, ray.direction.y, ray.direction.z];
        let mut d_min = f64::INFINITY;
        let t_split = (self.split_distance - ray_origin[self.split_axis]) / ray_dir[self.split_axis];
        if self.is_leaf {
            // eprintln!("Hit a leaf!");
            let mut potential_hit: Option<KDTreeHitRecord> = None;
            if let Some(points) = &self.points {
                for triangle in points {
                    let p1 = Vec3::new(
                        triangle.points[0][0],
                        triangle.points[0][1],
                        triangle.points[0][2],
                    );
                    let p2 = Vec3::new(
                        triangle.points[1][0],
                        triangle.points[1][1],
                        triangle.points[1][2],
                    );
                    let p3 = Vec3::new(
                        triangle.points[2][0],
                        triangle.points[2][1],
                        triangle.points[2][2],
                    );

                    let p1p2 = &p2 - &p1;
                    let p1p3 = &p3 - &p1;
                    let n = p1p2.cross(&p1p3);

                    let triangle_ray_dot_product = n.dot(&ray.direction);
                    if triangle_ray_dot_product.abs() == 0.0 {
                        continue;
                    }

                    let d = -n.dot(&p1);

                    let t = -(n.dot(&ray.origin) + d) / triangle_ray_dot_product;
                    if t < 0.0 {
                        continue;
                    }

                    // TODO IS THIS WRONG?
                    if t_start > t || t > t_end {
                        continue;
                    }

                    let p = ray.at(t);

                    let edge0 = &p2 - &p1;
                    let v_p1 = &p - &p1;
                    let c0 = edge0.cross(&v_p1);
                    if n.dot(&c0) < 0.0 {
                        continue;
                    }

                    let edge1 = &p3 - &p2;
                    let v_p2 = &p - &p2;
                    let c1 = edge1.cross(&v_p2);
                    if n.dot(&c1) < 0.0 {
                        continue;
                    }

                    let edge2 = p1 - &p3;
                    let v_p3 = &p - &p3;
                    let c2 = edge2.cross(&v_p3);
                    if n.dot(&c2) < 0.0 {
                        continue;
                    }

                    let n_norm = n.unit();
                    let mut _front_face = true;
                    if ray.direction.dot(&n_norm) > 0.0 {
                        _front_face = false;
                        continue;
                    }

                    let cam_look_from = Vec3::new(8.0, 2.0, 2.0);
                    let z_distance = distance(&p, &cam_look_from).abs();
                    if z_distance <= d_min {
                        d_min = z_distance;
                    } else {
                        continue;
                    }


                    potential_hit = Some(KDTreeHitRecord {
                        p,
                        t,
                        normal: n_norm,
                        front_face: _front_face,
                    });
                }
            }

            return potential_hit;
        }

        // eprintln!(
        //     "split_distance: {}, axis: {}",
        //     self.split_distance, self.split_axis
        // );
        // eprintln!("{}", t);
        let flip_front_and_back = ray_dir[self.split_axis].is_sign_negative();
        if t_split <= t_start {

            // eprintln!("Right side");
            if flip_front_and_back {
                if let Some(chosen_child) = &self.left_child {
                    return chosen_child.traverse(ray, t_start, t_end);
                }
            } else {
                if let Some(chosen_child) = &self.right_child {
                    return chosen_child.traverse(ray, t_start, t_end);
                }
            }
        } else if t_split >= t_end {
            // eprintln!("Left side");
            if flip_front_and_back {
                if let Some(chosen_child) = &self.right_child {
                    return chosen_child.traverse(ray, t_start, t_end);
                }
            } else {
                if let Some(chosen_child) = &self.left_child {
                    return chosen_child.traverse(ray, t_start, t_end);
                }
            }
        } else {
            // left first
            // eprintln!("WUT");

            if flip_front_and_back {
                if let Some(right_child) = &self.right_child {
                    if let Some(KDTreeHitRecord {
                        p,
                        t: t_hit,
                        normal,
                        front_face,
                    }) = right_child.traverse(ray, t_start, t_split)
                    {
                        // eprintln!("t_hit: {}", t_hit);
                        if t_hit < t_split {
                            // eprintln!("t_hit hit left???");
                            return Some(KDTreeHitRecord {
                                p,
                                t: t_split,
                                normal,
                                front_face,
                            });
                        }
                    }

                    if let Some(left_child) = &self.left_child {
                        // eprintln!("GO RIGHT? HELLO?");
                        return left_child.traverse(ray, t_split, t_end);
                    }
                }
            } else {
                if let Some(left_child) = &self.left_child {
                    if let Some(KDTreeHitRecord {
                        p,
                        t: t_hit,
                        normal,
                        front_face,
                    }) = left_child.traverse(ray, t_start, t_split)
                    {
                        // eprintln!("t_hit: {}", t_hit);
                        if t_hit < t_split {
                            // eprintln!("t_hit hit left???");
                            return Some(KDTreeHitRecord {
                                p,
                                t: t_split,
                                normal,
                                front_face,
                            });
                        }
                    }

                    if let Some(right_child) = &self.right_child {
                        // eprintln!("GO RIGHT? HELLO?");
                        return right_child.traverse(ray, t_split, t_end);
                    }
                }
            }
        }

        None
    }
}

pub fn build(point_list: &mut [Box<Triangle>], max_depth: u32, depth: u32) -> Option<Box<KDTree>> {
    let axis = (depth % 3) as usize; // only 2D for now - CHANGE for 3D
    point_list.sort_by(|triangle_a, triangle_b| {
        // Sort the points inside the triangle by axis too
        let mut triangle_a_0 = Triangle::clone(triangle_a);
        let mut triangle_b_0 = Triangle::clone(triangle_b);

        triangle_a_0
            .points
            .sort_by(|a, b| a[axis].partial_cmp(&b[axis]).unwrap());
        triangle_b_0
            .points
            .sort_by(|a, b| a[axis].partial_cmp(&b[axis]).unwrap());

        triangle_a_0.points[0][axis]
            .partial_cmp(&triangle_b_0.points[0][axis])
            .unwrap()
    });
    let median = point_list.len() / 2 as usize;

    let mut median_triangle = Triangle::copy(&point_list[median]);
    median_triangle
        .points
        .sort_by(|a, b| a[axis].partial_cmp(&b[axis]).unwrap());

    if point_list.len() == 1 || depth == max_depth {
        return Some(Box::new(KDTree {
            split_axis: axis,
            left_child: None,
            right_child: None,
            split_distance: median_triangle.points[0][axis],
            location: Box::new(median_triangle),
            is_leaf: true,
            points: Some(point_list.to_vec()),
        }));
    }

    let mut left_additional = vec![];
    let mut right_additional = vec![];
    for i in 0..point_list.len() {
        let mut point_on_right = false;
        let mut point_on_left = false;
        if point_list[i].points[0][axis] >= median_triangle.points[0][axis] {
            point_on_right = true;
        } else if point_list[i].points[1][axis] >= median_triangle.points[0][axis] {
            point_on_right = true;
        } else if point_list[i].points[2][axis] >= median_triangle.points[0][axis] {
            point_on_right = true;
        }

        if point_list[i].points[0][axis] <= median_triangle.points[0][axis] {
            point_on_left = true;
        } else if point_list[i].points[1][axis] <= median_triangle.points[0][axis] {
            point_on_left = true;
        } else if point_list[i].points[2][axis] <= median_triangle.points[0][axis] {
            point_on_left = true;
        }
        
        if point_on_left && point_on_right {
            if i < median {
                right_additional.push(Triangle::copy(&point_list[i]));
            } 
            else if i > median {
                left_additional.push(Triangle::copy(&point_list[i]));
            }
        }
    }

    let mut left_points = vec![];
    let mut right_points = vec![];

    for left_point in &point_list[..median] {
        left_points.push(Box::new(Triangle::copy(left_point)));
    }

    for left_additional_point in &left_additional {
        left_points.push(Box::new(Triangle::copy(left_additional_point)));
    }

    for right_point in &point_list[median..] {
        right_points.push(Box::new(Triangle::copy(right_point)));
    }

    for right_additional_point in &right_additional {
        right_points.push(Box::new(Triangle::copy(right_additional_point)));
    }

    let left_child = build(&mut left_points[..], max_depth, depth + 1);
    let right_child = build(&mut right_points[..], max_depth, depth + 1);

    Some(Box::new(KDTree {
        split_axis: axis,
        left_child,
        right_child,
        split_distance: median_triangle.points[0][axis],
        location: Box::new(median_triangle),
        is_leaf: false,
        points: None,
    }))
}

pub fn build_from_obj<'a>(object: Obj) -> Vec<Box<Triangle>> {
    let mut points = vec![];

    for indices in object.indices.chunks(3) {
        let p1 = [
            object.vertices[indices[0] as usize].position[0].into(),
            object.vertices[indices[0] as usize].position[1].into(),
            object.vertices[indices[0] as usize].position[2].into(),
        ];
        let p2 = [
            object.vertices[indices[1] as usize].position[0].into(),
            object.vertices[indices[1] as usize].position[1].into(),
            object.vertices[indices[1] as usize].position[2].into(),
        ];
        let p3 = [
            object.vertices[indices[2] as usize].position[0].into(),
            object.vertices[indices[2] as usize].position[1].into(),
            object.vertices[indices[2] as usize].position[2].into(),
        ];

        let triangle = Triangle {
            points: [p1, p2, p3],
        };
        points.push(Box::new(triangle));
    }

    points
}

fn side(p: [f64; 3], q: [f64; 3], a: [f64; 3], b: [f64; 3]) -> f64 {
    let z1 = (b[0] - a[0]) * (p[1] - a[1]) - (p[0] - a[0]) * (b[1] - a[1]);
    let z2 = (b[0] - a[0]) * (q[1] - a[1]) - (q[0] - a[0]) * (b[1] - a[1]);
    z1 * z2
}

fn left_of(a: [f64; 3], b: [f64; 3], point: [f64; 3]) -> f64 {
    (b[0] - a[0]) * (point[1] - a[1]) - (b[1] - a[1]) * (point[0] - a[0])
}
