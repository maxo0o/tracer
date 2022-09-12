mod aabb;
mod bxdf;
mod camera;
mod colour;
mod hittable;
mod instance;
mod json;
mod kdtree;
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

fn main() {
    let scene = Scene::new("car.json".to_string());

    scene.render();
}
