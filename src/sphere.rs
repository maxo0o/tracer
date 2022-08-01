use crate::aabb::AxisAlignedBoundingBox;
use crate::camera::Camera;
use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::onb::OrthonormalBasis;
use crate::ray::Ray;
use crate::utils::{distance, random_in_unit_sphere, random_to_sphere};
use crate::vector::Vec3;

use std::f64::consts::PI;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Box<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Box<dyn Material>) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(
        &self,
        ray: &Ray,
        _camera: &Camera,
        t_min: f64,
        t_max: f64,
        _pixel: Option<(usize, usize)>,
        _zbuffer: Arc<Mutex<Vec<Vec<f64>>>>,
    ) -> Option<HitRecord> {
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

        let (u, v) = get_sphere_uv(&p, &self.center);

        Some(HitRecord {
            p,
            t: root,
            normal: outward_normal,
            material: &self.material,
            front_face,
            u,
            v,
        })
    }

    fn bounding_box(&self) -> Option<AxisAlignedBoundingBox> {
        let point_a = &self.center - Vec3::new(self.radius, self.radius, self.radius);
        let point_b = &self.center + Vec3::new(self.radius, self.radius, self.radius);

        Some(AxisAlignedBoundingBox::new(point_a, point_b))
    }

    fn pdf_value(
        &self,
        origin: &Vec3,
        v: &Vec3,
        camera: &Camera,
        pixel: Option<(usize, usize)>,
        zbuffer: Arc<Mutex<Vec<Vec<f64>>>>,
    ) -> f64 {
        // if the point is inside the light sampler we can't choose a point on the sphere
        // or else we get some weird behaviour
        if distance(origin, &self.center) < self.radius {
            return 0.0;
        }

        if let Some(hit) = self.hit(
            &Ray::new(*origin, *v),
            camera,
            0.0001,
            f64::INFINITY,
            pixel,
            zbuffer,
        ) {
            let cos_theta_max =
                (1.0 - self.radius * self.radius / (self.center - origin).length_squared()).sqrt();
            let solid_angle = 2.0 * PI * (1.0 - cos_theta_max);

            return 1.0 / solid_angle;
        }

        0.0
    }

    fn random(&self, origin: &Vec3) -> Option<Vec3> {
        // if the point is inside the light sampler we can't choose a point on the sphere
        // or else we get some weird behaviour
        if distance(origin, &self.center) < self.radius {
            return None;
        }

        let direction = self.center - origin;
        let distance_squared = direction.length_squared();

        let uvw = OrthonormalBasis::build_from_w(&direction);
        return Some(uvw.local_vec(&random_to_sphere(self.radius, distance_squared)));
    }
}

fn get_sphere_uv(p: &Vec3, center: &Vec3) -> (f64, f64) {
    let theta = -p.y.acos();
    let phi = -p.z.atan2(p.x) + std::f64::consts::PI;

    //(
    //  phi / (2.0 * std::f64::consts::PI),
    //   theta / std::f64::consts::PI,
    //)
    let n = (p - center).unit();
    let u = n.x.atan2(n.z) / (2.0 * std::f64::consts::PI) + 0.5;
    let v = n.y.asin() / std::f64::consts::PI + 0.5;

    (u, v)
}
