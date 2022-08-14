use crate::colour::Colour;
use crate::vector::Vec3;

use image::{DynamicImage, GenericImageView, Pixel};

pub trait Texture: std::fmt::Debug + Send + Sync {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Colour;

    fn normal_value(&self, _u: f64, _v: f64, _p: &Vec3) -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0)
    }

    fn alpha_value(&self, _u: f64, _v: f64) -> f64 {
        0.0
    }
}

#[derive(Debug)]
pub struct SolidColour {
    colour: Colour,
    pub normal_map: Option<DynamicImage>,
    pub normal_scale: Option<f64>,
}

impl SolidColour {
    pub fn new(
        colour: Colour,
        normal_map: Option<DynamicImage>,
        normal_scale: Option<f64>,
    ) -> SolidColour {
        SolidColour {
            colour,
            normal_map,
            normal_scale,
        }
    }
}

impl Texture for SolidColour {
    fn value(&self, _u: f64, _v: f64, _p: &Vec3) -> Colour {
        Colour::copy(&self.colour)
    }

    fn normal_value(&self, u: f64, v: f64, _p: &Vec3) -> Vec3 {
        normal_sample(u, v, self.normal_map.as_ref(), self.normal_scale)
    }
}

#[derive(Debug)]
pub struct CheckerTexture {
    pub odd: Box<dyn Texture>,
    pub even: Box<dyn Texture>,
}

impl CheckerTexture {
    pub fn _new(even: Box<dyn Texture>, odd: Box<dyn Texture>) -> CheckerTexture {
        CheckerTexture { even, odd }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Colour {
        let sines = (250.0 * u).sin() * (250.0 * v).sin();
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}

#[derive(Debug)]
pub struct ImageTexture {
    image: DynamicImage,
    alpha: Option<DynamicImage>,
    normal_map: Option<DynamicImage>,
    normal_scale: Option<f64>,
    width: u32,
    height: u32,
    is_light: bool,
    scale: f64,
}

impl ImageTexture {
    pub fn new(
        image: DynamicImage,
        alpha: Option<DynamicImage>,
        normal_map: Option<DynamicImage>,
        normal_scale: Option<f64>,
        is_light: bool,
        scale: f64,
    ) -> ImageTexture {
        let (width, height) = image.dimensions();
        ImageTexture {
            image,
            alpha,
            normal_map,
            normal_scale,
            width,
            height,
            is_light,
            scale,
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: &Vec3) -> Colour {
        let u = (u * self.scale).fract() % 1.0;
        let v = 1.0 - (v * self.scale).fract() % 1.0;

        let i = (u * self.width as f64) as u32;
        let j = (v * self.height as f64) as u32;

        let pixel = self
            .image
            .get_pixel(i.clamp(0, self.width - 1), j.clamp(0, self.height - 1))
            .to_rgb();

        let mut r = pixel[0] as f64 / 255.0;
        let mut g = pixel[1] as f64 / 255.0;
        let mut b = pixel[2] as f64 / 255.0;

        if self.is_light {
            r = r.powf(2.0);
            g = g.powf(2.0);
            b = b.powf(2.0);
        }

        if let Some(alpha) = &self.alpha {
            let alpha_pixel = alpha
                .get_pixel(i.clamp(0, self.width - 1), j.clamp(0, self.height - 1))
                .to_rgba();
            let alpha_pixel = alpha_pixel[3] as f64 / 255.0;

            if alpha_pixel < 0.1 && self.is_light {
                r = r.sqrt();
                g = g.sqrt();
                b = b.sqrt();
            } else {
                // r *= alpha_pixel;
                // g *= alpha_pixel;
                // b *= alpha_pixel;
            }
        }

        Colour::new(r, g, b)
    }

    fn alpha_value(&self, u: f64, v: f64) -> f64 {
        if let Some(alpha) = &self.alpha {
            let u = (u * self.scale).fract() % 1.0;
            let v = 1.0 - (v * self.scale).fract() % 1.0;

            let i = (u * self.width as f64) as u32;
            let j = (v * self.height as f64) as u32;

            let alpha_pixel = alpha
                .get_pixel(i.clamp(0, self.width - 1), j.clamp(0, self.height - 1))
                .to_rgba();
            let alpha_pixel = alpha_pixel[3] as f64 / 255.0;

            return alpha_pixel;
        }

        0.0
    }

    fn normal_value(&self, u: f64, v: f64, _p: &Vec3) -> Vec3 {
        normal_sample(u, v, self.normal_map.as_ref(), self.normal_scale)
    }
}

fn normal_sample(
    u: f64,
    v: f64,
    normal_map: Option<&DynamicImage>,
    normal_scale: Option<f64>,
) -> Vec3 {
    let normal_map = match normal_map {
        Some(map) => map,
        None => return Vec3::new(0.0, 0.0, 0.0),
    };

    let (width, height) = normal_map.dimensions();

    let scale = normal_scale.unwrap_or(1.0);

    let u = (u * scale).fract() % 1.0;
    let v = 1.0 - (v * scale).fract() % 1.0;

    let i = (u * width as f64) as u32;
    let j = (v * height as f64) as u32;

    let pixel = normal_map
        .get_pixel(i.clamp(0, width - 1), j.clamp(0, height - 1))
        .to_rgb();

    let x = (pixel[0] as f64 / 255.0) * 2.0 - 1.0;
    let y = (pixel[1] as f64 / 255.0) * 2.0 - 1.0;
    let z = (pixel[2] as f64 / 255.0) * 2.0 - 1.0;

    Vec3::new(x, y, z)
}
