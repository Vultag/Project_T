
//TO DO 1: make the configure the texture as repeate to not have to scale it so much and lose texture surface
//TO DO 2: blur the edges of the noise mask of the reapeting textures


#import bevy_pbr::{
    //mesh_view_bindings::globals,
    //mesh_vertex_output::MeshVertexOutput,
    forward_io::VertexOutput,
}
#import noisy_bevy::fbm_simplex_2d_seeded

@group(1) @binding(0)
var texture_g: texture_2d<f32>;
@group(1) @binding(1)
var texture_sampler_g: sampler;
@group(1) @binding(2)
var<uniform> value: f32;
@group(1) @binding(3)
var texture_d: texture_2d<f32>;
@group(1) @binding(4)
var texture_sampler_d: sampler;

// struct FragmentInput{
//     @builtin(front_facing)is_front: bool,
//     @builtin(position) frag_coord:vec4<f32>

// }

// struct VertexOutput {
//     @builtin(position) clip_position: vec4<f32>,
//     @location(0) color: vec4<f32>,
// };


@fragment
fn fragment(
    mesh:VertexOutput
    ) -> @location(0)vec4<f32> {


    //var position = mesh.world_position.xz;

    var color_d: vec4<f32>;
    var color_g: vec4<f32>;
    var final_color: vec4<f32>;

    //texture grass
    {
    //var uv = fract(mesh.uv*2.0);
    var uv = mesh.world_position.xz*3.0;

    //uv *= 0.9;
    //center the uvs
    uv = uv *2.0-vec2<f32>(1.0,1.0);

    //calcules ici

    //todo1
    //scale to overlap on other tile
    uv = uv*0.7;

    let noise = clamp(fbm_simplex_2d_seeded(mesh.world_position.xz*40.0, 7, 2.0, 0.5, 1.0)*0.13+0.45,-0.1,1.0)*vec2<f32>(1.1,1.1);// * 1.0 - 2.0;
    //let noise = clamp(fbm_simplex_2d_seeded(mesh.uv*40.0, 7, 2.0, 0.5, 1.0)*1.0,0.0,1.0)-vec2<f32>(2.5,2.5);// * 1.0 - 2.0;

    uv = uv+noise;
    //generate random angle with simple noise thought the seed (IMPERFECT) *1.28255 to mimic normalize
    let rotation = clamp(fbm_simplex_2d_seeded(floor(uv), 1, 1.0, 1.0, 3.0)*1.28255,-1.0,1.0);
    uv = fract(uv);
    uv = uv-noise;


    let rot = mat2x2<f32>(cos(rotation),-sin(rotation),sin(rotation),cos(rotation));
    uv = rot * uv;

    //reverse the offest on the uvs
    uv = uv *0.5+vec2<f32>(0.5,0.5);
    

    color_g = textureSample(texture_g,texture_sampler_g,uv);
    }

   //texture dirt
    {
    //var uv = fract(mesh.uv*2.0);
    var uv = mesh.world_position.xz*10.0;

    //uv *= 0.9;
    //center the uvs
    uv = uv *2.0-vec2<f32>(1.0,1.0);

    //calcules ici

    //todo1
    //scale to overlap on other tile
    uv = uv*0.7;

    let noise = clamp(fbm_simplex_2d_seeded(mesh.world_position.xz*40.0, 7, 2.0, 0.5, 1.0)*0.13+0.45,-0.1,1.0)*vec2<f32>(1.1,1.1);// * 1.0 - 2.0;
    //let noise = clamp(fbm_simplex_2d_seeded(mesh.uv*40.0, 7, 2.0, 0.5, 1.0)*1.0,0.0,1.0)-vec2<f32>(2.5,2.5);// * 1.0 - 2.0;

    uv = uv+noise;
    //generate random angle with simple noise thought the seed (IMPERFECT) *1.28255 to mimic normalize
    let rotation = clamp(fbm_simplex_2d_seeded(floor(uv), 1, 1.0, 1.0, 3.0)*1.28255,-1.0,1.0);
    uv = fract(uv);
    uv = uv-noise;


    let rot = mat2x2<f32>(cos(rotation),-sin(rotation),sin(rotation),cos(rotation));
    uv = rot * uv;

    //reverse the offest on the uvs
    uv = uv *0.5+vec2<f32>(0.5,0.5);
    

    color_d = textureSample(texture_d,texture_sampler_d,uv);
    }



    //blend two colors
    {


        final_color = mix(color_d,color_g,clamp(abs(fbm_simplex_2d_seeded(mesh.world_position.xz*0.5, 1, 1.0, 1.0, 4.0)*1.28255)*2.5,0.0,1.0));

    }

    //var test_noise = clamp(abs(fbm_simplex_2d_seeded(mesh.world_position.xz*0.5, 1, 1.0, 1.0, 4.0)*1.28255)*2.5,0.0,1.0);
    //var test_noise = (fbm_simplex_2d_seeded(mesh.world_position.xz*0.1, 1, 1.0, 1.0, 4.0)*1.28255)*0.5+0.5;
    var test_noise_for_hills = (clamp(fbm_simplex_2d_seeded(mesh.world_position.xz*0.05, 1, 1.0, 1.0, 4.0)*1.28255+(-0.5),0.0,1.0)*10.0);
    //invert
    //test_noise = abs(test_noise+(-1.0));

    //return vec4(uv*rotation,1.0,1.0);
    //return vec4(noise,1.0,1.0);

    //return vec4(final_color.rgb,1.0);
    //test noise
    return vec4(test_noise_for_hills,test_noise_for_hills,test_noise_for_hills,1.0);

}