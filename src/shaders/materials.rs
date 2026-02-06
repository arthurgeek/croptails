use bevy::{
    mesh::MeshVertexBufferLayoutRef,
    prelude::*,
    render::render_resource::{
        AsBindGroup, RenderPipelineDescriptor, SpecializedMeshPipelineError,
    },
    shader::ShaderRef,
    sprite_render::{AlphaMode2d, Material2d, Material2dKey},
};

#[derive(Asset, AsBindGroup, TypePath, Clone)]
pub struct ShakeMaterial {
    #[uniform(0)]
    pub shake_intensity: f32,
    #[uniform(0)]
    pub shake_speed: f32,
    #[uniform(0)]
    pub uv_offset: Vec2,
    #[uniform(0)]
    pub uv_scale: Vec2,
    #[texture(1)]
    #[sampler(2)]
    pub texture: Handle<Image>,
}

impl Material2d for ShakeMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/shake.wgsl".into()
    }

    fn fragment_shader() -> ShaderRef {
        "shaders/shake.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }

    fn specialize(
        descriptor: &mut RenderPipelineDescriptor,
        layout: &MeshVertexBufferLayoutRef,
        _key: Material2dKey<ShakeMaterial>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        let vertex_layout = layout.0.get_layout(&[
            Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
            Mesh::ATTRIBUTE_UV_0.at_shader_location(1),
        ])?;
        descriptor.vertex.buffers = vec![vertex_layout];
        Ok(())
    }
}
