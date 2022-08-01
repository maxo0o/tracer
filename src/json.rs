use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SkyboxJSON {
    pub image_texture: TextureJSON,
    pub radius: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TextureJSON {
    SolidColour {
        colour: [f64; 3]
    },
    ImageTexture {
        image_path: String,
        is_light: bool,
        scale: f64,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub enum MaterialJSON {
    Lambertian {
        albedo: TextureJSON,
    },
    Metal {
        albedo: TextureJSON,
        f: f64,
    },
    Dialectric {
        index_of_refraction: f64,
    },
    Light {
        albedo: TextureJSON,
        intensity: f64,
    },
    Isotropic {
        albedo: TextureJSON,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ModelJSON {
    pub obj_path: String,
    pub material: MaterialJSON,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CameraJSON {
    pub resolution: [u32; 2],
    pub samples: u32,
    pub aspect_ratio: f64,
    pub look_from: [f64; 3],
    pub look_at: [f64; 3],
    pub vfov: f64,
    pub v_up: [f64; 3],
    pub dist_to_focus: f64,
    pub aperture: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SceneJSON {
    pub camera: CameraJSON,
    pub models: Vec<ModelJSON>,
    pub skybox: SkyboxJSON,
}
