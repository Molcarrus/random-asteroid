use std::default;

use crate::prng::{self, PRNG};

use bevy::{
    math::{ Vec3,FloatExt },
    prelude::{ Resource,default },
    reflect::Reflect,
    render::render_resource::ShaderType
};

use rand::{
    rngs::StdRng,
    SeedableRng
};

use bytemuck::{ Pod,Zeroable };

#[derive(Resource, Default, Debug, Reflect, Clone)]
pub struct SimpleNoiseSettings {
    pub layers: f32,
    pub lacuranity: f32,
    pub persistence: f32,
    pub scale: f32,
    pub elevation: f32,
    pub vertical_shift: f32,
    pub offset_x: f32,
    pub offset_y: f32,
    pub offset_z: f32
}

impl SimpleNoiseSettings {
    pub fn get_noise_args(
        &self, 
        mut prng: PRNG
    ) -> Vec<[f32; 4]> {
        let seeded_offset = Vec3::new(prng.get_value(), prng.get_value(), prng.get_value()) * prng.get_value() * 10000.0;

        vec![[seeded_offset.x + self.offset_x, seeded_offset.y + self.offset_y, seeded_offset.z + self.offset_z, self.layers],
        [self.persistence, self.lacuranity, self.scale, self.elevation],
        [self.vertical_shift, 0.0, 0.0, 0.0]]        
    }
}

#[derive(Resource, Default, Debug, Reflect, Clone)]
pub struct RidgeNoiseSettings {
    pub layers: f32,
    pub lacunarity: f32,
    pub persistence: f32,
    pub scale: f32,
    pub power: f32,
    pub elevation: f32,
    pub gain: f32,
    pub vertical_shift: f32,
    pub peak_smoothing: f32,
    pub offset_x: f32,
    pub offset_y: f32,
    pub offset_z: f32
}

impl RidgeNoiseSettings {
    pub fn get_noise_args(
        &self,
        mut prng: PRNG
    ) -> Vec<[f32; 4]> {
        let seeded_offset = Vec3::new(prng.get_value(), prng.get_value(), prng.get_value()) * prng.get_value() * 10000.0;

        vec![[seeded_offset.x + self.offset_x, seeded_offset.y + self.offset_y, seeded_offset.z + self.offset_z, self.layers],
        [self.persistence, self.lacunarity, self.scale, self.elevation],
        [self.power, self.gain, self.vertical_shift, self.peak_smoothing]]
    }
}

#[repr(C)]
#[derive(ShaderType, Clone, Default, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Crater {
    pub centre: Vec3,
    pub radius: f32,
    pub floor_height: f32,
    pub smoothness: f32
}

#[derive(Resource, Default, Debug, Reflect, Clone)]
pub struct CraterSettings {
    pub craters: f32,
    pub crater_size_min: f32,
    pub crater_size_max: f32,
    pub rim_steepness: f32,
    pub rim_width: f32,
    pub smooth_min: f32,
    pub smooth_max: f32,
    pub size_distribution: f32
}

impl CraterSettings {
    pub fn get_rim_steepness(&self) -> f32 {
        self.rim_steepness
    }

    pub fn get_rim_width(&self) -> f32 {
        self.rim_width
    }

    pub fn get_craters(
        &self,
        crater_seed: u64
    ) -> Vec<Crater> {
        let num_craters = self.craters as usize;

        let mut craters = Vec::with_capacity(num_craters);

        let seed = crater_seed + 2;
        let mut prng = PRNG { rng: StdRng::seed_from_u64(seed) };

        for _i in 0..num_craters {
            let t = prng.value_bias_lower(self.size_distribution);
            let radius = self.crater_size_min.lerp(self.crater_size_max, t);
            let floor_height = -1.2.lerp(-0.2, t + prng.value_bias_lower(0.3));
            let smoothness = self.smooth_min.lerp(self.smooth_max, 1.0 - t);

            let centre = prng.random_on_unit_sphere();

            craters.push(
                Crater { centre,radius,floor_height,smoothness }
            );
        }

        craters
    }
}

#[derive(Resource)]
pub struct AsteroidSettings {
    pub peturb_strength: f32,
    pub crater_settings: CraterSettings,
    pub simple_noise_settings: SimpleNoiseSettings,
    pub ridge_noise_settings: RidgeNoiseSettings,
    pub ridge_noise_settings2: RidgeNoiseSettings
}

impl Default for AsteroidSettings {
    fn default() -> Self {
        AsteroidSettings {
            peturb_strength: 0.45,
            crater_settings: CraterSettings { 
                craters: 100.0, 
                crater_size_min: 0.01, 
                crater_size_max: 0.14, 
                rim_steepness: 0.13, 
                rim_width: 0.61, 
                smooth_min: 0.5, 
                smooth_max: 0.76, 
                size_distribution: 0.05 
            },
            simple_noise_settings: SimpleNoiseSettings { 
                layers: 3.4, 
                lacuranity: 2.0, 
                persistence: 0.5, 
                scale: 0.66, 
                elevation: 13.5, 
                offset_y: 4.57, 
                ..default() 
            },
            ridge_noise_settings: RidgeNoiseSettings { 
                layers: 5.0, 
                lacunarity: 2.0, 
                persistence: 0.5, 
                scale: 4.44, 
                power: 0.92, 
                elevation: 0.5, 
                gain: 0.5, 
                ..default() 
            },
            ridge_noise_settings2: RidgeNoiseSettings { 
                layers: 4.0, 
                lacunarity: 5.0, 
                persistence: 0.42, 
                scale: 2.97, 
                elevation: -3.64, 
                gain: 1.0, 
                peak_smoothing: 1.5, 
                ..default() }
        }
    }
}