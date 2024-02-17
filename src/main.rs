#![allow(warnings)]


use std::any::Any;
use std::collections::btree_map::Range;
use std::f32::consts::{PI, E};

use bevy::ecs::archetype::ArchetypeRow;
use bevy::input::mouse::MouseMotion;
use bevy::input::mouse::MouseButtonInput;
use bevy::math;

use bevy::utils::petgraph::matrix_graph::Zero;
use noise::{NoiseFn, Perlin};
use rand::Rng;

use noisy_bevy::{simplex_noise_2d_seeded,fbm_simplex_2d_seeded, NoisyShaderPlugin};

use bevy_fps_counter::{FpsCounter, FpsCounterPlugin};


use bevy::input::mouse::MouseWheel;
use bevy::reflect::TypeUuid;
use bevy::render::render_resource::{AddressMode, PrimitiveTopology};
use bevy::render::render_resource::AsBindGroup;
use bevy::render::mesh::{VertexAttributeValues, Indices};
use bevy::render::render_resource::ShaderRef;
use bevy::ui::debug;
use bevy::{
    pbr::wireframe::{NoWireframe, Wireframe, WireframeColor, WireframeConfig, WireframePlugin},
    prelude::*,
    render::{
        render_resource::{WgpuFeatures,
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        settings::{RenderCreation, WgpuSettings},
        RenderPlugin,
    },
};

mod Terrain;

mod debug_line;

use debug_line::LineMaterial;
use Terrain::terrain_mesh::TerrainMesh;
use Terrain::terrain_ui::TerrainUIPlugin;

#[derive(Component)]
struct CameraData {

    pub lenght: f32,
    pub target: Vec3,
    pub direction: Vec3

}
#[derive(Component)]
struct GroundTag {}


#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
//#[uuid = "af7c4b08-d02a-41c9-8d7c-3f8ce7a2ccb3"]
pub struct TerainMaterial{

    #[texture(0)]
    #[sampler(1)]
    pub color_texture_g: Option<Handle<Image>>,
    #[uniform(2)]
    pub value: f32,
    //ajout 2de texture dirt
    #[texture(3)]
    #[sampler(4)]
    pub color_texture_d: Option<Handle<Image>>,
    // alpha_mode: AlphaMode,
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct TestMaterial{


}

impl Material for TerainMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/shader_test.wgsl".into()
    }
    // fn alpha_mode(&self) -> AlphaMode {
    //     self.alpha_mode
    // }
}


impl Material for TestMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/shader_terrain_test.wgsl".into()
    }
    // fn alpha_mode(&self) -> AlphaMode {
    //     self.alpha_mode
    // }
}

fn main() {
    App::new()
    .add_plugins((
        DefaultPlugins.set(RenderPlugin {
            render_creation: RenderCreation::Automatic(WgpuSettings {
                // WARN this is a native only feature. It will not work with webgl or webgpu
                features: WgpuFeatures::POLYGON_MODE_LINE,
                ..default()
            }),
        }),
        // You need to add this plugin to enable wireframe rendering
        WireframePlugin,
    ))
    // Wireframes can be configured with this resource. This can be changed at runtime.
        .insert_resource(WireframeConfig {
        // The global wireframe config enables drawing of wireframes on every mesh,
        // except those with `NoWireframe`. Meshes with `Wireframe` will always have a wireframe,
        // regardless of the global configuration.
        global: true,
        // Controls the default color of all wireframes. Used as the default color for global wireframes.
        // Can be changed per mesh using the `WireframeColor` component.
        default_color: Color::RED,
    })
    //.init_resource::<TerrainMeshResource>()
    .add_plugins(MaterialPlugin::<TerainMaterial>::default())
    .add_plugins(MaterialPlugin::<TestMaterial>::default())
    .add_plugins(MaterialPlugin::<LineMaterial>::default())
    .add_plugins(NoisyShaderPlugin)
    .add_plugins(FpsCounterPlugin)
    .add_plugins(TerrainUIPlugin)
    .add_systems(Startup, test_create_terrain)
    .add_systems(Startup, spawn_camera)
    .add_systems(Update, camera_control)
    //.add_systems(Startup, terrain_authoring_window)

    .run()
}

#[derive(Default,Component)]
pub struct TerrainMeshData {
    pub subdivision_pow:u32,
    pub NOISE_SCALE:f32,
    pub CLIFF_STEEPNESS:f32,
    pub PLATEAU_HEIGHT:f32,
    pub HILL_VOLUME:f32,
    pub PIT_VOLUME:f32,
 }


fn test_create_terrain(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: ResMut<AssetServer>,
    mut terainmaterial: ResMut<Assets<TerainMaterial>>,
    //mut terrain_mesh_res: ResMut<TerrainMeshResource>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut line_materials: ResMut<Assets<debug_line::LineMaterial>>,
    //mut mattest_asset: ResMut<Assets<testMat>>
) {

    
    const subdivision_pow:u32 = 4;
    
    let mesh = TerrainMesh::build_RTIN_terrain( 100.0, subdivision_pow);
    let terrain_shaded_mesh_handle = meshes.add(mesh.clone());

    //terrain_mesh_res.shaded = terrain_shaded_mesh_handle;

    let custom_texture_handle: Handle<Image> = asset_server.load("uv_grid.png");
    let custom_texture_handle_g: Handle<Image> = asset_server.load("grass-texture-background.png");
    let custom_texture_handle_d: Handle<Image> = asset_server.load("texture_dirt.png");

    let terain_mat = terainmaterial.add(TerainMaterial{
        color_texture_g:Some(custom_texture_handle_g),
        value: 0.0,
        color_texture_d:Some(custom_texture_handle_d),
    });

    commands.spawn((MaterialMeshBundle  {
        mesh: terrain_shaded_mesh_handle,
        //material: terain_mat,
        material: materials.add(StandardMaterial{
            
            base_color_texture:Some(custom_texture_handle),
            alpha_mode:AlphaMode::Opaque,
            double_sided:true,

            ..Default::default()
            }),
        ..default()
    },
    )).insert(TerrainMeshData{
        subdivision_pow,
        NOISE_SCALE:0.05,
        CLIFF_STEEPNESS:15.0,
        PLATEAU_HEIGHT:2.0,
        HILL_VOLUME:0.5,
        PIT_VOLUME:0.5
    });

    // DRAW TERRAIN MESH NORMALS
    /* 
    let mesh = mesh;
    let v_pos =  mesh.attribute(Mesh::ATTRIBUTE_POSITION);
    let v_normal =  mesh.attribute(Mesh::ATTRIBUTE_NORMAL);
    //println!("append:{}",mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap().len());
    for i in  0..mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap().len()
    {
        // Spawn a list of lines with start and end points for each lines
        commands.spawn(MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(debug_line::LineList {
                lines: vec![
                    (Vec3::from(v_pos.unwrap().as_float3().unwrap()[i]),
                    (Vec3::from(v_pos.unwrap().as_float3().unwrap()[i])
                    +(Vec3::from(v_normal.unwrap().as_float3().unwrap()[i]).normalize()*Vec3::new(4.0, 4.0, 4.0)))
                    )
                ],
            })),
            transform: Transform::from_translation(Vec3::ZERO),
            material: line_materials.add(debug_line::LineMaterial {
                color: Color::RED,
            }),
            ..default()
        });
    }
    */


}



fn spawn_camera(mut commands: Commands) {

    let direction = Vec3{x:0.5,y:0.4,z:0.5};
    let length = 6.0;
    let pos = direction.normalize()*length;

    commands.spawn((
        Camera3dBundle {
        projection: PerspectiveProjection {
            near: 0.1,
            ..default()
        }
        .into(),
        transform: Transform::from_xyz(pos.x,pos.y,pos.z)
        .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    },
    CameraData{lenght :length,target : Vec3::ZERO, direction: pos.normalize()},   
));         
//light tied to camera
// )).insert(PointLightBundle {
//     // transform: Transform::from_xyz(5.0, 8.0, 2.0),
//     transform: Transform::from_xyz(1.0, 2.0, 0.0),
//     point_light: PointLight {
//         intensity: 16000.0, // lumens - roughly a 100W non-halogen incandescent bulb
//         color: Color::WHITE,
//         shadows_enabled: true,
//         ..default()
//     },
//     ..default()
// });

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 10.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },

        ..default()
    });




}

fn camera_control(
    keyboard_input: Res<Input<KeyCode>>,
    mut mouse_motion: EventReader<MouseMotion>,
    mut mouse_wheel: EventReader<MouseWheel>,
    mouse_button: Res<Input<MouseButton>>,    
    mesh_query: Query<&Handle<Mesh>, With<GroundTag>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut query: Query<(&mut Transform,&mut CameraData)>,
    mut terainmaterial: ResMut<Assets<TerainMaterial>>,

    time: Res<Time>,
) 
{
    let mut cam = query.single_mut();


    //test noise
    // for mat in terainmaterial.iter_mut()
    // {
    //     mat.1.value += time.delta_seconds();
    //     let val = fbm_simplex_2d_seeded(Vec2{x:mat.1.value,y:mat.1.value}, 1, 1.0, 1.0, 5.0)*1.28255;
    //     if(val>1.0 || val <(-1.0))
    //     {
    //         println!("{}", val);
    //     }
    //     //println!("{}", ((simplex_noise_2d_seeded(Vec2{x:mat.1.value,y:mat.1.value},5.0))));
    // }


    for scroll in mouse_wheel.read()
    {

        
        for mat in terainmaterial.iter_mut()
        {
            mat.1.value += scroll.y*0.1;
        }

        //println!("{}", cam.1.lenght);

        //ZOOM disable
        cam.1.lenght-=scroll.y*0.4*cam.1.lenght;
        cam.1.lenght = cam.1.lenght.clamp(0.1, 50.0);
        
        // = lerp cam.1.direction*cam.1.lenght;

    }



    if (mouse_button.pressed(MouseButton::Right))
    {

        for mouv in mouse_motion.read()
        {

            let factor = mouv.delta.x*0.001;

            cam.1.direction = Vec3{x: cam.1.direction.x*factor.cos() + cam.1.direction.z*-factor.sin(),y: cam.1.direction.y, z: cam.1.direction.x*factor.sin() + cam.1.direction.z*factor.cos()};

            //println!("{}", cam.1.direction);


            //2D ROT
            //cam.0.rotate_around(Vec3::ZERO, Quat::from_euler(EulerRot::XYZ,mouv.delta.y*0.006,mouv.delta.x*0.006,0.0));
        }
    }

    if(mouse_button.just_pressed(MouseButton::Left))
    {
        //TO DO

        // let mesh_handle = mesh_query.get_single().expect("Query not successful");
        // let ground_mesh = meshes.get_mut(mesh_handle).unwrap();
        // toggle_texture(ground_mesh);
    }

    let mut mouv = Vec2::ZERO;
    let mouv_speed = 0.05;

    if keyboard_input.pressed(KeyCode::Z) {
        mouv.x -= 1.0*mouv_speed*cam.1.lenght*0.2;

    }
    if keyboard_input.pressed(KeyCode::Q) {
        mouv.y += 1.0*mouv_speed*cam.1.lenght*0.2;

    }
    if keyboard_input.pressed(KeyCode::S) {
        mouv.x += 1.0*mouv_speed*cam.1.lenght*0.2;

    }
    if keyboard_input.pressed(KeyCode::D) {
        mouv.y -= 1.0*mouv_speed*cam.1.lenght*0.2;

    }
    
    mouv = mouv.rotate(Vec2{x:cam.1.direction.x,y:cam.1.direction.z}); 
    cam.1.target += Vec3{x:mouv.x,y:0.0,z:mouv.y};



    //lerping disabled
    //cam.0.translation = cam.0.translation.lerp(cam.1.target + (cam.1.direction*cam.1.lenght), time.delta_seconds()*3.0);
    cam.0.translation = cam.1.target + (cam.1.direction*cam.1.lenght);
    cam.0.look_at(cam.1.target, Vec3::Y)
    //cam.0.translation = cam.1.direction;

    //cam.0.translation = cam.0.translation.lerp(cam.0.translation.normalize()*cam.1.lenght, time.delta_seconds()*3.0);

}

// Function that changes the UV mapping of the mesh, to apply the other texture.
fn toggle_texture(mesh_to_change: &mut Mesh) {
    // Get a mutable reference to the values of the UV attribute, so we can iterate over it.
    let uv_attribute = mesh_to_change.attribute_mut(Mesh::ATTRIBUTE_UV_0).unwrap();
    // The format of the UV coordinates should be Float32x2.
    let VertexAttributeValues::Float32x2(uv_attribute) = uv_attribute else {
        panic!("Unexpected vertex format, expected Float32x2.");
    };

    // Iterate over the UV coordinates, and change them as we want.
    for uv_coord in uv_attribute.iter_mut() {
        // If the UV coordinate points to the upper, "dirt+grass" part of the texture...
        if (uv_coord[1] + 0.5) < 1.0 {
            // ... point to the equivalent lower, "sand+water" part instead,
            uv_coord[1] += 0.5;
        } else {
            // else, point back to the upper, "dirt+grass" part.
            uv_coord[1] -= 0.5;
        }
    }
}
