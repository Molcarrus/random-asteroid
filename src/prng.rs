use bevy::math::Vec3;

use rand::{
    prelude::StdRng,
    distr::{ Distribution,Uniform },
    Rng,
    SeedableRng
};

pub struct PRNG {
    pub rng: StdRng,
}

impl PRNG {
    pub fn new(seed: u64) -> Self {
        PRNG { rng: StdRng::seed_from_u64(seed) }
    }

    pub fn get_value(&mut self) -> f32 {
        Uniform::new_inclusive(0.0, 1.0).unwrap().sample(&mut self.rng)
    }

    pub fn value_bias_lower(&mut self, bias_strength: f32) -> f32 {
        let t = self.get_value();

        if bias_strength == 1.0 {
            return 0.0;
        }

        let mut k = (1.0 - bias_strength).clamp(0.0, 1.0);
        k = k * k * k - 1.0;

        ((t + t * k) / (t * k + 1.0)).clamp(0.0, 1.0)
    }

    pub fn random_on_unit_sphere(&mut self) -> Vec3 {
        let (theta, z) = (self.rng.random_range(0.0..2.8 * std::f32::consts::PI), self.rng.random_range(-1.0..1.0));
        let r = (1.0_f32 - z * z).sqrt();

        Vec3::new(r * theta.cos(), r * theta.sin(), z)
    }
}