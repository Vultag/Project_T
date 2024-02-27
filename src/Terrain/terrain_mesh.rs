
use bevy::app::AppExit;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::{prelude::Mesh, ecs::component};
use bevy::render::mesh::Indices;
use bevy::math::{self, vec3};
use bevy::prelude::Vec3;
use winit::event::ElementState;
use winit::window::Theme;

use std::any::Any;
use std::collections::HashMap;
use std::f64::consts::E;
use std::fmt;

//a retirer
use noise::{NoiseFn, Perlin, Seedable};

use bevy::prelude::*;

use bevy::render::render_resource::PrimitiveTopology::TriangleList; 


//use noisy_bevy::{simplex_noise_2d_seeded,fbm_simplex_2d_seeded};
//use noise::{NoiseFn, Perlin, Seedable};
pub type BinId = u32;

use crate::TerrainMeshData;

use super::terrain_noise::TerrainParameters;
use super::{terrain_noise, terrain_rtin, terrain_ui};


//use super::terrain_rtin;


#[derive(Component)]
pub struct TerrainMesh {
    pub positions: Vec<[f32; 3]>,
    uvs: Vec<[f32;2]>,
    normals: Vec<Vec3>,
    indices: Vec<u32>,
}

impl TerrainMesh {
    fn new() -> Self {
        Self {
            positions: Vec::new(),
            uvs: Vec::new(),
            normals:Vec::new(),
            indices: Vec::new(),
        }
    }
    
    
    /// RTIN
    pub fn build_RTIN_terrain(

        size:f32,
        subdivision_pow:u32,
        terrain_parameters:&TerrainParameters
    
    ) -> Mesh{ 

        let mut premesh = Self::new();

        let sub = (2 as u32).pow(subdivision_pow);

        let grid_size = sub*sub + 1;
        
        let number_of_levels = (grid_size-1).ilog2()*2;
        let last_level = number_of_levels - 1;

        let number_of_last_level_triangles = (grid_size-1)*(grid_size-1)*2;

        //noise
        let noisemap = &terrain_noise::NoiseMap::build(size,subdivision_pow, terrain_parameters);
        //terrain_ui::spawn_noise_image(commands, images, assets)

        let errors_vec = terrain_rtin::build_imperative_triangle_vec(grid_size,size,&terrain_parameters,1);

        let mut vertices_array_position = HashMap::<u32, usize>::new(); 
    
        let mut triangles = Vec::<BinId>::new();

        terrain_rtin::rtin_identify_triangles(
            grid_size,&errors_vec, &mut triangles,0,0.005);
        terrain_rtin::rtin_identify_triangles(
            grid_size,&errors_vec, &mut triangles,1,0.005);
    
        
        //VERY BAD NEED TO FIX !!
        //arbitrary aproximation overshoot the normal array size but sometimes undershoot -> oob -> crash 
        premesh.normals.resize( (triangles.len()/2+grid_size as usize) , Vec3::ZERO);


        for triangle_bin_id in triangles {

            let triangle_coords = terrain_rtin::get_triangle_coords(triangle_bin_id, grid_size);

            let new_vertices = &[triangle_coords[0], triangle_coords[1], triangle_coords[2]];

            let mut tri_vertex:[[f32;3];3] = [[0.,0.,0.],[0.,0.,0.],[0.,0.,0.]];
            let mut tri_vertex_index:[usize;3] = [0,0,0];
            let mut i = 0;
    
            for new_vertex in new_vertices {
                let vertex_id = new_vertex[1] * grid_size + new_vertex[0];


                //check if the vertex exist by its position and if not, create it
                let vertex_index = if vertices_array_position.contains_key(&vertex_id) {
                    let vertex_3d_index = *vertices_array_position.get(&vertex_id).unwrap();
                    tri_vertex[i] = [premesh.positions[vertex_3d_index][0],premesh.positions[vertex_3d_index][1],premesh.positions[vertex_3d_index][2]];
                    tri_vertex_index[i] = vertex_3d_index;
                    i += 1;
                    *vertices_array_position.get(&vertex_id).unwrap()
                } else {
                    
                    let new_vertex_index = premesh.positions.len();
                    vertices_array_position.insert(vertex_id, new_vertex_index);

        
                    let new_vertex_x = (new_vertex[0] as f32 / (grid_size-1) as f32)*size-size/2.0;
                    let new_vertex_z = (new_vertex[1] as f32 / (grid_size-1) as f32)*size-size/2.0;
                    
                    //let vertex_noise = Perlin::new(1).get(([(new_vertex_x as f64)*terrain_data.NOISE_SCALE as f64,(new_vertex_z as f64)*terrain_data.NOISE_SCALE as f64]))as f32;

                    //let new_vertex_hill = (((vertex_noise+terrain_data.HILL_VOLUME -1.0).clamp(0.0, 1.0))*terrain_data.CLIFF_STEEPNESS).clamp(-terrain_data.PLATEAU_HEIGHT,terrain_data.PLATEAU_HEIGHT);
                    //let new_vertex_pit = (((vertex_noise-terrain_data.PIT_VOLUME +1.0).clamp(-1.0, 0.0))*terrain_data.CLIFF_STEEPNESS).clamp(-terrain_data.PLATEAU_HEIGHT,terrain_data.PLATEAU_HEIGHT);
                    
                    
                    //here get the noise from terrain_noise.rs
                    //let new_vertex_y = new_vertex_hill + new_vertex_pit;
                    let new_vertex_y = terrain_noise::get_noise_value(new_vertex_x, new_vertex_z, terrain_parameters);
               

                    let new_vertex_3d = [
                        new_vertex_x,
                        new_vertex_y,
                        new_vertex_z
                    ];
               
                    tri_vertex[i] = new_vertex_3d;
                    tri_vertex_index[i] = new_vertex_index as usize;
                    i += 1;
           
                    premesh.positions.push(new_vertex_3d);
               
                    new_vertex_index
                };

                premesh.indices.push(vertex_index as u32);

            }
            //normal setup
            let normal = compute_normal(tri_vertex[0], tri_vertex[1], tri_vertex[2]);
            premesh.normals[tri_vertex_index[0]] += vec3(normal[0], normal[1], normal[2]).normalize();
            premesh.normals[tri_vertex_index[1]] += vec3(normal[0], normal[1], normal[2]).normalize();
            premesh.normals[tri_vertex_index[2]] += vec3(normal[0], normal[1], normal[2]).normalize();
              
        }
        //uv setup
        for i in (0..premesh.positions.len())
        {
        
            premesh.uvs.extend(vec![[premesh.positions[i][0] / size+0.5, premesh.positions[i][2] / size+0.5]]);
            
        }
        //normalize normals
        for i in (0..premesh.normals.len())
        {
        
            premesh.normals[i] = premesh.normals[i].normalize();

        }

        let mut mesh = Mesh::new(TriangleList, RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, premesh.positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, premesh.uvs);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, premesh.normals);
        mesh.insert_indices(Indices::U32(premesh.indices));
        mesh

    }

    //work in progress
    pub fn edit_terrain(

        mesh:&mut Mesh,
        terrain_parameters:&TerrainParameters
    
    ){ 

        let new_mesh = Self::build_RTIN_terrain(100.0, terrain_parameters.subdivision_pow, terrain_parameters);

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, new_mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap().clone());        
        mesh.insert_indices(new_mesh.indices().unwrap().clone());
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, new_mesh.attribute(Mesh::ATTRIBUTE_NORMAL).unwrap().clone());
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, new_mesh.attribute(Mesh::ATTRIBUTE_UV_0).unwrap().clone());

    }



}
 

fn compute_normal(v0: [f32; 3], v1: [f32; 3], v2: [f32; 3]) -> [f32; 3] {
    let edge1 = [
        v1[0] - v0[0],
        v1[1] - v0[1],
        v1[2] - v0[2]
    ];
    let edge2 = [
        v2[0] - v0[0],
        v2[1] - v0[1],
        v2[2] - v0[2]
    ];

    // Cross product
    [
        // edge1[1] * edge2[2] - edge1[2] * edge2[1],
        // edge1[2] * edge2[0] - edge1[0] * edge2[2],
        // edge1[0] * edge2[1] - edge1[1] * edge2[0]
        edge1[1] * edge2[2] - edge2[1] * edge1[2],
        edge1[2] * edge2[0] - edge2[2] * edge1[0],
        edge1[0] * edge2[1] - edge2[0] * edge1[1]
    ]
}
 
 

 
