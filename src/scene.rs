use std::io::BufReader;
use std::sync::Arc;
use std::{fs::File, io::Read};

use obj::{load_obj, Obj, TexturedVertex};

use crate::camera::Camera;
use crate::colour::Colour;
use crate::hittable::{Hittable, HittableList};
use crate::json::*;
use crate::material::{Lambertian, Material};
use crate::sphere::Sphere;
use crate::texture::{ImageTexture, SolidColour, Texture};
use crate::vector::Vec3;

pub struct Scene {
    camera: Camera,
    objects: HittableList,
    lights: Vec<Arc<Box<dyn Hittable>>>,
    skybox: Sphere<Lambertian>,
}

impl Scene {
    pub fn new(filename: String) -> Scene {
        if filename.is_empty() {
            panic!("Empty scene filename!");
        }

        let mut file = match File::open(&filename) {
            Err(why) => panic!("Error opening {}: {}", &filename, why),
            Ok(file) => file,
        };

        let mut s = String::new();
        if let Err(why) = file.read_to_string(&mut s) {
            panic!("Error converting file contents to string: {}", why);
        };

        let scene: SceneJSON = match serde_json::from_str(&s) {
            Err(why) => panic!("Error parsing json file: {}", why),
            Ok(json) => json,
        };

        let camera = Camera::new(
            Vec3::new_arr(scene.camera.look_from),
            Vec3::new_arr(scene.camera.look_at),
            Vec3::new_arr(scene.camera.v_up),
            scene.camera.vfov,
            scene.camera.aspect_ratio,
            scene.camera.aperture,
            scene.camera.dist_to_focus,
        );

        let mut objects = HittableList::new();
        let mut lights: Vec<Arc<Box<dyn Hittable>>> = vec![];

        let skybox_material = parse_texture(scene.skybox.image_texture);
        let skybox = Sphere::new(
            Vec3::new(0.0, 0.0, 0.0),
            scene.skybox.radius,
            skybox_material,
        );

        for model in scene.models {
            let object = match File::open(&model.obj_path) {
                Err(why) => panic!("Error opening obj {} :{}", &model.obj_path, why),
                Ok(object) => BufReader::new(object),
            };

            let object: Obj<TexturedVertex, u32> = match load_obj(object) {
                Err(why) => panic!("Could not load model {}: {}", model.obj_path, why),
                Ok(model) => model,
            };

            let object_material = match model.material {
                MaterialJSON::Metal { albedo, f } => {}
                MaterialJSON::Light { albedo, intensity } => {}
                MaterialJSON::Isotropic { albedo: () } => {}
                MaterialJSON::Lambertian { albedo } => {}
                MaterialJSON::Dialectric {
                    index_of_refraction: (),
                } => {}
            };
        }

        Scene {
            camera,
            objects,
            lights,
            skybox,
        }
    }
}

fn parse_texture(texture: TextureJSON) -> &'static dyn Material {
    match texture {
        TextureJSON::ImageTexture {
            image_path,
            is_light,
            scale,
        } => {
            let img = match image::open(&image_path) {
                Err(why) => panic!(
                    "Error opening skybox image texture {}: {}",
                    &image_path, why
                ),
                Ok(image) => image,
            };
            let image_texture = ImageTexture::new(img, is_light, scale);
            &Lambertian {
                albedo: Box::new(image_texture),
            }
        }
        TextureJSON::SolidColour { colour } => &Lambertian {
            albedo: Box::new(SolidColour::new(Colour::new(
                colour[0], colour[1], colour[2],
            ))),
        },
    }
}
