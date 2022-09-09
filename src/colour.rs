use rand::Rng;
use std::ops;

#[derive(Debug)]
pub struct Colour {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Colour {
    pub fn new(r: f64, g: f64, b: f64) -> Colour {
        Colour { r, g, b }
    }

    pub fn copy(c: &Colour) -> Colour {
        Colour {
            r: c.r,
            g: c.g,
            b: c.b,
        }
    }

    pub fn random() -> Colour {
        Colour {
            r: rand::random::<f64>(),
            g: rand::random::<f64>(),
            b: rand::random::<f64>(),
        }
    }

    pub fn random_min_max(min: f64, max: f64) -> Colour {
        Colour {
            r: rand::thread_rng().gen_range(min..max),
            g: rand::thread_rng().gen_range(min..max),
            b: rand::thread_rng().gen_range(min..max),
        }
    }

    pub fn write_colour(&self, samples_per_pixel: u32) {
        let mut r = self.r;
        let mut g = self.g;
        let mut b = self.b;

        let scale = 1.0 / samples_per_pixel as f64;
        r = (scale * r).sqrt();
        g = (scale * g).sqrt();
        b = (scale * b).sqrt();

        let ir = (256.0 * r.clamp(0.0, 0.999)) as u8;
        let ig = (256.0 * g.clamp(0.0, 0.999)) as u8;
        let ib = (256.0 * b.clamp(0.0, 0.999)) as u8;

        println!("{} {} {}", ir, ig, ib);
    }
}

impl ops::Add<Colour> for Colour {
    type Output = Colour;

    fn add(self, rhs: Colour) -> Self::Output {
        Colour::new(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b)
    }
}

impl ops::AddAssign for Colour {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl ops::Mul<Colour> for f64 {
    type Output = Colour;

    fn mul(self, rhs: Colour) -> Self::Output {
        Colour::new(self * rhs.r, self * rhs.g, self * rhs.b)
    }
}

impl ops::Mul<&Colour> for f64 {
    type Output = Colour;

    fn mul(self, rhs: &Colour) -> Self::Output {
        Colour::new(self * rhs.r, self * rhs.g, self * rhs.b)
    }
}

impl ops::Mul<Colour> for Colour {
    type Output = Colour;

    fn mul(self, rhs: Colour) -> Self::Output {
        Colour::new(self.r * rhs.r, self.g * rhs.g, self.b * rhs.b)
    }
}

impl ops::Mul<&Colour> for Colour {
    type Output = Colour;

    fn mul(self, rhs: &Colour) -> Self::Output {
        Colour::new(self.r * rhs.r, self.g * rhs.g, self.b * rhs.b)
    }
}
impl ops::Div<f64> for Colour {
    type Output = Colour;

    fn div(self, rhs: f64) -> Self::Output {
        Colour::new(self.r / rhs, self.g / rhs, self.b / rhs)
    }
}
