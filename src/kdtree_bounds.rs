use crate::camera::Camera;
use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;

use std::f64::INFINITY;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct KDTreeBounds {
    pub split_axis: usize,
    pub left_child: Option<Box<KDTreeBounds>>,
    pub right_child: Option<Box<KDTreeBounds>>,
    pub split_distance: f64,
    pub location: Arc<Box<dyn Hittable>>,
    pub is_leaf: bool,
    pub objects: Vec<Arc<Box<dyn Hittable>>>,
}

impl KDTreeBounds {
    pub fn traverse(
        &self,
        ray: &Ray,
        camera: &Camera,
        t_start: f64,
        t_end: f64,
        pixel: Option<(usize, usize)>,
        zbuffer: Arc<Mutex<Vec<Vec<f64>>>>,
    ) -> Option<HitRecord> {
        let ray_origin = [ray.origin.x, ray.origin.y, ray.origin.z];
        let ray_dir = [ray.direction.x, ray.direction.y, ray.direction.z];
        let t_split =
            (self.split_distance - ray_origin[self.split_axis]) / ray_dir[self.split_axis];

        if self.is_leaf {
            let mut potential_hit: Option<HitRecord> = None;
            for object in &self.objects {
                if let Some(hit) =
                    object.hit(ray, camera, t_start, t_end, pixel, Arc::clone(&zbuffer))
                {
                    potential_hit = Some(hit);
                }
            }

            return potential_hit;
        }

        let flip_front_and_back = ray_dir[self.split_axis].is_sign_negative();
        if t_split <= t_start {
            if let (Some(chosen_child), true) = (&self.left_child, flip_front_and_back) {
                return chosen_child.traverse(
                    ray,
                    camera,
                    t_start,
                    t_end,
                    pixel,
                    Arc::clone(&zbuffer),
                );
            } else if let (Some(chosen_child), false) = (&self.right_child, flip_front_and_back) {
                return chosen_child.traverse(
                    ray,
                    camera,
                    t_start,
                    t_end,
                    pixel,
                    Arc::clone(&zbuffer),
                );
            }
        } else if t_split >= t_end {
            if let (Some(chosen_child), true) = (&self.right_child, flip_front_and_back) {
                return chosen_child.traverse(
                    ray,
                    camera,
                    t_start,
                    t_end,
                    pixel,
                    Arc::clone(&zbuffer),
                );
            } else if let (Some(chosen_child), false) = (&self.left_child, flip_front_and_back) {
                return chosen_child.traverse(
                    ray,
                    camera,
                    t_start,
                    t_end,
                    pixel,
                    Arc::clone(&zbuffer),
                );
            }
        } else if let (Some(right_child), true) = (&self.right_child, flip_front_and_back) {
            if let Some(HitRecord {
                p,
                t: t_hit,
                normal,
                tangent,
                bitangent,
                front_face,
                u,
                v,
                material,
            }) = right_child.traverse(ray, camera, t_start, t_split, pixel, Arc::clone(&zbuffer))
            {
                if t_hit < t_split {
                    return Some(HitRecord {
                        p,
                        t: t_split,
                        normal,
                        tangent,
                        bitangent,
                        front_face,
                        u,
                        v,
                        material,
                    });
                }
            }

            if let Some(left_child) = &self.left_child {
                return left_child.traverse(
                    ray,
                    camera,
                    t_split,
                    t_end,
                    pixel,
                    Arc::clone(&zbuffer),
                );
            }
        } else if let (Some(left_child), false) = (&self.left_child, flip_front_and_back) {
            if let Some(HitRecord {
                p,
                t: t_hit,
                normal,
                tangent,
                bitangent,
                front_face,
                u,
                v,
                material,
            }) = left_child.traverse(ray, camera, t_start, t_split, pixel, Arc::clone(&zbuffer))
            {
                if t_hit < t_split {
                    return Some(HitRecord {
                        p,
                        t: t_split,
                        normal,
                        tangent,
                        bitangent,
                        front_face,
                        u,
                        v,
                        material,
                    });
                }
            }

            if let Some(right_child) = &self.right_child {
                return right_child.traverse(
                    ray,
                    camera,
                    t_split,
                    t_end,
                    pixel,
                    Arc::clone(&zbuffer),
                );
            }
        }

        None
    }

    pub fn build(
        object_list: &mut [Arc<Box<dyn Hittable>>],
        max_depth: u32,
        depth: u32,
    ) -> Option<Box<KDTreeBounds>> {
        let axis = (depth % 3) as usize;
        object_list.sort_by(|object_a, object_b| {
            // Sort the points inside the triangle by axis too
            let box_a = object_a.bounding_box().unwrap();
            let box_b = object_b.bounding_box().unwrap();

            match axis {
                0 => box_a.minimum.x.partial_cmp(&box_b.minimum.x).unwrap(),
                1 => box_a.minimum.y.partial_cmp(&box_b.minimum.y).unwrap(),
                2 => box_a.minimum.z.partial_cmp(&box_b.minimum.z).unwrap(),
                _ => panic!("Invalid axis!"),
            }
        });
        let median = object_list.len() / 2_usize;

        let median_object = object_list[median].clone();

        let split_distance = median_object.bounding_box().unwrap().minimum.get(axis);
        if object_list.len() <= 15 || depth == max_depth {
            return Some(Box::new(KDTreeBounds {
                split_axis: axis,
                left_child: None,
                right_child: None,
                split_distance,
                location: median_object,
                is_leaf: true,
                objects: object_list.to_vec(),
            }));
        }

        // find any points that may not have been placed on the correct side
        let mut left_additional = vec![];
        let mut right_additional = vec![];
        for (i, object) in object_list.iter().enumerate() {
            let mut point_on_right = false;
            let mut point_on_left = false;
            if object.bounding_box().unwrap().maximum.get(axis) >= split_distance {
                point_on_right = true;
            }

            if object.bounding_box().unwrap().minimum.get(axis) <= split_distance {
                point_on_left = true;
            }

            if point_on_left && point_on_right {
                match i.cmp(&median) {
                    std::cmp::Ordering::Less => right_additional.push(Arc::clone(object)),
                    std::cmp::Ordering::Greater => left_additional.push(Arc::clone(object)),
                    std::cmp::Ordering::Equal => {}
                }
            }
        }

        let mut left_points = vec![];
        let mut right_points = vec![];

        for left_point in &object_list[..median] {
            left_points.push(Arc::clone(left_point));
        }

        for left_additional_point in &left_additional {
            left_points.push(Arc::clone(left_additional_point));
        }

        for right_point in &object_list[median..] {
            right_points.push(Arc::clone(right_point));
        }

        for right_additional_point in &right_additional {
            right_points.push(Arc::clone(right_additional_point));
        }

        let left_child = KDTreeBounds::build(&mut left_points[..], max_depth, depth + 1);
        let right_child = KDTreeBounds::build(&mut right_points[..], max_depth, depth + 1);

        Some(Box::new(KDTreeBounds {
            split_axis: axis,
            left_child,
            right_child,
            split_distance: median_object.bounding_box().unwrap().minimum.get(axis),
            location: median_object,
            is_leaf: false,
            objects: vec![],
        }))
    }
}
