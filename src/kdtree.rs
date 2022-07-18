use crate::aabb::AxisAlignedBoundingBox;
use crate::camera::Camera;
use crate::ray::Ray;
use crate::utils::distance;
use crate::vector::Vec3;

use obj::{Obj, TexturedVertex};

#[derive(Debug, Clone)]
pub struct UVCoord {
    pub u: f64,
    pub v: f64,
}

#[derive(Debug, Clone)]
pub struct Face {
    points: [Vec3; 3],
    text_coords: [UVCoord; 3],
}

pub struct KDTreeHitRecord {
    pub p: Vec3,
    pub t: f64,
    pub normal: Vec3,
    pub front_face: bool,
    pub text_coord: UVCoord,
}

#[derive(Debug, Clone)]
pub struct KDTree {
    pub split_axis: usize,
    pub left_child: Option<Box<KDTree>>,
    pub right_child: Option<Box<KDTree>>,
    pub split_distance: f64,
    pub location: Box<Face>,
    pub is_leaf: bool,
    pub faces: Option<Vec<Box<Face>>>,
}

impl KDTree {
    pub fn traverse(
        &self,
        ray: &Ray,
        camera: &Camera,
        t_start: f64,
        t_end: f64,
    ) -> Option<KDTreeHitRecord> {
        let ray_origin = [ray.origin.x, ray.origin.y, ray.origin.z];
        let ray_dir = [ray.direction.x, ray.direction.y, ray.direction.z];
        let mut d_min = f64::INFINITY;
        let t_split =
            (self.split_distance - ray_origin[self.split_axis]) / ray_dir[self.split_axis];

        if self.is_leaf {
            let mut potential_hit: Option<KDTreeHitRecord> = None;
            if let Some(faces) = &self.faces {
                for triangle in faces {
                    if let (Some(hit), distance_min) =
                        triangle_intersection(t_start, t_end, ray, &triangle, camera, d_min)
                    {
                        potential_hit = Some(hit);
                        d_min = distance_min;
                    }
                }
            }

            return potential_hit;
        }

        let flip_front_and_back = ray_dir[self.split_axis].is_sign_negative();
        if t_split <= t_start {
            if flip_front_and_back {
                if let Some(chosen_child) = &self.left_child {
                    return chosen_child.traverse(ray, camera, t_start, t_end);
                }
            } else {
                if let Some(chosen_child) = &self.right_child {
                    return chosen_child.traverse(ray, camera, t_start, t_end);
                }
            }
        } else if t_split >= t_end {
            if flip_front_and_back {
                if let Some(chosen_child) = &self.right_child {
                    return chosen_child.traverse(ray, camera, t_start, t_end);
                }
            } else {
                if let Some(chosen_child) = &self.left_child {
                    return chosen_child.traverse(ray, camera, t_start, t_end);
                }
            }
        } else {
            if flip_front_and_back {
                if let Some(right_child) = &self.right_child {
                    if let Some(KDTreeHitRecord {
                        p,
                        t: t_hit,
                        normal,
                        front_face,
                        text_coord,
                    }) = right_child.traverse(ray, camera, t_start, t_split)
                    {
                        if t_hit < t_split {
                            return Some(KDTreeHitRecord {
                                p,
                                t: t_split,
                                normal,
                                front_face,
                                text_coord,
                            });
                        }
                    }

                    if let Some(left_child) = &self.left_child {
                        return left_child.traverse(ray, camera, t_split, t_end);
                    }
                }
            } else {
                if let Some(left_child) = &self.left_child {
                    if let Some(KDTreeHitRecord {
                        p,
                        t: t_hit,
                        normal,
                        front_face,
                        text_coord,
                    }) = left_child.traverse(ray, camera, t_start, t_split)
                    {
                        if t_hit < t_split {
                            return Some(KDTreeHitRecord {
                                p,
                                t: t_split,
                                normal,
                                front_face,
                                text_coord,
                            });
                        }
                    }

                    if let Some(right_child) = &self.right_child {
                        return right_child.traverse(ray, camera, t_split, t_end);
                    }
                }
            }
        }

        None
    }

    pub fn build(
        triangle_list: &mut [Box<Face>],
        max_depth: u32,
        depth: u32,
    ) -> Option<Box<KDTree>> {
        let axis = (depth % 3) as usize;
        triangle_list.sort_by(|triangle_a, triangle_b| {
            // Sort the points inside the triangle by axis too
            let mut triangle_a_0 = *triangle_a.clone();
            let mut triangle_b_0 = *triangle_b.clone();

            triangle_a_0
                .points
                .sort_by(|a, b| a.get(axis).partial_cmp(&b.get(axis)).unwrap());
            triangle_b_0
                .points
                .sort_by(|a, b| a.get(axis).partial_cmp(&b.get(axis)).unwrap());

            triangle_a_0.points[0]
                .get(axis)
                .partial_cmp(&triangle_b_0.points[0].get(axis))
                .unwrap()
        });
        let median = triangle_list.len() / 2 as usize;

        let mut median_triangle = *triangle_list[median].clone();
        median_triangle
            .points
            .sort_by(|a, b| a.get(axis).partial_cmp(&b.get(axis)).unwrap());

        let split_distance = median_triangle.points[0].get(axis);
        if triangle_list.len() <= 15 || depth == max_depth {
            return Some(Box::new(KDTree {
                split_axis: axis,
                left_child: None,
                right_child: None,
                split_distance,
                location: Box::new(median_triangle),
                is_leaf: true,
                faces: Some(triangle_list.to_vec()),
            }));
        }

        // find any points that may not have been placed on the correct side
        let mut left_additional = vec![];
        let mut right_additional = vec![];
        for i in 0..triangle_list.len() {
            let mut point_on_right = false;
            let mut point_on_left = false;
            if triangle_list[i].points[0].get(axis) >= split_distance {
                point_on_right = true;
            } else if triangle_list[i].points[1].get(axis) >= split_distance {
                point_on_right = true;
            } else if triangle_list[i].points[2].get(axis) >= split_distance {
                point_on_right = true;
            }

            if triangle_list[i].points[0].get(axis) <= split_distance {
                point_on_left = true;
            } else if triangle_list[i].points[1].get(axis) <= split_distance {
                point_on_left = true;
            } else if triangle_list[i].points[2].get(axis) <= split_distance {
                point_on_left = true;
            }

            if point_on_left && point_on_right {
                if i < median {
                    right_additional.push(*triangle_list[i].clone());
                } else if i > median {
                    left_additional.push(*triangle_list[i].clone());
                }
            }
        }

        let mut left_points = vec![];
        let mut right_points = vec![];

        for left_point in &triangle_list[..median] {
            left_points.push(Box::new(*left_point.clone()));
        }

        for left_additional_point in &left_additional {
            left_points.push(Box::new(left_additional_point.clone()));
        }

        for right_point in &triangle_list[median..] {
            right_points.push(Box::new(*right_point.clone()));
        }

        for right_additional_point in &right_additional {
            right_points.push(Box::new(right_additional_point.clone()));
        }

        let left_child = KDTree::build(&mut left_points[..], max_depth, depth + 1);
        let right_child = KDTree::build(&mut right_points[..], max_depth, depth + 1);

        Some(Box::new(KDTree {
            split_axis: axis,
            left_child,
            right_child,
            split_distance: median_triangle.points[0].get(axis),
            location: Box::new(median_triangle),
            is_leaf: false,
            faces: None,
        }))
    }
}

pub fn build_from_obj<'a>(
    object: Obj<TexturedVertex, u32>,
) -> (Vec<Box<Face>>, AxisAlignedBoundingBox) {
    let mut points = vec![];
    let mut minimum = Vec3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
    let mut maximum = Vec3::new(-f64::INFINITY, -f64::INFINITY, -f64::INFINITY);

    for indices in object.indices.chunks(3) {
        let p1 = Vec3::new(
            object.vertices[indices[0] as usize].position[0].into(),
            object.vertices[indices[0] as usize].position[1].into(),
            object.vertices[indices[0] as usize].position[2].into(),
        );
        let p2 = Vec3::new(
            object.vertices[indices[1] as usize].position[0].into(),
            object.vertices[indices[1] as usize].position[1].into(),
            object.vertices[indices[1] as usize].position[2].into(),
        );
        let p3 = Vec3::new(
            object.vertices[indices[2] as usize].position[0].into(),
            object.vertices[indices[2] as usize].position[1].into(),
            object.vertices[indices[2] as usize].position[2].into(),
        );

        let uv1 = UVCoord {
            u: object.vertices[indices[0] as usize].texture[0] as f64,
            v: object.vertices[indices[0] as usize].texture[1] as f64,
        };
        let uv2 = UVCoord {
            u: object.vertices[indices[1] as usize].texture[0] as f64,
            v: object.vertices[indices[1] as usize].texture[1] as f64,
        };
        let uv3 = UVCoord {
            u: object.vertices[indices[2] as usize].texture[0] as f64,
            v: object.vertices[indices[2] as usize].texture[1] as f64,
        };

        let face = Face {
            points: [p1, p2, p3],
            text_coords: [uv1, uv2, uv3],
        };

        let min_x = p1.x.min(p2.x).min(p3.x);
        if min_x < minimum.x {
            minimum.x = min_x;
        }

        let min_y = p1.y.min(p2.y).min(p3.y);
        if min_y < minimum.y {
            minimum.y = min_y;
        }

        let min_z = p1.z.min(p2.z).min(p3.z);
        if min_z < minimum.z {
            minimum.z = min_z;
        }

        let max_x = p1.x.max(p2.x).max(p3.x);
        if max_x > maximum.x {
            maximum.x = max_x;
        }

        let max_y = p1.y.max(p2.y).max(p3.y);
        if max_y > maximum.y {
            maximum.y = max_y;
        }

        let max_z = p1.z.max(p2.z).max(p3.z);
        if max_z > maximum.z {
            maximum.z = max_z;
        }

        points.push(Box::new(face));
    }

    let bounding_box = AxisAlignedBoundingBox { minimum, maximum };
    (points, bounding_box)
}

fn triangle_intersection(
    t_start: f64,
    t_end: f64,
    ray: &Ray,
    face: &Face,
    camera: &Camera,
    mut d_min: f64,
) -> (Option<KDTreeHitRecord>, f64) {
    let p1 = Vec3::new(face.points[0].x, face.points[0].y, face.points[0].z);
    let p2 = Vec3::new(face.points[1].x, face.points[1].y, face.points[1].z);
    let p3 = Vec3::new(face.points[2].x, face.points[2].y, face.points[2].z);

    let p1p2 = &p2 - &p1;
    let p1p3 = &p3 - &p1;
    let n = p1p2.cross(&p1p3);

    let triangle_ray_dot_product = n.dot(&ray.direction);
    if triangle_ray_dot_product.abs() == 0.0 {
        return (None, 0.0);
    }

    let d = -n.dot(&p1);

    let t = -(n.dot(&ray.origin) + d) / triangle_ray_dot_product;
    if t < 0.0 {
        return (None, 0.0);
    }

    // TODO IS THIS WRONG?
    if t_start > t || t > t_end {
        return (None, 0.0);
    }

    let p = ray.at(t);

    let edge0 = &p2 - &p1;
    let v_p1 = &p - &p1;
    let c0 = edge0.cross(&v_p1);
    if n.dot(&c0) < 0.0 {
        return (None, 0.0);
    }

    let edge1 = &p3 - &p2;
    let v_p2 = &p - &p2;
    let c1 = edge1.cross(&v_p2);
    if n.dot(&c1) < 0.0 {
        return (None, 0.0);
    }

    let edge2 = p1 - &p3;
    let v_p3 = &p - &p3;
    let c2 = edge2.cross(&v_p3);
    if n.dot(&c2) < 0.0 {
        return (None, 0.0);
    }

    let mut n_norm = n.unit();
    let mut _front_face = true;
    if ray.direction.dot(&n_norm) > 0.0 {
        _front_face = false;
        n_norm = -n_norm;
        // return (None, 0.0);
    }

    let z_distance = distance(&p, &camera.origin).abs();
    if z_distance <= d_min {
        d_min = z_distance;
    } else {
        return (None, 0.0);
    }

    // Determine the UV coords of the hitpoint
    let mut text_coord = UVCoord { u: 0.3, v: 0.5 };
    if let Some((b1, b2)) = get_bary_coords(&p1, &p2, &p3, &p) {
        let b0 = 1.0 - b1 - b2;
        text_coord.u =
            b0 * face.text_coords[0].u + b1 * face.text_coords[1].u + b2 * face.text_coords[2].u;
        text_coord.v =
            b0 * face.text_coords[0].v + b1 * face.text_coords[1].v + b2 * face.text_coords[2].v;
    }

    (
        Some(KDTreeHitRecord {
            p,
            t,
            normal: n_norm,
            front_face: _front_face,
            text_coord,
        }),
        d_min,
    )
}

// This is used to work out the UV coordinates of the ray intersection point from the UV coordinates
// of the three vertices. Bary centric coords seem pretty cool...
fn get_bary_coords(p0: &Vec3, p1: &Vec3, p2: &Vec3, hit_point: &Vec3) -> Option<(f64, f64)> {
    let u = p1 - p0;
    let v = p2 - p0;
    let w = hit_point - p0;

    let v_cross_w = v.cross(&w);
    let v_cross_u = v.cross(&u);

    if v_cross_w.dot(&v_cross_u) < 0.0 {
        return None;
    }

    let u_cross_w = u.cross(&w);
    let u_cross_v = u.cross(&v);

    if u_cross_w.dot(&u_cross_v) < 0.0 {
        return None;
    }

    let denom = u_cross_v.length();
    let r = v_cross_w.length() / denom;
    let t = u_cross_w.length() / denom;

    if (r > 1.0) || (t > 1.0) || (r + t > 1.0) {
        return None;
    }

    Some((r, t))
}
