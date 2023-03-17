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
    @location(7) rotation: f32,
    @location(8) scale: f32,
}

struct Dimensions {
    width: f32,
    height: f32,
}

// I think the trig functions are in radians...
@vertex
fn vs_main(
    model: VertexInput,
    entity: EntityInput,
) -> VertexOutput {
    // Currently this is for rotation only
    // Angle calculation might actualy be somewhat off....
    // TEMP, THESE WILL BE PASSED INTO SHADER SOMEHOW
    var screen_width = 562.0;
    var screen_height = 1021.0;
    var out: VertexOutput;
    var orig_x: f32 = model.position.x - entity.origin.x;
    var orig_y: f32 = model.position.y - entity.origin.y;
    var new_x = (orig_x * cos(entity.rotation)) - (orig_y * sin(entity.rotation));
    var new_y = (orig_y * cos(entity.rotation)) + (orig_x * sin(entity.rotation));
    var final_x = new_x + entity.origin.x;
    var final_y = new_y + entity.origin.y;
    var final_vec = (vec4<f32>(final_x, final_y, 0.0, 0.0) + vec4<f32>(entity.position, 0.0, 0.0));
    final_vec.x = final_vec.x * entity.scale;
    final_vec.y = final_vec.y * entity.scale;
    final_vec = vec4<f32>(normalise(vec2<f32>(final_vec.x, final_vec.y), screen_width, screen_height), 1.0, 1.0);
    out.tex_coords = model.tex_coords;
    out.clip_position = final_vec;
    return out;
}



fn normalise(given: vec2<f32>, width: f32, height: f32) -> vec2<f32> {
    var new_vec: vec2<f32>;
    new_vec.x = ((2.0 * (given.x)) / width) - 1.0;
    new_vec.y = ((2.0 * (given.y)) / height) - 1.0;
    return new_vec;
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
