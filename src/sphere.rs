use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vector::Vec3;
use crate::camera::Camera;
use crate::aabb::AxisAlignedBoundingBox;

pub struct Sphere<T: Material> {
    pub center: Vec3,
    pub radius: f64,
    pub material: T,
}

impl<T: Material> Sphere<T> {
    pub fn new(center: Vec3, radius: f64, material: T) -> Sphere<T> {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl<T: Material> Hittable for Sphere<T> {
    fn hit(&self, ray: &Ray, _camera: &Camera, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = &ray.origin - &self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(&ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let mut root = (-half_b - discriminant.sqrt()) / a;
        if root < t_min || t_max < root {
            root = (-half_b + discriminant.sqrt()) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let mut outward_normal = (&ray.at(root) - &self.center) / self.radius;
        let front_face = ray.direction.dot(&outward_normal) < 0.0;
        if !front_face {
            outward_normal = -outward_normal;
        }

        let p = ray.at(root);

        Some(HitRecord {
            p,
            t: root,
            normal: outward_normal,
            material: &self.material,
            front_face,
        })
    }

    fn bounding_box(&self) -> Option<AxisAlignedBoundingBox> {
        let point_a = &self.center - Vec3::new(self.radius, self.radius, self.radius);
        let point_b = &self.center + Vec3::new(self.radius, self.radius, self.radius);

        Some(AxisAlignedBoundingBox::new(point_a, point_b))
    }
}
