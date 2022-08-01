use std::io::BufReader;
use std::sync::Arc;
use std::{fs::File, io::Read};

use obj::{load_obj, Obj, TexturedVertex};

use crate::camera::Camera;
use crate::colour::Colour;
use crate::hittable::{Hittable, HittableList};
use crate::json::*;
use crate::material::{Dialectric, Isotropic, Lambertian, Light, Material, Metal};
use crate::object::Object;
use crate::sphere::Sphere;
use crate::texture::{ImageTexture, SolidColour, Texture};
use crate::vector::Vec3;

pub struct Scene {
    pub camera: Camera,
    pub objects: HittableList,
    pub lights: Vec<Arc<Box<dyn Hittable>>>,
    pub skybox: Sphere,
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
            Box::new(Lambertian {
                albedo: skybox_material,
            }),
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

            let mut is_light = false;
            let object_material = match model.material {
                MaterialJSON::Metal { albedo, f } => {
                    let texture = parse_texture(albedo);
                    Box::new(Metal { albedo: texture, f }) as Box<dyn Material>
                }
                MaterialJSON::Light { albedo, intensity } => {
                    let texture = parse_texture(albedo);
                    is_light = true;
                    Box::new(Light {
                        albedo: texture,
                        intensity,
                    }) as Box<dyn Material>
                }
                MaterialJSON::Isotropic { albedo } => {
                    let texture = parse_texture(albedo);
                    Box::new(Isotropic { albedo: texture }) as Box<dyn Material>
                }
                MaterialJSON::Lambertian { albedo } => {
                    let texture = parse_texture(albedo);
                    Box::new(Lambertian { albedo: texture }) as Box<dyn Material>
                }
                MaterialJSON::Dialectric {
                    index_of_refraction,
                } => Box::new(Dialectric {
                    index_of_refraction,
                }) as Box<dyn Material>,
            };

            let object = Object::new(object, object_material);

            if is_light {
                let light_sampler = Box::new(object.get_light_sampler_sphere());
                lights.push(Arc::new(light_sampler));
            }

            objects.objects.push(Box::new(object));
        }

        Scene {
            camera,
            objects,
            lights,
            skybox,
        }
    }
}

fn parse_texture(texture: TextureJSON) -> Box<dyn Texture + Send + Sync + 'static> {
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
            Box::new(ImageTexture::new(img, is_light, scale))
        }
        TextureJSON::SolidColour { colour } => Box::new(SolidColour::new(Colour::new(
            colour[0], colour[1], colour[2],
        ))),
    }
}
