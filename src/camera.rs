use crate::ray::Ray;
use crate::utils::random_in_unit_disk;
use crate::vector::Vec3;

pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub lens_radius: f64,
    u: Vec3,
    v: Vec3,
}

impl Camera {
    pub fn new(
        look_from: &Vec3,
        look_at: &Vec3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Camera {
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).unit();
        let u = vup.cross(&w).unit();
        let v = w.cross(&u);

        let origin = Vec3::copy(&look_from);
        let horizontal = focus_dist * viewport_width * &u;
        let vertical = focus_dist * viewport_height * &v;
        let lower_left_corner = &origin - &horizontal / 2.0 - &vertical / 2.0 - focus_dist * &w;

        let lens_radius = aperture / 2.0;

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            lens_radius,
            u,
            v,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * &random_in_unit_disk();
        let offset = rd.x * &self.u + rd.y * &self.v;

        let origin_c = Vec3::new(self.origin.x, self.origin.y, self.origin.z);
        let direction = &self.lower_left_corner + s * &self.horizontal + t * &self.vertical
            - &self.origin
            - &offset;
        Ray::new(origin_c + offset, direction)
    }
}
