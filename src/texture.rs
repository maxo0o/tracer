use crate::colour::Colour;
use crate::vector::Vec3;

use image::{DynamicImage, GenericImageView, Pixel};

pub trait Texture: std::fmt::Debug + Send + Sync {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Colour;
}

#[derive(Debug)]
pub struct SolidColour {
    colour: Colour,
}

impl SolidColour {
    pub fn new(colour: Colour) -> SolidColour {
        SolidColour { colour }
    }
}

impl Texture for SolidColour {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Colour {
        Colour::copy(&self.colour)
    }
}

#[derive(Debug)]
pub struct CheckerTexture {
    pub odd: Box<dyn Texture>,
    pub even: Box<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(even: Box<dyn Texture>, odd: Box<dyn Texture>) -> CheckerTexture {
        CheckerTexture { even, odd }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Colour {
        let sines = (250.0 * u).sin() * (250.0 * v).sin();
        if sines < 0.0 {
            return self.odd.value(u, v, p);
        } else {
            return self.even.value(u, v, p);
        }
    }
}

#[derive(Debug)]
pub struct ImageTexture {
    image: DynamicImage,
    width: u32,
    height: u32,
    is_light: bool,
}

impl ImageTexture {
    pub fn new(image: DynamicImage, is_light: bool) -> ImageTexture {
        let (width, height) = image.dimensions();
        ImageTexture {
            image,
            width,
            height,
            is_light,
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Colour {
        let u = u % 1.0;
        let v = 1.0 - v % 1.0;

        let i = (u * self.width as f64) as u32;
        let j = (v * self.height as f64) as u32;

        let pixel = self
            .image
            .get_pixel(i.clamp(0, self.width - 1), j.clamp(0, self.height - 1))
            .to_rgb();

        let mut r = pixel[0] as f64 / 256.0;
        let mut g = pixel[1] as f64 / 256.0;
        let mut b = pixel[2] as f64 / 256.0;

        if self.is_light {
            r = r.powf(2.0);
            g = g.powf(2.0);
            b = b.powf(2.0);
        }

        Colour::new(r, g, b)
    }
}
