use crate::hittable::{HitRecord, Hittable, HittableList};
use crate::material::Material;
use crate::vector::Vec3;
use crate::colour::Colour;
use crate::texture::SolidColour;
use crate::material::Lambertian;

#[derive(Debug)]
pub enum PlaneOrientation {
    XY,
    XZ,
    YZ,
}

#[derive(Debug)]
pub struct Plane<T: Material> {
    pub orientation: PlaneOrientation,
    pub material: T,
    pub a0: f64,
    pub a1: f64,
    pub b0: f64,
    pub b1: f64,
    pub k: f64,
}

impl<T: Material> Plane<T> {
    pub fn new(
        points: (f64, f64, f64, f64),
        k: f64,
        material: T,
        orientation: PlaneOrientation,
    ) -> Plane<T> {
        Plane {
            material,
            a0: points.0,
            a1: points.1,
            b0: points.2,
            b1: points.3,
            k,
            orientation,
        }
    }
}

impl<T: Material + std::fmt::Debug> Hittable for Plane<T> {
    fn hit(
        &self,
        ray: &crate::ray::Ray,
        _camera: &crate::camera::Camera,
        t_min: f64,
        t_max: f64,
    ) -> Option<crate::hittable::HitRecord> {
        let t = match &self.orientation {
            PlaneOrientation::XY => (self.k - ray.origin.z) / ray.direction.z,
            PlaneOrientation::XZ => (self.k - ray.origin.y) / ray.direction.y,
            PlaneOrientation::YZ => (self.k - ray.origin.x) / ray.direction.x,
        };

        if t < t_min || t > t_max {
            return None;
        }

        let a = match &self.orientation {
            PlaneOrientation::XY | PlaneOrientation::XZ => ray.origin.x + t * ray.direction.x,
            PlaneOrientation::YZ => ray.origin.y + t * ray.direction.y,
        };

        let b = match self.orientation {
            PlaneOrientation::XY => ray.origin.y + t * ray.direction.y,
            PlaneOrientation::XZ | PlaneOrientation::YZ => ray.origin.z + t * ray.direction.z,
        };

        if a < self.a0 || a > self.a1 || b < self.b0 || b > self.b1 {
            return None;
        }

        let u = (a - self.a0) / (self.a1 - self.a0);
        let v = (b - self.b0) / (self.b1 - self.b0);

        let mut outward_normal = match &self.orientation {
            PlaneOrientation::XY => Vec3::new(0.0, 0.0, 1.0),
            PlaneOrientation::XZ => Vec3::new(0.0, 1.0, 0.0),
            PlaneOrientation::YZ => Vec3::new(1.0, 0.0, 0.0),
        };
        let p = ray.at(t);

        let n_norm = outward_normal.unit();
        let mut front_face = true;
        if ray.direction.dot(&n_norm) > 0.0 {
            front_face = false;
            outward_normal = -outward_normal;
        }

        Some(HitRecord {
            p,
            t,
            normal: outward_normal,
            material: &self.material,
            front_face,
            u,
            v,
        })
    }
}

#[derive(Debug)]
pub struct Cube {
    box_min: Vec3,
    box_max: Vec3,
    sides: HittableList,
    colour: Colour,
}

impl Cube {
    pub fn new(box_min: Vec3, box_max: Vec3, colour: Colour) -> Cube {
        let mut sides = HittableList::new();

        let side_colour_1 = Lambertian {
            albedo: Box::new(SolidColour::new(Colour::copy(&colour))),
        };
        let side_colour_2 = Lambertian {
            albedo: Box::new(SolidColour::new(Colour::copy(&colour))),
        };
        let side_colour_3 = Lambertian {
            albedo: Box::new(SolidColour::new(Colour::copy(&colour))),
        };
        let side_colour_4 = Lambertian {
            albedo: Box::new(SolidColour::new(Colour::copy(&colour))),
        };
        let side_colour_5 = Lambertian {
            albedo: Box::new(SolidColour::new(Colour::copy(&colour))),
        };
        let side_colour_6 = Lambertian {
            albedo: Box::new(SolidColour::new(Colour::copy(&colour))),
        };

        sides.objects.push(Box::new(Plane::new(
            (box_min.x, box_max.x, box_min.y, box_max.y),
            box_max.z,
            side_colour_1,
            PlaneOrientation::XY,
        )));
        sides.objects.push(Box::new(Plane::new(
            (box_min.x, box_max.x, box_min.y, box_max.y),
            box_min.z,
            side_colour_2,
            PlaneOrientation::XY,
        )));

        sides.objects.push(Box::new(Plane::new(
            (box_min.x, box_max.x, box_min.z, box_max.z),
            box_max.y,
            side_colour_3,
            PlaneOrientation::XZ,
        )));
        sides.objects.push(Box::new(Plane::new(
            (box_min.x, box_max.x, box_min.z, box_max.z),
            box_min.y,
            side_colour_4,
            PlaneOrientation::XZ,
        )));

        sides.objects.push(Box::new(Plane::new(
            (box_min.y, box_max.y, box_min.z, box_max.z),
            box_max.x,
            side_colour_5,
            PlaneOrientation::YZ,
        )));
        sides.objects.push(Box::new(Plane::new(
            (box_min.y, box_max.y, box_min.z, box_max.z),
            box_min.x,
            side_colour_6,
            PlaneOrientation::YZ,
        )));

        Cube { box_min, box_max, sides, colour}
    }
}

impl Hittable for Cube {
    fn hit(&self, ray: &crate::ray::Ray, camera: &crate::camera::Camera, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.sides.hit(ray, camera, t_min, t_max)
    }
}