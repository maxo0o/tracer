mod aabb;
mod bvh;
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

use self::colour::Colour;
use self::rectangle::Cube;
use self::vector::Vec3;

fn main() {
    let mut scene = Scene::new("car.json".to_string());

    let cube = Cube::new(
        Vec3::new(-0.6927120089530945, 0.2732959985733032, 1.1210089921951294),
        Vec3::new(0.6927120089530945, 0.44357600808143616, 1.2457760572433472),
        Colour::new(0.5, 0.5, 0.5),
    );
    //    scene.objects.objects.push(Box::new(cube));

    scene.render();
}
