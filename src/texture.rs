use crate::colour::Colour;
use crate::vector::Vec3;


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