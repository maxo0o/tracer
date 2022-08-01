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

use crate::pdf::{HittablePDF, MixturePDF, ProbabilityDensityFunction};
use camera::Camera;
use colour::Colour;
use hittable::{Hittable, HittableList};
use instance::Translate;
use material::{Isotropic, Lambertian, Light, Metal};
use obj::{load_obj, Obj, TexturedVertex};
use object::Object;
use pdf::CosinePDF;
use ray::Ray;
use rayon::prelude::*;
use sphere::Sphere;
use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, Mutex};
use texture::{ImageTexture, SolidColour};
use vector::Vec3;
use volume::Volume;

use crate::rectangle::Cube;

const INFINITY: f64 = f64::INFINITY;
const MAX_RAY_DEPTH: u32 = 100;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Image
    const ASPECT_RATIO: f64 = 3.0 / 2.0;
    const IMAGE_WIDTH: u32 = 2000;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
    let samples_per_pixel = 25;

    let zbuffer = Arc::new(Mutex::new(vec![
        vec![INFINITY; IMAGE_WIDTH as usize];
        IMAGE_HEIGHT as usize
    ]));

    // World
    let mut world = random_scene();
    let mut lights: Vec<Arc<Box<dyn Hittable>>> = vec![];

    // MUSHROOM ORB 1
    let mushroom_orb = BufReader::new(File::open("/mnt/c/Users/maxmc/Desktop/mushroom_orb1.obj")?);
    let model: Obj<TexturedVertex, u32> = load_obj(mushroom_orb)?;
    let img = image::open("/mnt/c/Users/maxmc/Desktop/mushroom.png").unwrap();
    let image_texture = ImageTexture::new(img, true, 1.0);
    let orb_material = Light {
        albedo: Box::new(image_texture),
        intensity: 3.0,
    };
    let object_orb = Object::new(model, Box::new(orb_material));
    eprintln!("Finished KDTree load");
    let sampler = Box::new(object_orb.get_light_sampler_sphere());
    lights.push(Arc::new(sampler));
    world.objects.push(Box::new(object_orb));

    // MUSHROOM ORB 2
    let mushroom_orb = BufReader::new(File::open("/mnt/c/Users/maxmc/Desktop/mushroom_orb2.obj")?);
    let model: Obj<TexturedVertex, u32> = load_obj(mushroom_orb)?;
    let img = image::open("/mnt/c/Users/maxmc/Desktop/mushroom.png").unwrap();
    let image_texture = ImageTexture::new(img, true, 1.0);
    let orb_material = Light {
        albedo: Box::new(image_texture),
        intensity: 3.0,
    };
    let object_orb = Object::new(model, Box::new(orb_material));
    eprintln!("Finished KDTree load");
    let sampler = Box::new(object_orb.get_light_sampler_sphere());
    lights.push(Arc::new(sampler));
    world.objects.push(Box::new(object_orb));

    // MUSHROOM ORB 3
    let mushroom_orb = BufReader::new(File::open("/mnt/c/Users/maxmc/Desktop/mushroom_orb3.obj")?);
    let model: Obj<TexturedVertex, u32> = load_obj(mushroom_orb)?;
    let img = image::open("/mnt/c/Users/maxmc/Desktop/mushroom.png").unwrap();
    let image_texture = ImageTexture::new(img, true, 1.0);
    let orb_material = Light {
        albedo: Box::new(image_texture),
        intensity: 3.0,
    };
    let object_orb = Object::new(model, Box::new(orb_material));
    eprintln!("Finished KDTree load");
    let sampler = Box::new(object_orb.get_light_sampler_sphere());
    lights.push(Arc::new(sampler));
    world.objects.push(Box::new(object_orb));

    // MUSHROOM ORB 4
    let mushroom_orb = BufReader::new(File::open("/mnt/c/Users/maxmc/Desktop/mushroom_orb4.obj")?);
    let model: Obj<TexturedVertex, u32> = load_obj(mushroom_orb)?;
    let img = image::open("/mnt/c/Users/maxmc/Desktop/mushroom.png").unwrap();
    let image_texture = ImageTexture::new(img, true, 1.0);
    let orb_material = Light {
        albedo: Box::new(image_texture),
        intensity: 3.0,
    };
    let object_orb = Object::new(model, Box::new(orb_material));
    eprintln!("Finished KDTree load");
    let sampler = Box::new(object_orb.get_light_sampler_sphere());
    lights.push(Arc::new(sampler));
    world.objects.push(Box::new(object_orb));

    // MUSHROOM ORB 5
    let mushroom_orb = BufReader::new(File::open("/mnt/c/Users/maxmc/Desktop/mushroom_orb5.obj")?);
    let model: Obj<TexturedVertex, u32> = load_obj(mushroom_orb)?;
    let img = image::open("/mnt/c/Users/maxmc/Desktop/mushroom.png").unwrap();
    let image_texture = ImageTexture::new(img, true, 1.0);
    let orb_material = Light {
        albedo: Box::new(image_texture),
        intensity: 3.0,
    };
    let object_orb = Object::new(model, Box::new(orb_material));
    eprintln!("Finished KDTree load");
    let sampler = Box::new(object_orb.get_light_sampler_sphere());
    lights.push(Arc::new(sampler));
    world.objects.push(Box::new(object_orb));

    let input = BufReader::new(File::open("/mnt/c/Users/maxmc/Desktop/mushroom_d.obj")?);
    let model: Obj<TexturedVertex, u32> = load_obj(input)?;
    let img = image::open("/mnt/c/Users/maxmc/Desktop/mushroom.png").unwrap();
    let image_texture = ImageTexture::new(img, false, 1.0);
    let image_material = Lambertian {
        albedo: Box::new(image_texture),
    };
    eprintln!("Started KDTree load");
    let object = Object::new(model, Box::new(image_material));
    // let object = Translate::new(Box::new(object), Vec3::new(0.0, 3.0, 0.0));
    eprintln!("Finished KDTree load");
    world.objects.push(Box::new(object));

    let river = BufReader::new(File::open("/mnt/c/Users/maxmc/Desktop/river_flat.obj")?);
    let model: Obj<TexturedVertex, u32> = load_obj(river)?;
    let _obj_material = Metal {
        albedo: Box::new(SolidColour::new(Colour::new(0.65, 0.65, 0.65))),
        // f: 0.001,
        f: 0.0,
    };
    let river_obj = Object::new(model, Box::new(_obj_material));
    let river_obj = Translate::new(Box::new(river_obj), Vec3::new(0.0, -1.7, 0.0));
    eprintln!("Finished KDTree load");
    world.objects.push(Box::new(river_obj));

    // let sphere_mat = Light {
    //     albedo: Box::new(SolidColour::new(Colour::new(2.0, 2.0, 2.0))),
    //     intensity: 1.0,
    // };
    // let sphere = Sphere::new(Vec3::new(0.0, 2., 0.0), 0.5, sphere_mat);
    // world.objects.push(Box::new(sphere));

    // let sphere_mat = Light {
    //     albedo: Box::new(SolidColour::new(Colour::new(4.0, 4.0, 4.0))),
    //     intensity: 5.0,
    // };
    // let sphere = Sphere::new(Vec3::new(0.0, 2., 0.0), 0.5, sphere_mat);
    // lights.push(Arc::new(Box::new(sphere)));

    // COOB
    // let mushroom_orb = BufReader::new(File::open("/mnt/c/Users/maxmc/Desktop/cube.obj")?);
    // let model: Obj<TexturedVertex, u32> = load_obj(mushroom_orb)?;
    // let text = SolidColour::new(Colour::new(0.5, 0.5, 0.5));
    // let orb_material = Lambertian {
    //     albedo: Box::new(text),
    // };
    // let object_orb = Object::new(model, orb_material);
    // eprintln!("Finished KDTree load");
    // world.objects.push(Box::new(object_orb));

    //let sphere_mat = Lambertian {
    //    albedo: Box::new(SolidColour::new(Colour::new(0.5, 0.5, 0.5))),
    //};
    //let sphere = Sphere::new(Vec3::new(0.0, 6., 0.0), 6.0, sphere_mat);
    //world.objects.push(Box::new(sphere));

    // let sphere_mat = Lambertian {
    //     albedo: Box::new(SolidColour::new(Colour::new(0.5, 0.5, 0.5))),
    // };
    // let sphere = Sphere::new(Vec3::new(-0.9934191703796387, 12.072142601013184 + 3.76 , -0.059713006019592285), 3.76, sphere_mat);
    // world.objects.push(Box::new(sphere));

    // solo ordbo
    let land = BufReader::new(File::open("/mnt/c/Users/maxmc/Desktop/land_front.obj")?);
    let model: Obj<TexturedVertex, u32> = load_obj(land)?;
    let img = image::open("/mnt/c/Users/maxmc/Desktop/dirt.png").unwrap();
    let image_texture = ImageTexture::new(img, false, 20.0);
    let _ground_material = Lambertian {
        albedo: Box::new(image_texture),
    };
    let object_orb = Object::new(model, Box::new(_ground_material));
    eprintln!("Finished KDTree load");
    //let sampler = Box::new(object_orb.get_light_sampler_sphere());
    //lights.push(Arc::new(sampler));
    world.objects.push(Box::new(object_orb));

    let land = BufReader::new(File::open("/mnt/c/Users/maxmc/Desktop/land_second.obj")?);
    let model: Obj<TexturedVertex, u32> = load_obj(land)?;
    let img = image::open("/mnt/c/Users/maxmc/Desktop/dirt.png").unwrap();
    let image_texture = ImageTexture::new(img, false, 20.0);
    let _ground_material = Lambertian {
        albedo: Box::new(image_texture),
    };
    let object_orb = Object::new(model, Box::new(_ground_material));
    eprintln!("Finished KDTree load");
    //let sampler = Box::new(object_orb.get_light_sampler_sphere());
    //lights.push(Arc::new(sampler));
    world.objects.push(Box::new(object_orb));

    let land = BufReader::new(File::open("/mnt/c/Users/maxmc/Desktop/land_third.obj")?);
    let model: Obj<TexturedVertex, u32> = load_obj(land)?;
    let img = image::open("/mnt/c/Users/maxmc/Desktop/dirt.png").unwrap();
    let image_texture = ImageTexture::new(img, false, 20.0);
    let _ground_material = Lambertian {
        albedo: Box::new(image_texture),
    };
    let object_orb = Object::new(model, Box::new(_ground_material));
    eprintln!("Finished KDTree load");
    //let sampler = Box::new(object_orb.get_light_sampler_sphere());
    //lights.push(Arc::new(sampler));
    world.objects.push(Box::new(object_orb));

    let land = BufReader::new(File::open("/mnt/c/Users/maxmc/Desktop/land_fourth.obj")?);
    let model: Obj<TexturedVertex, u32> = load_obj(land)?;
    let img = image::open("/mnt/c/Users/maxmc/Desktop/dirt.png").unwrap();
    let image_texture = ImageTexture::new(img, false, 100.0);
    let _ground_material = Lambertian {
        albedo: Box::new(image_texture),
    };
    let object_orb = Object::new(model, Box::new(_ground_material));
    eprintln!("Finished KDTree load");
    //let sampler = Box::new(object_orb.get_light_sampler_sphere());
    //lights.push(Arc::new(sampler));
    world.objects.push(Box::new(object_orb));

    let input = BufReader::new(File::open("/mnt/c/Users/maxmc/Desktop/mushroomzzz.obj")?);
    let model: Obj<TexturedVertex, u32> = load_obj(input)?;
    let img = image::open("/mnt/c/Users/maxmc/Desktop/mushroom.png").unwrap();
    let image_texture = ImageTexture::new(img, false, 1.0);
    let image_material = Lambertian {
        albedo: Box::new(image_texture),
    };
    eprintln!("Started KDTree load");
    let object = Object::new(model, Box::new(image_material));
    let object = Translate::new(Box::new(object), Vec3::new(0.0, 3.0, 0.0));
    eprintln!("Finished KDTree load");
    world.objects.push(Box::new(object));

    let input = BufReader::new(File::open("/mnt/c/Users/maxmc/Desktop/mushroomt1.obj")?);
    let model: Obj<TexturedVertex, u32> = load_obj(input)?;
    let img = image::open("/mnt/c/Users/maxmc/Desktop/mushroomt1.png").unwrap();
    let image_texture = ImageTexture::new(img, false, 1.0);
    let image_material = Lambertian {
        albedo: Box::new(image_texture),
    };
    eprintln!("Started KDTree load");
    let object = Object::new(model, Box::new(image_material));
    let object = Translate::new(Box::new(object), Vec3::new(0.0, 3.0, 0.0));
    eprintln!("Finished KDTree load");
    world.objects.push(Box::new(object));

    let input = BufReader::new(File::open("/mnt/c/Users/maxmc/Desktop/mushroomt2.obj")?);
    let model: Obj<TexturedVertex, u32> = load_obj(input)?;
    let img = image::open("/mnt/c/Users/maxmc/Desktop/mushroomt1.png").unwrap();
    let image_texture = ImageTexture::new(img, false, 1.0);
    let image_material = Lambertian {
        albedo: Box::new(image_texture),
    };
    eprintln!("Started KDTree load");
    let object = Object::new(model, Box::new(image_material));
    let object = Translate::new(Box::new(object), Vec3::new(0.0, 3.0, 0.0));
    eprintln!("Finished KDTree load");
    world.objects.push(Box::new(object));

    let input = BufReader::new(File::open("/mnt/c/Users/maxmc/Desktop/mushroomt3.obj")?);
    let model: Obj<TexturedVertex, u32> = load_obj(input)?;
    let img = image::open("/mnt/c/Users/maxmc/Desktop/mushroomt1.png").unwrap();
    let image_texture = ImageTexture::new(img, false, 1.0);
    let image_material = Lambertian {
        albedo: Box::new(image_texture),
    };
    eprintln!("Started KDTree load");
    let object = Object::new(model, Box::new(image_material));
    let object = Translate::new(Box::new(object), Vec3::new(0.0, 3.0, 0.0));
    eprintln!("Finished KDTree load");
    world.objects.push(Box::new(object));

    //   let img = image::open("/mnt/c/Users/maxmc/Desktop/earthmap.jpg").unwrap();
    //   let image_texture = ImageTexture::new(img, true, 1.0);
    //   let image_material = Lambertian {
    //       albedo: Box::new(image_texture),
    //   };

    //let s = Sphere::new(Vec3::new(0.0, 0.0, 0.0), 50.0, image_material);
    //world.objects.push(Box::new(s));

    let img = image::open("/mnt/c/Users/maxmc/Desktop/sky.jpg").unwrap();
    let image_texture = ImageTexture::new(img, true, 8.0);
    let image_material = Lambertian {
        albedo: Box::new(image_texture),
    };
    let skybox = Sphere::new(Vec3::new(0.0, 0.0, 0.0), 4000.0, Box::new(image_material));
    let skybox = Arc::new(skybox);

    let colour = Colour::new(0.1, 0.1, 0.1);
    let cube = Cube::new(
        Vec3::new(-4000.0, -20.0, -4000.0),
        Vec3::new(4000.0, 150.0, 4000.0),
        colour,
    );
    let material = Isotropic {
        albedo: Box::new(SolidColour::new(Colour::new(0.8, 0.8, 0.8))),
    };
    let mist = Volume::new(Box::new(cube), 0.000215, Box::new(material));
    world.objects.push(Box::new(mist));

    // Camera
    let look_from = Vec3::new(295.0, 20.0, 205.0);
    let look_at = Vec3::new(-35.0, -5.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 1.0;
    let aperture = 0.0;

    let cam = Camera::new(
        look_from,
        look_at,
        vup,
        20.0,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
    );

    // Render
    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..=IMAGE_HEIGHT - 1).rev() {
        eprint!("\rScanlines remaining: {:?}", j);

        let scanline: Vec<Colour> = (0..IMAGE_WIDTH)
            .into_par_iter()
            .map(|i| {
                let mut pixel_colour = Colour::new(0.0, 0.0, 0.0);
                for _ in 0..samples_per_pixel {
                    let u = (i as f64 + rand::random::<f64>()) / (IMAGE_WIDTH - 1) as f64;
                    let v = (j as f64 + rand::random::<f64>()) / (IMAGE_HEIGHT - 1) as f64;

                    let pixel = Some((j as usize, i as usize));
                    let ray = cam.get_ray(u, v);
                    pixel_colour += ray_colour(
                        &ray,
                        &cam,
                        &world,
                        MAX_RAY_DEPTH,
                        pixel,
                        Arc::clone(&zbuffer),
                        &lights,
                        Arc::clone(&skybox),
                    );

                    let mut zbuff = zbuffer.lock().unwrap();
                    zbuff[j as usize][i as usize] = INFINITY;
                }
                pixel_colour
            })
            .collect();

        for pixel_colour in scanline {
            pixel_colour.write_colour(samples_per_pixel);
        }
    }

    eprintln!("\nDone!");
    Ok(())
}

fn ray_colour(
    ray: &Ray,
    camera: &Camera,
    world: &HittableList,
    depth: u32,
    pixel: Option<(usize, usize)>,
    zbuffer: Arc<Mutex<Vec<Vec<f64>>>>,
    lights: &Vec<Arc<Box<dyn Hittable>>>,
    skybox: Arc<Sphere>,
) -> Colour {
    if depth == 0 {
        return Colour::new(0.0, 0.0, 0.0);
    }

    let mut pixel_tup: Option<(usize, usize)> = None;
    if depth == MAX_RAY_DEPTH {
        pixel_tup = pixel;
    }

    if let Some(hit_record) = world.hit(
        ray,
        camera,
        0.001,
        INFINITY,
        pixel_tup,
        Arc::clone(&zbuffer),
    ) {
        let (scattered_ray, albedo, is_scattered) = hit_record.material.scatter(ray, &hit_record);

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
            for light in lights {
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
                camera,
                pixel,
                Arc::clone(&zbuffer),
            );
        }

        return emitted
            + hit_record
                .material
                .scattering_pdf(ray, &hit_record, &scattered_ray)
                * albedo
                * ray_colour(
                    &scattered_ray,
                    camera,
                    world,
                    depth - 1,
                    pixel,
                    Arc::clone(&zbuffer),
                    lights,
                    Arc::clone(&skybox),
                )
                / pdf;
    }

    //   let direction = ray.direction.unit();
    //   let t = 0.5 * (direction.y + 1.0);

    if let Some(hit) = skybox.hit(ray, camera, 0.0001, INFINITY, pixel, Arc::clone(&zbuffer)) {
        let (_, albedo, _) = hit.material.scatter(ray, &hit);
        return albedo;
    };
    //return (1.0 - t) * Colour::new(70. / 256., 216. / 256., 253. / 256.) + t * Colour::new( 39. / 256., 87. / 256., 185. / 256.);
    // return (1.0 - t) * Colour::new(1.0, 1.0, 1.0) + t * Colour::new(0.5, 0.7, 1.0);
    // return Colour::new(0.0, 0.0, 0.0);
    Colour::new(
        (39. / 256. as f64 - 0.0).powf(2.0),
        (87. / 256. as f64 - 0.0).powf(2.0),
        (185. / 256. as f64 - 0.0).powf(2.0),
    )
}

fn random_scene() -> HittableList {
    let world = HittableList::new();
    let _obj_material = Metal {
        albedo: Box::new(SolidColour::new(Colour::new(0.35, 0.35, 0.45))),
        f: 0.05,
    };
    let _ground_material = Lambertian {
        albedo: Box::new(SolidColour::new(Colour::new(0.7, 0.7, 0.7))),
    };
    //   let solid_text_1 = Box::new(SolidColour::new(Colour::new(0.2, 0.3, 0.1)));
    //   let solid_text_2 = Box::new(SolidColour::new(Colour::new(0.9, 0.9, 0.9)));
    //   let material3 = Lambertian {
    //       albedo: Box::new(CheckerTexture::new(solid_text_1, solid_text_2)),
    //   };
    // world.objects.push(Box::new(Sphere::new(
    //     Vec3::new(0.0, -10002.0, 0.0),
    //     10000.0,
    //     _ground_material,
    // )));

    world
}
