mod camera;
mod colour;
mod hittable;
mod material;
mod ray;
mod sphere;
mod utils;
mod vector;
mod object;

use camera::Camera;
use colour::Colour;
use hittable::{Hittable, HittableList};
use material::{Dialectric, Lambertian, Metal, Light};
use ray::Ray;
use sphere::Sphere;
use vector::Vec3;
use object::Object;
use std::fs::File;
use std::io::BufReader;
use obj::{load_obj, Obj};
use rayon::prelude::*;
use std::sync::{Arc, Mutex};
use rand::{Rng};

const INFINITY: f64 = f64::INFINITY;
// const PI: f64 = std::f64::consts::PI;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    // Image
    const ASPECT_RATIO: f64  = 3.0 / 2.0;
    const IMAGE_WIDTH: u32 = 2000;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
    let samples_per_pixel = 55;
    let max_depth = 100;
    let zbuffer= Arc::new(Mutex::new(vec![vec![INFINITY; IMAGE_WIDTH as usize]; IMAGE_HEIGHT as usize]));

    // World
    let mut world = random_scene();

    let input = BufReader::new(File::open("/Users/maxmclaughlin/Desktop/suz2.obj")?);
    let model: Obj = load_obj(input)?;
    let _obj_material = Metal { albedo: Colour::new(0.35, 0.35, 0.45), f: 0.0 };
    let _obj_material_glass = Dialectric { index_of_refraction: 1.5 };
    let _obj_diffuse = Lambertian { albedo: Colour::new(0.35, 0.35, 0.35) };
    let object = Object::new(model, _obj_diffuse);

    //let mut world = HittableList::new();
    world.objects.push(Box::new(object));
    // let ground_material = Lambertian { albedo: Colour::new(0.5, 0.5, 0.5) };
    // world.objects.push(Box::new(Sphere::new(
    //     Vec3::new(0.0, -1000.839506, 0.0),
    //     1000.0,
    //     ground_material,
    // )));
    let light_material = Light { intensity: 30.0, colour: Colour::new(180.0 / 255.0, 162.0 / 255.0, 252.0 / 255.0) };
    world.objects.push(Box::new(Sphere::new(
        Vec3::new(2.0, 2.8, 0.0),
        0.3,
        light_material,
    )));

    // Camera
    let look_from = Vec3::new(8.0, 2.0, 2.0);
    let look_at = Vec3::new(0.0, 1.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 1.0;
    let aperture = 0.0;

    let cam = Camera::new(
        &look_from,
        &look_at,
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

        let scanline: Vec<Colour> = (0..IMAGE_WIDTH).into_par_iter().map(|i| {
            let mut pixel_colour = Colour::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + rand::random::<f64>()) / (IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + rand::random::<f64>()) / (IMAGE_HEIGHT - 1) as f64;

                let ray = cam.get_ray(u, v);
                pixel_colour += ray_colour(&ray, &world, max_depth, j, i, Arc::clone(&zbuffer));
            }
            pixel_colour
        }).collect();

        for pixel_colour in scanline {
            pixel_colour.write_colour(samples_per_pixel);
        }
    }

    eprintln!("\nDone!");
    Ok(())
}

fn ray_colour(ray: &Ray, world: &HittableList, depth: i32, p_0: u32, p_1: u32, zbuffer: Arc<Mutex<Vec<Vec<f64>>>>) -> Colour {
    if depth <= 0 {
        return Colour::new(0.0, 0.0, 0.0);
    }

    if let Some(hit_record) = world.hit(ray, 0.001, INFINITY, p_0, p_1, Arc::clone(&zbuffer)) {
        let (scattered_ray, albedo, is_scattered, is_light) = hit_record.material.scatter(ray, &hit_record);
        if is_scattered && !is_light {
            return albedo * ray_colour(&scattered_ray, world, depth - 1, p_0, p_1, Arc::clone(&zbuffer));
        } else if is_light {
            return albedo;
        }

        return Colour::new(0.0, 0.0, 0.0);
    }
    let direction = ray.direction.unit();

    let t = 0.5 * (direction.y + 1.0);
    //return (1.0 - t) * Colour::new(1.0, 1.0, 1.0) + t * Colour::new(0.5, 0.7, 1.0);
    return Colour::new(0.0, 0.0, 0.0);
}

fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let ground_material = Lambertian { albedo: Colour::new(0.5, 0.5, 0.5) };
    world.objects.push(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    // for a in -5..5 {
    //     for b in -5..5 {
    //         let choose_mat = rand::random::<f64>();
    //         let center = Vec3::new(a as f64 + 0.9 * rand::random::<f64>(), 0.2, b as f64 + 0.9 * rand::random::<f64>());

    //         if (&center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
    //             if choose_mat < 0.8 {
    //                 // DIFFUSE
    //                 let albedo = Colour::random() * Colour::random();
    //                 let sphere_material = Lambertian { albedo };
    //                 world.objects.push(Box::new(Sphere::new(
    //                     Vec3::copy(&center),
    //                     0.2,
    //                     sphere_material,
    //                 )));
    //             } else if choose_mat < 0.95 {
    //                 // METAL
    //                 let albedo = Colour::random_min_max(0.5, 1.0);
    //                 let fuzz = rand::thread_rng().gen_range(0.0..0.5);
    //                 let sphere_material = Metal { albedo, f: fuzz };
    //                 world.objects.push(Box::new(Sphere::new(
    //                     Vec3::copy(&center),
    //                     0.2,
    //                     sphere_material,
    //                 )));
    //             } else {
    //                 // GLASS
    //                 let sphere_material = Dialectric { index_of_refraction: 1.5 };
    //                 world.objects.push(Box::new(Sphere::new(
    //                     Vec3::copy(&center),
    //                     0.2,
    //                     sphere_material,
    //                 )));
    //             }
    //         }
    //     }
    // }

    // let material1 = Dialectric { index_of_refraction: 1.5 };
    // world.objects.push(Box::new(Sphere::new(
    //     Vec3::new(0.0, 1.0, 0.0),
    //     1.0,
    //     material1,
    // )));

    // let material2 = Lambertian { albedo: Colour::new(0.4, 0.2, 0.1) };
    // world.objects.push(Box::new(Sphere::new(
    //     Vec3::new(-4.0, 1.0, 0.0),
    //     1.0,
    //     material2,
    // )));

    // let material3 = Metal { albedo: Colour::new(0.7, 0.6, 0.5), f: 0.0 };
    // world.objects.push(Box::new(Sphere::new(
    //     Vec3::new(4.0, 1.0, 0.0),
    //     1.0,
    //     material3,
    // )));

    world
}