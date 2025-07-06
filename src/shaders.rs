use bevy::{
    prelude::TypePath,
    render::render_resource::ShaderRef
};

use bevy_easy_compute::prelude::ComputeShader;

#[derive(TypePath)]
pub struct AsteroidShapeComputeShader;

impl ComputeShader for AsteroidShapeComputeShader {
    fn shader() -> ShaderRef {
        "shaders/asteroid.wgsl".into()
    }
}

#[derive(TypePath)]
pub struct NormalComputeShader;

impl ComputeShader for NormalComputeShader {
    fn shader() -> ShaderRef {
        "shaders/normals.wgsl".into()
    }
}

#[derive(TypePath)]
pub struct NormalizeNormalComputerShader;

impl ComputeShader for NormalizeNormalComputerShader {
    fn shader() -> ShaderRef {
        "shaders/normalise.wgsl".into()
    }
}
