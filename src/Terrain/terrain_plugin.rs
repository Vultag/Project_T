

use bevy::{prelude::*};

use super::{terrain_mesh::TerrainMesh};
use super::terrain_noise::TerrainParameters;
use noise::{NoiseFn, Perlin};




pub struct TerrainPlugin;



impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App)
    {
        app
        .add_systems(Startup, spawn_terrain)
        .insert_resource(TerrainParameters {
            size: 20.0,
            subdivision_pow: 4,
            NOISE_SCALE: 0.05,
            CLIFF_STEEPNESS: 15.0,
            PLATEAU_HEIGHT: 2.0,
            HILL_VOLUME: 0.5,
            PIT_VOLUME: 0.5,
            perlin:Perlin::new(1),
        });
    }

    //create whole terrain here that will tile in terrain mesh


}

 #[derive(Default,Component)]
pub struct TerrainMeshData {
    pub coords:[u32;2],
}

pub fn spawn_terrain(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: ResMut<AssetServer>,
    mut terrain_parameters: ResMut<TerrainParameters>,
    //mut noise_map: ResMut<NoiseMap>,
    //mut terainmaterial: ResMut<Assets<TerainMaterial>>,
    //mut terrain_mesh_res: ResMut<TerrainMeshResource>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    //mut line_materials: ResMut<Assets<debug_line::LineMaterial>>,
    //mut mattest_asset: ResMut<Assets<testMat>>
) {

    
    let terrain_size = [4,4];

    for x in 0..terrain_size[0]
    {
        for z in 0..terrain_size[1]
        {

            let coords = [x*256,z*256];

            let mesh = TerrainMesh::build_RTIN_terrain( coords,terrain_parameters.subdivision_pow,&terrain_parameters);
            let terrain_shaded_mesh_handle = meshes.add(mesh.clone());
        
            //terrain_mesh_res.shaded = terrain_shaded_mesh_handle;
        
            let custom_texture_handle: Handle<Image> = asset_server.load("uv_grid.png");
            let custom_texture_handle_g: Handle<Image> = asset_server.load("grass-texture-background.png");
            let custom_texture_handle_d: Handle<Image> = asset_server.load("texture_dirt.png");
        
            // let terain_mat = terainmaterial.add(TerainMaterial{
            //     color_texture_g:Some(custom_texture_handle_g),
            //     value: 0.0,
            //     color_texture_d:Some(custom_texture_handle_d),
            // });
        
            let pos_x = (x as f32)*20.0 - ((terrain_size[0] as f32)*20.0)*0.5;
            let pos_z = (z as f32)*20.0 - ((terrain_size[1] as f32)*20.0)*0.5;

            commands.spawn((MaterialMeshBundle  {
                transform: Transform::from_translation(Vec3::new(pos_x, 0.0, pos_z)),
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
            )).insert(TerrainMeshData{coords});

        }
    }



}