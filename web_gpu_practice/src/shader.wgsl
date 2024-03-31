// vertex shader

struct CameraUniform {
    view_proj: mat4x4<f32>,
};

// because there's more than one bind group,
// we need to specify which one we're using in the shader
// the number is determined by our render_pipeline_layout
// the texture_bind_group_layout is listed first, thus group(0)
// camera_bind_group is second, thus group(1)
@group(1) @binding(0)
var<uniform> camera: CameraUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
};

@vertex
fn vs_main(
    model: VertexInput
) -> VertexOutput {
    var out: VertexOutput;
    out.tex_coords = model.tex_coords;
    // multiplication for matricies is not communitive
    // vector goes on right, matries go on left in order of importance
    out.clip_position = camera.view_proj * vec4<f32>(model.position, 1.0);
    return out;
}

// fragment shader
@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, in.tex_coords);
}