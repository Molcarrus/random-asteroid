use bevy::{
    math::Vec3,
    prelude::Event
};

use crate::settings::{ CraterSettings,RidgeNoiseSettings,SimpleNoiseSettings };

#[derive(Event)]
pub struct MeshDataAfterCompute(pub Vec<Vec3>, pub Vec<Vec3>);

#[derive(Event)]
pub struct PerturbStrengthChanged(pub f32);

#[derive(Event)]
pub struct CraterSettingsChanged(pub CraterSettings);

#[derive(Event)]
pub struct SimpleNoiseSettingsChanged(pub SimpleNoiseSettings);

#[derive(Event)]
pub struct RidgeNoiseSettingsChanged(pub RidgeNoiseSettings);