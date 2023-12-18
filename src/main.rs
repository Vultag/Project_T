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
    .init_resource::<TerrainMeshResource>()
    .add_plugins(MaterialPlugin::<TerainMaterial>::default())
    .add_plugins(MaterialPlugin::<TestMaterial>::default())
    .add_plugins(NoisyShaderPlugin)
    .add_systems(Startup, create_terrain)
    .add_systems(Startup, test_create_terrain)
    .add_systems(Startup, spawn_camera)
    .add_systems(Update, camera_control)

    .run()
}

pub struct TerrainMeshData {
    pub vertices: Vec::<Vec3>,
    pub indices: Vec::<u32>
 }


 #[derive(Default,Resource)]
 pub struct TerrainMeshResource {
     pub shaded: Handle<Mesh>,
     pub wireframe: Handle<Mesh>,
 }


fn test_create_terrain(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: ResMut<AssetServer>,
    mut terrain_mesh_res: ResMut<TerrainMeshResource>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    //mut mattest_asset: ResMut<Assets<testMat>>
) {

    
    let mut ground_mesh = Mesh::new(PrimitiveTopology::TriangleList);

    //let mut vertices : Vec::<[f32; 3]> = Vec::new();
    let mut positions : Vec::<[f32; 3]> = Vec::new();
    let mut normals : Vec::<[f32; 3]> = Vec::new();
    let mut indices : Vec::<u32> = Vec::new();
    let mut uvs = Vec::new();

    //number of vertices
    let vertex_number = ( 2 * 2) as usize; 

    // vertices.resize(vertex_number, [0.0f32, 0.0f32, 0.0f32]);
    // normals.resize(vertex_number, [0.0f32, 1.0f32, 0.0f32]);
    // let uvs = vec![[0.0, 0.0]; vertices.len()];


    let mut vertex_index = 0;
    // for cy in 0..(heightmap.height() as i32 +1) {
    //     for cx in 0..(heightmap.width() as i32 +1) {
            
    //         let height = sample_vertex_height(cy, cx, heightmap);

    //         vertices[vertex_index] = [cx as f32 * options.pixel_side_length,
    //           height * options.max_image_height, 

    //           cy as f32 * options.pixel_side_length];

    //         vertex_index += 1;
    //     }
    // }
        let v_num = vertex_number as i32;

        let plane_size = 2.0;

        // for v in 0..(v_num){

        //     vertices[vertex_index] = [vertex_index as f32 * 0.1,vertex_index as f32 * 0.1,vertex_index as f32 * 0.1];
        //     println!("{:?}",vertices[vertex_index]);

        //     vertex_index +=1;
        // }

        let mut vertices = [
            ([plane_size, 0.0, -plane_size], [0.0, 1.0, 0.0], [1.0, 1.0]),
            ([plane_size, 0.0, plane_size], [0.0, 1.0, 0.0], [1.0, 0.0]),
            ([-plane_size, 0.0, plane_size], [0.0, 1.0, 0.0], [0.0, 0.0]),
            ([-plane_size, 0.0, -plane_size], [0.0, 1.0, 0.0], [0.0, 1.0]),
        ];
        for (position, normal, uv) in vertices.iter() {
            positions.push(*position);
            normals.push(*normal);
            uvs.push(*uv);
        }


        // vertices[0] = [2.0,0.0,-2.0];
        // vertices[1] = [2.0,0.0,2.0];
        // vertices[2] = [-2.0,0.0,2.0];
        // vertices[3] = [-2.0,0.0,-2.0];
        // normals[0] = [0.0,1.0,0.0];
        // normals[1] = [0.0,1.0,0.0];
        // normals[2] = [0.0,1.0,0.0];
        // normals[3] = [0.0,1.0,0.0];

        indices = vec![0, 2, 1, 0, 3, 2];
    
            // for i in 0..(v_num){
            //     indices.extend([
            //         i * v_num + i, 
            //         (i + 1) * v_num + i + 1, 
            //         i * v_num + i + 1, 
            //     ].iter());
            //     indices.extend([
            //         i * v_num + i, 
            //         (i + 1) * v_num + i, 
            //         (i + 1) * v_num + i + 1, 
            //     ].iter());  
            // }


        ground_mesh.insert_attribute(
            Mesh::ATTRIBUTE_POSITION,
            VertexAttributeValues::Float32x3(positions));
        ground_mesh.insert_attribute(
            Mesh::ATTRIBUTE_NORMAL, 
            VertexAttributeValues::Float32x3(normals));
        // ground_mesh.insert_attribute(
        //     Mesh::ATTRIBUTE_UV_0,
        //      VertexAttributeValues::Float32x2(uvs));
        ground_mesh.set_indices(Some(Indices::U32(indices)));


    //let (terrain_shaded_mesh) = rtin_load_terrain(image_filename,&rtin_params);

    let terrain_shaded_mesh_handle = meshes.add(ground_mesh);

    //terrain_mesh_res.shaded = terrain_shaded_mesh_handle;

    let custom_texture_handle: Handle<Image> = asset_server.load("texture_dirt.png");

    commands.spawn((MaterialMeshBundle  {
        mesh: terrain_shaded_mesh_handle,
        material: materials.add(StandardMaterial{base_color_texture:Some(custom_texture_handle),

            ..Default::default()
            }),
        ..default()
    },
    ));



}


fn create_terrain(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    //mut materials: ResMut<Assets<StandardMaterial>>,
    mut terainmaterial: ResMut<Assets<TerainMaterial>>,
    mut testmaterial: ResMut<Assets<TestMaterial>>,
    asset_server: ResMut<AssetServer>,
    //mut mattest_asset: ResMut<Assets<testMat>>
) {

    // commands.spawn(MaterialMeshBundle)
    // {

    // }

    let custom_texture_handle_g: Handle<Image> = asset_server.load("grass-texture-background.png");
    let custom_texture_handle_d: Handle<Image> = asset_server.load("texture_dirt.png");

    let terain_mat = terainmaterial.add(TerainMaterial{
        color_texture_g:Some(custom_texture_handle_g),
        value: 0.0,
        color_texture_d:Some(custom_texture_handle_d),
    });
    let test_mat = testmaterial.add(TestMaterial{});

    


    let ground_mesh = meshes.add(Mesh::from(shape::Plane { size: 15.0, subdivisions: 15 }));

    // commands.spawn((MaterialMeshBundle  {
    //     mesh: ground_mesh,
    //     //material: materials.add(Color::rgb(0.0, 1.0, 0.0).into()),
    //     // material: materials.add(StandardMaterial{base_color_texture:Some(custom_texture_handle),

    //     //     ..Default::default()
    //     // }),
    //     material: test_mat,
    //     ..default()
    // },
    // GroundTag{},
    // ));

    
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-90.0),
            ..default()
        },

        ..default()
    });

    //toggle_texture(meshes.get_mut(ground_mesh).unwrap());

}

fn spawn_camera(mut commands: Commands) {

    let direction = Vec3{x:0.5,y:0.4,z:0.5};
    let length = 6.0;
    let pos = direction.normalize()*length;


    commands.spawn((
        Camera3dBundle {
        projection: PerspectiveProjection {
            near: 0.0,
            ..default()
        }
        .into(),
        transform: Transform::from_xyz(pos.x,pos.y,pos.z)
        .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    },
    CameraData{lenght :length,target : Vec3::ZERO, direction: pos.normalize()},            
    ));


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
            //println!("{}", ((simplex_noise_2d_seeded(Vec2{x:mat.1.value,y:mat.1.value},5.0))));
        }

        //println!("{}", cam.1.lenght);

        //ZOOM disabled
        cam.1.lenght-=scroll.y*0.4*cam.1.lenght;
        cam.1.lenght = cam.1.lenght.clamp(0.1, 10.0);
        
        // = lerp cam.1.direction*cam.1.lenght;

    }



    if (mouse_button.pressed(MouseButton::Right))
    {

        for mouv in mouse_motion.read()
        {
            //previous rot
            //cam.0.rotate_around(cam.1.target, Quat::from_rotation_y(mouv.delta.x*0.006));
            
            let factor = mouv.delta.x*0.001;

            cam.1.direction = Vec3{x: cam.1.direction.x*factor.cos() + cam.1.direction.z*-factor.sin(),y: cam.1.direction.y, z: cam.1.direction.x*factor.sin() + cam.1.direction.z*factor.cos()};

            //println!("{}", cam.1.direction);


            //2D ROT
            //cam.0.rotate_around(Vec3::ZERO, Quat::from_euler(EulerRot::XYZ,mouv.delta.y*0.006,mouv.delta.x*0.006,0.0));
        }
    }

    if(mouse_button.just_pressed(MouseButton::Left))
    {



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
    //Quat::mul_vec3(mouv)
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