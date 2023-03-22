struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
}

struct EntityInput {
    @location(5) position: vec2<f32>,
    @location(6) origin: vec2<f32>,
    @location(7) rotation_1: vec2<f32>,
    @location(8) rotation_2: vec2<f32>,
    @location(9) scale_1: vec2<f32>,
    @location(10) scale_2: vec2<f32>,
}

struct Dimensions {
    width: f32,
    height: f32,
}

// Trig function in radians
@vertex
fn vs_main(
    model: VertexInput,
    entity: EntityInput,
) -> VertexOutput {
    // TEMP, THESE WILL BE PASSED INTO SHADER SOMEHOW
    var screen_width = 562.0;
    var screen_height = 1021.0;
    var out: VertexOutput;
    var rot_mat = mat2x2<f32>(
        entity.rotation_1,
        entity.rotation_2,
    );
    var scale_mat = mat2x2<f32>(
        entity.scale_1,
        entity.scale_2,
    );
    var orig_vec = vec2<f32>(model.position.x- entity.origin.x, model.position.y - entity.origin.y);
    orig_vec = (orig_vec * rot_mat * scale_mat) + entity.position + entity.origin;
    orig_vec = normalise(orig_vec, screen_width, screen_height);
    out.tex_coords = model.tex_coords;
    out.clip_position = vec4<f32>(orig_vec, 1.0, 1.0);
    return out;
}

fn normalise(given: vec2<f32>, width: f32, height: f32) -> vec2<f32> {
    return vec2<f32>(((2.0 * (given.x)) / width) - 1.0, ((2.0 * (given.y)) / height) - 1.0);
}

// Fragment shader

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, in.tex_coords);
}
