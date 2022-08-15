mod aabb;
mod bvh;
mod camera;
mod colour;
mod hittable;
mod instance;
mod json;
mod kdtree;
mod kdtree_bounds;
mod material;
mod object;
mod onb;
mod pdf;
mod ray;
mod rectangle;
mod scene;
mod sphere;
mod texture;
mod utils;
mod vector;
mod volume;

use crate::scene::Scene;

use self::colour::Colour;
use self::rectangle::Cube;
use self::vector::Vec3;

fn main() {
    let mut scene = Scene::new("car.json".to_string());

    scene.render();
}
