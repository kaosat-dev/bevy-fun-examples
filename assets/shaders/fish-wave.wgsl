/*#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_bindings
#import bevy_pbr::mesh_functions

#import bevy_pbr::prepass_bindings
#import bevy_pbr::mesh_vertex_output  MeshVertexOutput
#import bevy_pbr::mesh_view_bindings globals
#import bevy_pbr::mesh_bindings mesh
#import bevy_pbr::mesh_functions mesh_position_local_to_world
#import bevy_pbr::mesh_functions mesh_position_world_to_clip
#import bevy_pbr::mesh_functions mesh_normal_local_to_world*/

// #import bevy_pbr::pbr_functions::prepare_world_normal
#import bevy_pbr::{
    // pbr_functions::{alpha_discard as discard, apply_pbr_lighting},
    mesh_view_bindings,
    mesh_bindings,
    mesh_bindings::{mesh},
    mesh_functions,
    mesh_functions::{
        get_model_matrix,
        mesh_position_local_to_world,
        mesh_position_world_to_clip,
        mesh_normal_local_to_world
    },
    view_transformations::position_world_to_clip,
    prepass_bindings,
    mesh_view_bindings::{globals},
    mesh_vertex_output,
    mesh_vertex_output:: {MeshVertexOutput}
    
}
#import bevy_render::instance_index::get_instance_index;



struct Vertex {
    @builtin(instance_index) instance_index: u32,

    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
#ifdef VERTEX_UVS
    @location(2) uv: vec2<f32>,
#endif
#ifdef VERTEX_TANGENTS
    @location(3) tangent: vec4<f32>,
#endif
#ifdef VERTEX_COLORS
    @location(4) color: vec4<f32>,
#endif
#ifdef SKINNED
    @location(5) joint_indices: vec4<u32>,
    @location(6) joint_weights: vec4<f32>,
#endif
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    // see https://github.com/bevyengine/bevy/blob/main/crates/bevy_pbr/src/render/mesh_vertex_output.wgsl
    // #import bevy_pbr::mesh_vertex_output
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    #ifdef VERTEX_UVS
    @location(2) uv: vec2<f32>,
    #endif
    #ifdef VERTEX_TANGENTS
    @location(3) world_tangent: vec4<f32>,
    #endif
    #ifdef VERTEX_COLORS
    @location(4) color: vec4<f32>,
    #endif
};

@group(1) @binding(3)
var<uniform> speed: f32;
@group(1) @binding(4)
var<uniform> amplitude: f32;
@group(1) @binding(5)
var<uniform> min_bounds: vec3<f32>;
@group(1) @binding(6)
var<uniform> max_bounds: vec3<f32>;
@group(1) @binding(7)
var<uniform> side_to_side_intensity: f32;
@group(1) @binding(8)
var<uniform> pivot_intensity: f32;
@group(1) @binding(9)
var<uniform> wave_intensity: f32;
@group(1) @binding(10)
var<uniform> twist_intensity: f32;

fn side_to_side(position: vec4<f32>, intensity: f32, mask: f32, offset_seed: f32) -> vec4<f32>{
    let time = globals.time * speed;
    let side_to_side_motion = cos(time + offset_seed) * intensity;

    var output = vec4(position.x, position.y, position.z, position.w);
    output.x += side_to_side_motion;
    return output;
}

fn pivot(position: vec4<f32>, intensity: f32, mask: f32, offset_seed: f32) -> vec4<f32>{
    let time = globals.time * speed;

    let pivot_angle = cos(time + offset_seed) * 0.1 * intensity;
    let rotation_matrix = mat2x2<f32>(
        vec2<f32>(cos(pivot_angle), -sin(pivot_angle)),
        vec2<f32>(sin(pivot_angle), cos(pivot_angle))
    );
    let rotation = rotation_matrix * position.xy;

    var output = vec4(rotation.x, rotation.y, position.z, position.w);
    return output;
}

fn wave(position: vec4<f32>, intensity: f32, body_gradient:f32, mask: f32, offset_seed: f32) -> vec4<f32>{
    let time = globals.time * speed;
    let wave_motion = cos(time + body_gradient + offset_seed) * intensity * mask ;

    let output = vec4(position.x + wave_motion, position.y, position.z, position.w);
    return output;
}

fn twist(position: vec4<f32>, intensity: f32, body_gradient:f32, mask: f32, offset_seed: f32) -> vec4<f32>{
    let time = globals.time * speed;
    let twist_angle = cos(time + body_gradient + offset_seed) * intensity * body_gradient;
    let twist_matrix = mat2x2<f32>(
        vec2<f32>(cos(twist_angle), -sin(twist_angle)),
        vec2<f32>(sin(twist_angle), cos(twist_angle))
    );
    let twist = mix(position.xz, twist_matrix * position.xz, mask);
    //mix(VERTEX.xy, twist_matrix * VERTEX.xy, mask);
    var output = vec4(twist.x, position.y, twist.y, position.w);
    return output;
}

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {

    let time = globals.time;

    // randomize movement based on world coordinates 
    let world_pos = mesh_position_local_to_world(get_model_matrix(vertex.instance_index), vec4<f32>(vertex.position.x, vertex.position.y, vertex.position.z, 1.0));
    let offset_seed = (world_pos.x * world_pos.y * world_pos.z) / 100. ;

    // body gradient
    let len = max_bounds.y - min_bounds.y;
    // body gradient is a value between 0 & 1 representing where we the current position is along the model , based on its bounds
    // multiplied by exp(speed) to have more movement amplitude at higher speeds
    let body_gradient = ((vertex.position.y - abs(min_bounds.y)) / len ) * exp(speed * 0.1) ; 

    let mask_black = 0.2;
    let mask_white = 1.0;
    let mask = body_gradient * 0.1;//smoothstep(mask_black, mask_white, 1.0 - body_gradient); //

    // side to side
    //let side_to_side_intensity = 0.3;
    // pivot
    //let pivot_intensity = 0.2;
    // wave
    //let wave_intensity = 2.8;
    // twist
    //let twist_intensity = 0.7;

    // calculate output position
    var position = vec4<f32>(vertex.position.x, vertex.position.y, vertex.position.z, 1.0);

    position = side_to_side(position, side_to_side_intensity, mask, offset_seed);
    position = pivot(position, pivot_intensity, mask, offset_seed);
    position = wave(position, wave_intensity, body_gradient, mask, offset_seed);
    position = twist(position, twist_intensity, body_gradient, mask, offset_seed);

    var out: VertexOutput;
    #ifdef SKINNED
        var model = skin_model(vertex.joint_indices, vertex.joint_weights);
        out.world_normal = skin_normals(model, vertex.normal);
    #else
        var model = get_model_matrix(vertex.instance_index);
        out.world_normal = mesh_normal_local_to_world(vertex.normal, vertex.instance_index);

    #endif
    #ifdef VERTEX_UVS
        out.uv = vertex.uv;
    #endif
    #ifdef VERTEX_TANGENTS
        out.world_tangent = mesh_tangent_local_to_world(model, vertex.tangent);
    #endif
    #ifdef VERTEX_COLORS
        out.color = vertex.color;
    #endif
    #ifdef VERTEX_POSITIONS
        out.world_position = mesh_position_local_to_world(model, position);
    #endif

    out.clip_position = position_world_to_clip(out.world_position.xyz);
    return out;
}

struct CustomMaterial {
    color: vec4<f32>
};

@group(1) @binding(2)
var<uniform> material: CustomMaterial;


@fragment
fn fragment(
   in: VertexOutput
) -> @location(0) vec4<f32> {
    let b = abs(sin(globals.time * speed));
    let color = material.color;
    #ifdef VERTEX_COLORS
        return in.color;
    #else
        return material.color;
    #endif
}