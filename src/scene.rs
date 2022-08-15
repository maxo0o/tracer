use std::io::BufReader;
use std::sync::{Arc, Mutex};
use std::{fs::File, io::Read};

use image::DynamicImage;
use obj::{load_obj, Obj, TexturedVertex};
use rayon::prelude::*;

use crate::bvh::BoundingVolumeHierarchy;
use crate::camera::Camera;
use crate::colour::Colour;
use crate::hittable::{Hittable, HittableList};
use crate::json::*;
use crate::kdtree_bounds::KDTreeBounds;
use crate::material::{Dialectric, Isotropic, Lambertian, Light, Material, Metal};
use crate::object::Object;
use crate::pdf::CosinePDF;
use crate::pdf::{HittablePDF, MixturePDF, ProbabilityDensityFunction};
use crate::ray::Ray;
use crate::rectangle::Cube;
use crate::sphere::Sphere;
use crate::texture::{ImageTexture, SolidColour, Texture};
use crate::vector::Vec3;
use crate::volume::Volume;

const INFINITY: f64 = f64::INFINITY;
const MAX_RAY_DEPTH: u32 = 100;

pub struct Scene {
    pub camera: Camera,
    pub objects: HittableList,
    pub lights: Vec<Arc<Box<dyn Hittable>>>,
    pub skybox: Option<Sphere>,
    pub render_settings: RenderSettings,
}

pub struct RenderSettings {
    pub samples: u32,
    pub image_width: u32,
    pub image_height: u32,
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

        let render_settings = RenderSettings {
            image_width: scene.render_settings.image_width,
            image_height: scene.render_settings.image_height,
            samples: scene.render_settings.samples,
        };

        let mut objects = HittableList::new();
        let mut lights: Vec<Arc<Box<dyn Hittable>>> = vec![];

        let mut skybox: Option<Sphere> = None;
        if let Some(sky) = &scene.skybox {
            let skybox_material = parse_texture(&sky.image_texture);
            skybox = Some(Sphere::new(
                Vec3::new(0.0, 0.0, 0.0),
                sky.radius,
                Box::new(Lambertian {
                    albedo: skybox_material,
                }),
            ));
        }

        for model in scene.models {
            match model {
                HittablesJSON::Model { obj_path, material } => {
                    let object = match File::open(&obj_path) {
                        Err(why) => panic!("Error opening obj {} :{}", &obj_path, why),
                        Ok(object) => BufReader::new(object),
                    };

                    let object: Obj<TexturedVertex, u32> = match load_obj(object) {
                        Err(why) => panic!("Could not load model {}: {}", &obj_path, why),
                        Ok(model) => model,
                    };

                    let object_material = parse_material(&material);

                    let object = Object::new(object, object_material);

                    if let MaterialJSON::Light { .. } = material {
                        let light_sampler = Box::new(object.get_light_sampler_sphere());
                        lights.push(Arc::new(light_sampler));
                    }

                    objects.objects.push(Box::new(object));
                }
                HittablesJSON::Volume {
                    box_min,
                    box_max,
                    colour,
                    material,
                    density,
                } => {
                    let colour = Colour::new(colour[0], colour[1], colour[2]);
                    let cube = Cube::new(
                        Vec3::new(box_min[0], box_min[1], box_min[2]),
                        Vec3::new(box_max[0], box_max[1], box_max[2]),
                        colour,
                    );
                    let object_material = parse_material(&material);
                    let mist = Volume::new(Box::new(cube), density, object_material);

                    objects.objects.push(Box::new(mist));
                }
            }
        }

        Scene {
            render_settings,
            camera,
            objects,
            lights,
            skybox,
        }
    }

    pub fn render(&self) {
        let zbuffer = Arc::new(Mutex::new(vec![
            vec![
                INFINITY;
                self.render_settings.image_width
                    as usize
            ];
            self.render_settings.image_height as usize
        ]));

        // Render
        print!(
            "P3\n{} {}\n255\n",
            self.render_settings.image_width, self.render_settings.image_height
        );

        for j in (0..=self.render_settings.image_height - 1).rev() {
            eprint!("\rScanlines remaining: {:?}", j);

            let scanline: Vec<Colour> = (0..self.render_settings.image_width)
                .into_par_iter()
                .map(|i| {
                    let mut pixel_colour = Colour::new(0.0, 0.0, 0.0);
                    for _ in 0..self.render_settings.samples {
                        let u = (i as f64 + rand::random::<f64>())
                            / (self.render_settings.image_width - 1) as f64;
                        let v = (j as f64 + rand::random::<f64>())
                            / (self.render_settings.image_height - 1) as f64;

                        let pixel = Some((j as usize, i as usize));
                        let ray = self.camera.get_ray(u, v);
                        pixel_colour +=
                            self.ray_colour(&ray, MAX_RAY_DEPTH, pixel, Arc::clone(&zbuffer));

                        let mut zbuff = zbuffer.lock().unwrap();
                        zbuff[j as usize][i as usize] = INFINITY;
                    }
                    pixel_colour
                })
                .collect();

            for pixel_colour in scanline {
                pixel_colour.write_colour(self.render_settings.samples);
            }
        }
    }

    fn ray_colour(
        &self,
        ray: &Ray,
        depth: u32,
        pixel: Option<(usize, usize)>,
        zbuffer: Arc<Mutex<Vec<Vec<f64>>>>,
    ) -> Colour {
        if depth == 0 {
            return Colour::new(0.0, 0.0, 0.0);
        }

        let mut pixel_tup: Option<(usize, usize)> = None;
        if depth == MAX_RAY_DEPTH {
            pixel_tup = pixel;
        }

        if let Some(hit_record) = &self.objects.hit(
            ray,
            &self.camera,
            0.001,
            INFINITY,
            pixel_tup,
            Arc::clone(&zbuffer),
        ) {
            let (scattered_ray, albedo, is_scattered) =
                hit_record.material.scatter(ray, hit_record);

            let emitted = hit_record
                .material
                .emitted(hit_record.u, hit_record.v, &hit_record.p);

            if !is_scattered {
                return emitted;
            }

            let mut pdf = 1.0;
            let mut scattered_ray = scattered_ray;
            // this only works for lambertian materials rn
            if hit_record.material.use_pdfs() {
                let mut pdfs: Vec<Box<dyn ProbabilityDensityFunction>> = vec![];
                for light in &self.lights {
                    let light_pdf = Box::new(HittablePDF::new(&hit_record.p, Arc::clone(light)));
                    let light_pdf = light_pdf as Box<dyn ProbabilityDensityFunction>;
                    pdfs.push(light_pdf);
                }

                let cos_pdf = Box::new(CosinePDF::new(&hit_record.normal));
                let cos_pdf = cos_pdf as Box<dyn ProbabilityDensityFunction>;
                pdfs.push(cos_pdf);

                let mixture_pdf = MixturePDF::new(pdfs);

                if let Some(ray) = mixture_pdf.generate() {
                    scattered_ray = Ray::new(hit_record.p, ray);
                }

                pdf = mixture_pdf.value(
                    &scattered_ray.direction,
                    &self.camera,
                    pixel,
                    Arc::clone(&zbuffer),
                );
            }

            return emitted
                + hit_record
                    .material
                    .scattering_pdf(ray, hit_record, &scattered_ray)
                    * albedo
                    * self.ray_colour(&scattered_ray, depth - 1, pixel, Arc::clone(&zbuffer))
                    / pdf;
        }

        if let Some(skybox) = &self.skybox {
            if let Some(hit) = &skybox.hit(
                ray,
                &self.camera,
                0.0001,
                INFINITY,
                pixel,
                Arc::clone(&zbuffer),
            ) {
                let (_, albedo, _) = hit.material.scatter(ray, hit);
                return albedo;
            };
        }

        // If not skybox is specified return this default gradient
        let direction = ray.direction.unit();
        let t = 0.5 * (direction.y + 1.0);

        (1.0 - t) * Colour::new(1.0, 1.0, 1.0) + t * Colour::new(0.5, 0.7, 1.0)
    }
}

fn parse_material(material: &MaterialJSON) -> Box<dyn Material + Send + Sync + 'static> {
    match material {
        MaterialJSON::Metal { albedo, f } => {
            let texture = parse_texture(albedo);
            Box::new(Metal {
                albedo: texture,
                f: *f,
            })
        }
        MaterialJSON::Light { albedo, intensity } => {
            let texture = parse_texture(albedo);
            Box::new(Light {
                albedo: texture,
                intensity: *intensity,
            })
        }
        MaterialJSON::Isotropic { albedo } => {
            let texture = parse_texture(albedo);
            Box::new(Isotropic { albedo: texture })
        }
        MaterialJSON::Lambertian { albedo } => {
            let texture = parse_texture(albedo);
            Box::new(Lambertian { albedo: texture })
        }
        MaterialJSON::Dialectric {
            index_of_refraction,
        } => Box::new(Dialectric {
            index_of_refraction: *index_of_refraction,
        }),
    }
}

fn parse_texture(texture: &TextureJSON) -> Box<dyn Texture + Send + Sync + 'static> {
    match texture {
        TextureJSON::ImageTexture {
            image_path,
            alpha_path,
            normal_path,
            normal_scale,
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

            let mut alpha_img: Option<DynamicImage> = None;
            if let Some(alpha) = alpha_path {
                alpha_img = match image::open(&alpha) {
                    Err(why) => panic!("Error opening skybox image texture {}: {}", &alpha, why),
                    Ok(image) => Some(image),
                };
            }

            let mut normal_img: Option<DynamicImage> = None;
            if let Some(normal) = normal_path {
                normal_img = match image::open(&normal) {
                    Err(why) => panic!("Error opening skybox image texture {}: {}", &normal, why),
                    Ok(image) => Some(image),
                };
            }

            Box::new(ImageTexture::new(
                img,
                alpha_img,
                normal_img,
                *normal_scale,
                *is_light,
                *scale,
            ))
        }
        TextureJSON::SolidColour {
            colour,
            normal_path,
            normal_scale,
        } => {
            let mut normal_img: Option<DynamicImage> = None;
            if let Some(normal) = normal_path {
                normal_img = match image::open(&normal) {
                    Err(why) => panic!("Error opening skybox image texture {}: {}", &normal, why),
                    Ok(image) => Some(image),
                };
            }

            Box::new(SolidColour::new(
                Colour::new(colour[0], colour[1], colour[2]),
                normal_img,
                *normal_scale,
            ))
        }
    }
}
