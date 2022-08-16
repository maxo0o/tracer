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

    //    let cube = Cube::new(
    //       Vec3::new(-0.323014996945858, 0.1250230073928833, -0.6372140049934387),
    //     Vec3::new(0.3378880023956299, 1.180912971496582, 0.3554600030183792),
    //   Colour::new(0.5, 0.5, 0.5),
    //);
    //scene.objects.objects.push(Box::new(cube));

    scene.render();
}
