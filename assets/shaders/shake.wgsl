#import bevy_sprite::mesh2d_functions::{get_world_from_local, mesh2d_position_local_to_clip}
#import bevy_sprite::mesh2d_view_bindings::globals

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
}

struct ShakeMaterial {
    shake_intensity: f32,
    shake_speed: f32,
    uv_offset: vec2<f32>,
    uv_scale: vec2<f32>,
}

@group(2) @binding(0) var<uniform> material: ShakeMaterial;
@group(2) @binding(1) var base_texture: texture_2d<f32>;
@group(2) @binding(2) var base_sampler: sampler;

@vertex
fn vertex(v: Vertex) -> VertexOutput {
    var out: VertexOutput;

    // Shake factor: 1.0 at top (uv.y=0), 0.0 at bottom (uv.y=1)
    let shake_factor = 1.0 - v.uv.y;

    // Horizontal displacement using sine wave
    let displacement = sin(globals.time * material.shake_speed + v.position.y * 0.1)
                       * material.shake_intensity
                       * shake_factor
                       * 3.0;

    var position = v.position;
    position.x += displacement;

    out.clip_position = mesh2d_position_local_to_clip(
        get_world_from_local(v.instance_index),
        vec4<f32>(position, 1.0),
    );

    // Transform UV to atlas region
    out.uv = material.uv_offset + v.uv * material.uv_scale;

    return out;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(base_texture, base_sampler, in.uv);
}
