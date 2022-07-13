mod aabb;
mod bvh;
mod camera;
mod colour;
mod hittable;
mod kdtree;
mod material;
mod object;
mod ray;
mod rectangle;
mod sphere;
mod texture;
mod utils;
mod vector;

use camera::Camera;
use colour::Colour;
use hittable::{Hittable, HittableList};
use material::{Dialectric, Lambertian, Light, Metal};
use obj::{load_obj, Obj, TexturedVertex};
use object::Object;
use std::sync::Arc;
// use rand::Rng;
use bvh::BoundingVolumeHierarchy;
use ray::Ray;
use rayon::prelude::*;
use rectangle::{Plane, PlaneOrientation};
use sphere::Sphere;
use std::fs::File;
use std::io::BufReader;
use texture::{CheckerTexture, ImageTexture, SolidColour};
use vector::Vec3;

const INFINITY: f64 = f64::INFINITY;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Image
    const ASPECT_RATIO: f64 = 3.0 / 2.0;
    // const ASPECT_RATIO: f64 = 1.0;
    const IMAGE_WIDTH: u32 = 3000;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
    let samples_per_pixel = 2;
    let max_depth = 100;

    // World
    let mut world = random_scene();

    let input = BufReader::new(File::open("/Users/maxmclaughlin/Desktop/dragon2_uv.obj")?);
    // let input = BufReader::new(File::open("/Users/maxmclaughlin/Desktop/suz2.obj")?);
    let model: Obj<TexturedVertex, u32> = load_obj(input)?;
    let _obj_material = Metal {
        albedo: Colour::new(0.35, 0.35, 0.45),
        f: 0.0,
    };
    let _obj_material_glass = Dialectric {
        index_of_refraction: 1.5,
    };
    let _obj_diffuse = Lambertian {
        albedo: Box::new(SolidColour::new(Colour::new(0.35, 0.35, 0.35))),
    };
    let solid_text_1 = Box::new(SolidColour::new(Colour::new(0.2, 0.3, 0.1)));
    let solid_text_2 = Box::new(SolidColour::new(Colour::new(0.9, 0.9, 0.9)));
    let material3 = Lambertian {
        albedo: Box::new(CheckerTexture::new(solid_text_1, solid_text_2)),
    };

    let img = image::open("/Users/maxmclaughlin/Desktop/xxx.webp").unwrap();
    let image_texture = ImageTexture::new(img);
    let image_material = Lambertian {
        albedo: Box::new(image_texture),
    };

    eprintln!("Started KDTree load");
    let object = Object::new(model, image_material);
    eprintln!("Finished KDTree load");
    world.objects.push(Box::new(object));

    // let input = BufReader::new(File::open("/Users/maxmclaughlin/Desktop/box.obj")?);
    let _obj_diffuse_box = Lambertian {
        albedo: Box::new(SolidColour::new(Colour::new(0.5, 0.5, 0.2))),
    };
    // let box_model: Obj = load_obj(input)?;
    // let box_object = Object::new(box_model, _obj_diffuse_box);
    // world.objects.push(Box::new(box_object));
    //objects.push(Arc::new(Box::new(box_object)));

    // let light_material = Light {
    //     intensity: 100.0,
    //     albedo: Box::new(SolidColour::new(Colour::new(
    //         (254. / 256.0 as f64).powf(2.0),
    //         (129.0 / 256.0 as f64).powf(2.0),
    //         (76.0 / 256.0 as f64).powf(2.0),
    //     ))),
    // };
    // let light = Box::new(Sphere::new(Vec3::new(2.0, 2.8, 0.0), 0.3, light_material));
    let light_material = Light {
        intensity: 1.0,
        albedo: Box::new(SolidColour::new(Colour::new(4.0, 4.0, 4.0))),
    };
    let light = Box::new(Plane::new(
        (3.0, 5.0, 1.0, 3.0),
        -2.0,
        light_material,
        PlaneOrientation::XZ,
    ));
    // world.objects.push(light);


    let green = Lambertian {
        albedo: Box::new(SolidColour::new(Colour::new(0.12, 0.45, 0.15))),
    };
    let box1 = Box::new(Plane::new(
        (0.0, 555.0, 0.0, 555.0),
        555.0,
        green,
        PlaneOrientation::YZ,
    ));
    world.objects.push(box1);

    let red = Lambertian {
        albedo: Box::new(SolidColour::new(Colour::new(0.65, 0.05, 0.05))),
    };
    let box2 = Box::new(Plane::new(
        (0.0, 555.0, 0.0, 555.0),
        0.0,
        red,
        PlaneOrientation::YZ,
    ));
    // world.objects.push(box2);

    let light = Light {
        intensity: 15.0,
        albedo: Box::new(SolidColour::new(Colour::new(1.0, 1.0, 1.0))),
    };
    let box3 = Box::new(Plane::new(
        (213.0, 343.0, 227.0, 332.0),
        554.0,
        light,
        PlaneOrientation::XZ,
    ));
    world.objects.push(box3);


    let white1 = Lambertian {
        albedo: Box::new(SolidColour::new(Colour::new(0.73, 0.73, 0.73))),
    };
    let box4 = Box::new(Plane::new(
        (0.0, 555.0, 0.0, 555.0),
        0.0,
        white1,
        PlaneOrientation::XZ,
    ));
    world.objects.push(box4);

    let white2 = Lambertian {
        albedo: Box::new(SolidColour::new(Colour::new(0.73, 0.73, 0.73))),
    };
    let box5 = Box::new(Plane::new(
        (0.0, 555.0, 0.0, 555.0),
        555.0,
        white2,
        PlaneOrientation::XZ,
    ));
    world.objects.push(box5);

    let white3 = Lambertian {
        albedo: Box::new(SolidColour::new(Colour::new(0.73, 0.73, 0.73))),
    };
    let box6 = Box::new(Plane::new(
        (0.0, 555.0, 0.0, 555.0),
        555.0,
        white3,
        PlaneOrientation::XY,
    ));
    world.objects.push(box6);

    // let objects = BoundingVolumeHierarchy::build(&mut objects[..], 0);
    // eprintln!("HELLO {:?}", objects);

    // Camera
    let look_from = Vec3::new(8.0, 2.0, 2.0);
    let look_at = Vec3::new(0.0, 1.0, 0.0);
    // let look_from = Vec3::new(26., 3., 6.);
    // let look_at = Vec3::new(0., 2., 0.);
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

    // let look_from = Vec3::new(278., 278., -800.);
    // let look_at = Vec3::new(278., 278., 0.);

    // let cam = Camera::new(
    //     &look_from,
    //     &look_at,
    //     vup,
    //     40.0,
    //     1.0,
    //     aperture,
    //     dist_to_focus,
    // );

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

                    let ray = cam.get_ray(u, v);
                    pixel_colour += ray_colour(&ray, &cam, &world, max_depth);
                    // if world.hit_something(&ray, 0.001, INFINITY) {
                    //    pixel_colour += ray_colour(&ray, &cam, &world, max_depth);
                    // } else {
                    //     pixel_colour += Colour::new(
                    //         (39. / 256. as f64).powf(2.),
                    //         (87. / 256. as f64).powf(2.),
                    //         (185. / 256. as f64).powf(2.),
                    //     )
                    // }
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

fn ray_colour(ray: &Ray, camera: &Camera, world: &HittableList, depth: i32) -> Colour {
    if depth <= 0 {
        return Colour::new(0.0, 0.0, 0.0);
    }

    if let Some(hit_record) = world.hit(ray, camera, 0.001, INFINITY) {
        let (scattered_ray, albedo, is_scattered) = hit_record.material.scatter(ray, &hit_record);

        let emitted = hit_record
            .material
            .emitted(hit_record.u, hit_record.v, &hit_record.p);
        if !is_scattered {
            return emitted;
        }

        return emitted + albedo * ray_colour(&scattered_ray, camera, world, depth - 1);
    }

    let direction = ray.direction.unit();
    let t = 0.5 * (direction.y + 1.0);
    //return (1.0 - t) * Colour::new(70. / 256., 216. / 256., 253. / 256.) + t * Colour::new( 39. / 256., 87. / 256., 185. / 256.);
    return (1.0 - t) * Colour::new(1.0, 1.0, 1.0) + t * Colour::new(0.5, 0.7, 1.0);
    // return Colour::new(0.0, 0.0, 0.0);
    // return Colour::new(
    //     (39. / 256. as f64).powf(2.),
    //     (87. / 256. as f64).powf(2.),
    //     (185. / 256. as f64).powf(2.),
    // );
}

fn random_scene() -> HittableList {
    let mut world = HittableList::new();
    let _obj_material = Metal {
        albedo: Colour::new(0.35, 0.35, 0.45),
        f: 0.05,
    };
    let _ground_material = Lambertian {
        albedo: Box::new(SolidColour::new(Colour::new(0.5, 0.5, 0.5))),
    };
    let solid_text_1 = Box::new(SolidColour::new(Colour::new(0.2, 0.3, 0.1)));
    let solid_text_2 = Box::new(SolidColour::new(Colour::new(0.9, 0.9, 0.9)));
    let material3 = Lambertian {
        albedo: Box::new(CheckerTexture::new(solid_text_1, solid_text_2)),
    };
    // world.objects.push(Box::new(Sphere::new(
    //     Vec3::new(0.0, -1000.0, 0.0),
    //     1000.0,
    //     _ground_material,
    // )));

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

    // let solid_text_1 = Box::new(SolidColour::new(Colour::new(0.2, 0.3, 0.1)));
    // let solid_text_2 = Box::new(SolidColour::new(Colour::new(0.9, 0.9, 0.9)));
    // let material3 = Lambertian { albedo: Box::new(CheckerTexture::new(solid_text_1, solid_text_2))};
    // world.objects.push(Box::new(Sphere::new(
    //     Vec3::new(4.0, 1.0, 0.0),
    //     1.0,
    //     material3,
    // )));

    world
}
