
#import bevy_pbr::{
    mesh_functions,
    skinning,
    morph::morph,
    forward_io::{Vertex, VertexOutput},
    view_transformations::position_world_to_clip,
}
#import noisy_bevy::fbm_simplex_2d_seeded
#import bevy_render::instance_index::get_instance_index
#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_bindings

// struct Vertex {
//     @builtin(instance_index) instance_index: u32,
//     @location(0) position: vec3<f32>,
//     @location(1) blend_color: vec4<f32>,
// };

// struct VertexOutput {
//     @builtin(position) clip_position: vec4<f32>,
//     @location(0) blend_color: vec4<f32>,
// };

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;

    //var model = mesh_functions::get_model_matrix(vertex.instance_index);
    //out.world_normal = mesh_functions::mesh_normal_local_to_world(vertex.normal);
    //out.world_position = mesh_functions::mesh_position_local_to_world(model, vec4<f32>(vertex.position-vec3<f32>(100.0,100.0,100.0), 1.0));

    return out;
}

@fragment
fn fragment(mesh:VertexOutput) -> @location(0)vec4<f32> {

    return vec4(0.0,1.0,0.0,1.0);

}