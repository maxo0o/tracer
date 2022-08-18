use crate::aabb::{surrounding_box, AxisAlignedBoundingBox};
use crate::camera::Camera;
use crate::ray::Ray;
use crate::utils::distance;
use crate::vector::Vec3;

use obj::{Obj, TexturedVertex};
use std::collections::HashSet;
use std::f64::INFINITY;
use std::fmt;

#[derive(Debug, Clone)]
pub struct UVCoord {
    pub u: f64,
    pub v: f64,
}

#[derive(Debug, Clone)]
pub struct Face {
    points: [Vec3; 3],
    text_coords: [UVCoord; 3],
    normals: [Vec3; 3],
    bounds: AxisAlignedBoundingBox,
}

impl Face {
    pub fn new(points: [Vec3; 3], text_coords: [UVCoord; 3], normals: [Vec3; 3]) -> Face {
        let minimum = Vec3::new(
            points[0].x.min(points[1].x).min(points[2].x),
            points[0].y.min(points[1].y).min(points[2].y),
            points[0].z.min(points[1].z).min(points[2].z),
        );
        let maximum = Vec3::new(
            points[0].x.min(points[1].x).min(points[2].x),
            points[0].y.min(points[1].y).min(points[2].y),
            points[0].z.min(points[1].z).min(points[2].z),
        );
        let bounds = AxisAlignedBoundingBox::new(minimum, maximum);
        Face {
            points,
            text_coords,
            normals,
            bounds,
        }
    }
}

pub struct KDTreeHitRecord {
    pub p: Vec3,
    pub t: f64,
    pub normal: Vec3,
    pub tangent: Vec3,
    pub bitangent: Vec3,
    pub front_face: bool,
    pub text_coord: UVCoord,
}

#[derive(Clone)]
pub struct KDTree {
    pub split_axis: usize,
    pub left_child: Option<Box<KDTree>>,
    pub right_child: Option<Box<KDTree>>,
    pub split_distance: f64,
    pub is_leaf: bool,
    pub faces: Option<Vec<Box<Face>>>,
}

impl fmt::Debug for KDTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "KDTree")
    }
}

#[derive(Copy, Clone)]
enum EdgeType {
    Start,
    End,
}

#[derive(Clone)]
struct BoundEdge {
    t: f64,
    triangle_num: usize,
    edge_type: EdgeType,
}

impl BoundEdge {
    fn new(t: f64, triangle_num: usize, starting: bool) -> BoundEdge {
        let edge_type = match starting {
            true => EdgeType::Start,
            false => EdgeType::End,
        };
        BoundEdge {
            t,
            triangle_num,
            edge_type,
        }
    }
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
                let mut closest_t_so_far = t_end;
                for triangle in faces {
                    if let (Some(hit), distance_min) = triangle_intersection(
                        t_start,
                        closest_t_so_far,
                        ray,
                        triangle,
                        camera,
                        d_min,
                    ) {
                        closest_t_so_far = hit.t;
                        potential_hit = Some(hit);
                        d_min = distance_min;
                    }
                }
            }

            return potential_hit;
        }

        let flip_front_and_back = ray_dir[self.split_axis].is_sign_negative();
        if t_split <= t_start {
            if let (Some(chosen_child), true) = (&self.left_child, flip_front_and_back) {
                return chosen_child.traverse(ray, camera, t_start, t_end);
            } else if let (Some(chosen_child), false) = (&self.right_child, flip_front_and_back) {
                return chosen_child.traverse(ray, camera, t_start, t_end);
            }
        } else if t_split >= t_end {
            if let (Some(chosen_child), true) = (&self.right_child, flip_front_and_back) {
                return chosen_child.traverse(ray, camera, t_start, t_end);
            } else if let (Some(chosen_child), false) = (&self.left_child, flip_front_and_back) {
                return chosen_child.traverse(ray, camera, t_start, t_end);
            }
        } else if let (Some(right_child), true) = (&self.right_child, flip_front_and_back) {
            if let Some(KDTreeHitRecord {
                p,
                t: t_hit,
                normal,
                tangent,
                bitangent,
                front_face,
                text_coord,
            }) = right_child.traverse(ray, camera, t_start, t_split)
            {
                if t_hit < t_split {
                    return Some(KDTreeHitRecord {
                        p,
                        t: t_split,
                        normal,
                        tangent,
                        bitangent,
                        front_face,
                        text_coord,
                    });
                }
            }

            if let Some(left_child) = &self.left_child {
                return left_child.traverse(ray, camera, t_split, t_end);
            }
        } else if let (Some(left_child), false) = (&self.left_child, flip_front_and_back) {
            if let Some(KDTreeHitRecord {
                p,
                t: t_hit,
                normal,
                tangent,
                bitangent,
                front_face,
                text_coord,
            }) = left_child.traverse(ray, camera, t_start, t_split)
            {
                if t_hit < t_split {
                    return Some(KDTreeHitRecord {
                        p,
                        t: t_split,
                        normal,
                        tangent,
                        bitangent,
                        front_face,
                        text_coord,
                    });
                }
            }

            if let Some(right_child) = &self.right_child {
                return right_child.traverse(ray, camera, t_split, t_end);
            }
        }

        None
    }

    // build KDTree by splitting on the median of sorted triangles
    pub fn build(
        triangle_list: &mut [Box<Face>],
        max_depth: u32,
        depth: u32,
    ) -> Option<Box<KDTree>> {
        if triangle_list.is_empty() {
            return None;
        }

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
        let median = triangle_list.len() / 2_usize;

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
                is_leaf: true,
                faces: Some(triangle_list.to_vec()),
            }));
        }

        // find any points that may not have been placed on the correct side
        let mut left_additional = vec![];
        let mut right_additional = vec![];
        for triangle in triangle_list.iter() {
            let mut point_on_right = false;
            let mut point_on_left = false;
            let mut points_on_boundary = false;
            if triangle.points[0].get(axis) > split_distance
                || triangle.points[1].get(axis) > split_distance
                || triangle.points[2].get(axis) > split_distance
            {
                point_on_right = true;
            }

            if triangle.points[0].get(axis) < split_distance
                || triangle.points[1].get(axis) < split_distance
                || triangle.points[2].get(axis) < split_distance
            {
                point_on_left = true;
            }

            if triangle.points[0].get(axis) == split_distance
                && triangle.points[1].get(axis) == split_distance
                && triangle.points[2].get(axis) == split_distance
            {
                points_on_boundary = true;
            }

            if (point_on_left && point_on_right) || points_on_boundary {
                right_additional.push(Box::new(*triangle.clone()));
                left_additional.push(Box::new(*triangle.clone()));
            } else if point_on_left {
                left_additional.push(Box::new(*triangle.clone()));
            } else if point_on_right {
                right_additional.push(Box::new(*triangle.clone()));
            }
        }

        let left_child = KDTree::build(&mut left_additional[..], max_depth, depth + 1);
        let right_child = KDTree::build(&mut right_additional[..], max_depth, depth + 1);

        Some(Box::new(KDTree {
            split_axis: axis,
            left_child,
            right_child,
            split_distance: median_triangle.points[0].get(axis),
            is_leaf: false,
            faces: None,
        }))
    }

    //build KDTree using Surface Area Heuristic
    pub fn build_sah(
        triangle_list: &mut [Box<Face>],
        triangle_list_len: usize,
        depth: u32,
        bounds: AxisAlignedBoundingBox,
        bad_refines: u32,
    ) -> Option<Box<KDTree>> {
        if triangle_list_len <= 500 || depth == 0 {
            return Some(Box::new(KDTree {
                split_axis: 0,
                left_child: None,
                right_child: None,
                split_distance: 0.0,
                is_leaf: true,
                faces: Some(triangle_list.to_vec()),
            }));
        };

        let mut best_axis = -1;
        let mut best_offset = -1;
        let mut best_cost = INFINITY;
        let isect_cost = 80.0;
        let traversal_cost = 1.0;
        let old_cost = triangle_list_len as f64 * isect_cost;
        let total_sa = bounds.surface_area();
        let inv_total_sa = 1.0 / total_sa;
        let d = bounds.maximum - bounds.minimum;
        let mut axis = bounds.maximum_extent() as usize;
        let mut retries = 0;
        let empty_bonus = 0.75;
        let mut edges: [Vec<BoundEdge>; 3] = [
            vec![BoundEdge::new(0.0, 0, true); 2_usize * triangle_list_len],
            vec![BoundEdge::new(0.0, 0, true); 2_usize * triangle_list_len],
            vec![BoundEdge::new(0.0, 0, true); 2_usize * triangle_list_len],
        ];

        while best_axis == -1 && retries < 2 {
            for i in 0..triangle_list_len {
                let t_bounds = &triangle_list[i].bounds;
                edges[axis][2 * i] = BoundEdge::new(t_bounds.minimum.get(axis), i, true);
                edges[axis][2 * i + 1] = BoundEdge::new(t_bounds.maximum.get(axis), i, false);
            }

            edges[axis].sort_by(|edge0, edge1| {
                if edge0.t == edge1.t {
                    return (edge0.edge_type as u32).cmp(&(edge1.edge_type as u32));
                }
                edge0.t.partial_cmp(&edge1.t).unwrap()
            });

            let mut number_below = 0;
            let mut number_above = triangle_list_len as u32;
            for i in 0..(2_usize * triangle_list_len) {
                match edges[axis][i].edge_type {
                    EdgeType::Start => {}
                    EdgeType::End => {
                        number_above -= 1;
                    }
                }

                let edge_t = edges[axis][i].t;
                if edge_t > bounds.minimum.get(axis) && edge_t < bounds.maximum.get(axis) {
                    let other_axis0 = (axis + 1) % 3;
                    let other_axis1 = (axis + 2) % 3;
                    let below_sa = 2.0
                        * (d.get(other_axis0) * d.get(other_axis1)
                            + (edge_t - bounds.minimum.get(axis))
                                * (d.get(other_axis0) + d.get(other_axis1)));
                    let above_sa = 2.0
                        * (d.get(other_axis0) * d.get(other_axis1)
                            + (bounds.maximum.get(axis) - edge_t)
                                * (d.get(other_axis0) + d.get(other_axis1)));

                    let p_below = below_sa * inv_total_sa;
                    let p_above = above_sa * inv_total_sa;
                    let eb = if number_above == 0 || number_below == 0 {
                        empty_bonus
                    } else {
                        0.0
                    };
                    let cost = traversal_cost
                        + isect_cost
                            * (1.0 - eb)
                            * (p_below * number_below as f64 + p_above * number_above as f64);

                    if cost < best_cost {
                        best_cost = cost;
                        best_axis = axis as i32;
                        best_offset = i as i32;
                    }
                }

                match edges[axis][i].edge_type {
                    EdgeType::Start => number_below += 1,
                    EdgeType::End => {}
                }
            }

            if best_axis == -1 && retries < 2 {
                retries += 1;
                axis = (axis + 1) % 3;
                continue;
            }

            break;
        }

        let mut b_refines = bad_refines;
        if best_cost > old_cost {
            b_refines += 1;
        }
        if (best_cost > 4.0 * old_cost && triangle_list_len < 16)
            || best_axis == -1
            || b_refines == 3
        {
            return Some(Box::new(KDTree {
                split_axis: 0,
                left_child: None,
                right_child: None,
                split_distance: 0.0,
                is_leaf: true,
                faces: Some(triangle_list.to_vec()),
            }));
        }

        let mut in_both = HashSet::new();
        let mut left = vec![];
        let mut right = vec![];
        for i in 0..best_offset as usize {
            match edges[best_axis as usize][i].edge_type {
                EdgeType::Start => {
                    left.push(triangle_list[edges[best_axis as usize][i].triangle_num].clone())
                }
                EdgeType::End => {
                    in_both.insert(edges[best_axis as usize][i].triangle_num);
                }
            }
        }

        for i in (best_offset as usize + 1)..(2 * triangle_list_len) {
            match edges[best_axis as usize][i].edge_type {
                EdgeType::End => {
                    right.push(triangle_list[edges[best_axis as usize][i].triangle_num].clone());
                    in_both.remove(&edges[best_axis as usize][i].triangle_num);
                }
                EdgeType::Start => {
                    in_both.insert(edges[best_axis as usize][i].triangle_num);
                }
            }
        }

        eprintln!("{}", in_both.len());
        //for triangle_num in in_both {
        //  left.push(triangle_list[triangle_num].clone());
        //right.push(triangle_list[triangle_num].clone());
        //}

        let t_split = edges[best_axis as usize][best_offset as usize].t;
        let mut bounds_left = bounds.clone();
        let mut bounds_right = bounds;
        bounds_left.maximum.set(best_axis as usize, t_split);
        bounds_right.minimum.set(best_axis as usize, t_split);
        let left_len = left.len();
        let right_len = right.len();

        let left_child =
            KDTree::build_sah(&mut left[..], left_len, depth - 1, bounds_left, bad_refines);
        let right_child = KDTree::build_sah(
            &mut right[..],
            right_len,
            depth - 1,
            bounds_right,
            bad_refines,
        );

        Some(Box::new(KDTree {
            split_axis: axis,
            left_child,
            right_child,
            split_distance: t_split,
            is_leaf: false,
            faces: None,
        }))
    }
}

pub fn build_from_obj(
    object: Obj<TexturedVertex, u32>,
) -> (Vec<Box<Face>>, AxisAlignedBoundingBox) {
    let mut points = vec![];
    let mut minimum = Vec3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
    let mut maximum = Vec3::new(-f64::INFINITY, -f64::INFINITY, -f64::INFINITY);

    let bounding_box_padding = 0.25;

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

        let normal1 = object.vertices[indices[0] as usize].normal;
        let normal1 = Vec3::new(normal1[0] as f64, normal1[1] as f64, normal1[2] as f64);

        let normal2 = object.vertices[indices[1] as usize].normal;
        let normal2 = Vec3::new(normal2[0] as f64, normal2[1] as f64, normal2[2] as f64);

        let normal3 = object.vertices[indices[2] as usize].normal;
        let normal3 = Vec3::new(normal3[0] as f64, normal3[1] as f64, normal3[2] as f64);

        let face = Face::new([p1, p2, p3], [uv1, uv2, uv3], [normal1, normal2, normal3]);

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

    minimum.x -= bounding_box_padding;
    minimum.y -= bounding_box_padding;
    minimum.z -= bounding_box_padding;

    maximum.x += bounding_box_padding;
    maximum.y += bounding_box_padding;
    maximum.z += bounding_box_padding;

    let bounding_box = AxisAlignedBoundingBox::new(minimum, maximum);
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

    let p1p2 = p2 - p1;
    let p1p3 = p3 - p1;
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

    let edge0 = p2 - p1;
    let v_p1 = p - p1;
    let c0 = edge0.cross(&v_p1);
    if n.dot(&c0) < 0.0 {
        return (None, 0.0);
    }

    let edge1 = p3 - p2;
    let v_p2 = p - p2;
    let c1 = edge1.cross(&v_p2);
    if n.dot(&c1) < 0.0 {
        return (None, 0.0);
    }

    let edge2 = p1 - p3;
    let v_p3 = p - p3;
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
    let mut smooth_normal = Vec3::new(0.0, 0.0, 0.0);
    if let Some((b1, b2)) = get_bary_coords(&p1, &p2, &p3, &p) {
        let b0 = 1.0 - b1 - b2;
        text_coord.u =
            b0 * face.text_coords[0].u + b1 * face.text_coords[1].u + b2 * face.text_coords[2].u;
        text_coord.v =
            b0 * face.text_coords[0].v + b1 * face.text_coords[1].v + b2 * face.text_coords[2].v;

        smooth_normal = b0 * &face.normals[0] + b1 * &face.normals[1] + b2 * &face.normals[2];
    }

    // remap coordinate basis used for normal mapping
    let edge1 = p2 - p1;
    let edge2 = p3 - p1;

    let uv1 = Vec3::new(
        face.text_coords[1].u - face.text_coords[0].u,
        face.text_coords[1].v - face.text_coords[0].v,
        0.0,
    );
    let uv2 = Vec3::new(
        face.text_coords[2].u - face.text_coords[0].u,
        face.text_coords[2].v - face.text_coords[0].v,
        0.0,
    );

    let f = 1.0 / (uv1.x * uv2.y - uv2.x * uv1.y);

    let tangent = Vec3::new(
        f * (uv2.y * edge1.x - uv1.y * edge2.x),
        f * (uv2.y * edge1.y - uv1.y * edge2.y),
        f * (uv2.y * edge1.z - uv1.y * edge2.z),
    );

    let bitangent = Vec3::new(
        f * (-uv2.x * edge1.x + uv1.x * edge2.x),
        f * (-uv2.x * edge1.y + uv1.x * edge2.y),
        f * (-uv2.x * edge1.z + uv1.x * edge2.z),
    );

    // compute smooth normals
    let normal = smooth_normal.unit();

    (
        Some(KDTreeHitRecord {
            p,
            t,
            normal,
            tangent: tangent.unit(),
            bitangent: bitangent.unit(),
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
