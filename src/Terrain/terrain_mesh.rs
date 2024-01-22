
use bevy::app::AppExit;
use bevy::{prelude::Mesh, ecs::component};
use bevy::render::mesh::Indices;
use bevy::math::{self, vec3};
use bevy::prelude::Vec3;

use std::any::Any;

use bevy::prelude::*;

use bevy::render::render_resource::PrimitiveTopology::TriangleList; 


use noisy_bevy::{simplex_noise_2d_seeded,fbm_simplex_2d_seeded};
use noise::{NoiseFn, Perlin, Seedable};


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
    
    pub fn build_terrain(

        //if I want to do a rectangle
        //size:[u32;2],
        size:f32,
        subdivision:u32
    
    ) -> Mesh{ 
            
        let mut premesh = Self::new();
        //rezise the normal vector for calculated normal insertion
        premesh.normals.resize(((subdivision+1)*(subdivision+1)) as usize, Vec3::ZERO);


        let step = size /subdivision as f32;

        for x in (0..subdivision+1).map(|x| x as f32 * step){
            for z in (0..subdivision+1).map(|x| x as f32 * step)
            {

                let fx = (x-size *0.5) ;
                let fz = (z-size *0.5) ;
                
                
                // 2 type of noise scale : 1 for hills, 1 for pits
                const NOISE_SCALE:f32 = 0.05;
                const CLIFF_STEEPNESS:f32 = 15.0; 
                const PLATEAU_HEIGHT:f32 = 2.0;
                const HILL_VOLUME:f32 = 0.5;
                const PIT_VOLUME:f32 = 0.5;

                
                let noise = Perlin::new(1).get(([(fx as f64)*NOISE_SCALE as f64,(fz as f64)*NOISE_SCALE as f64]))as f32;
                
                let fy_hill = (((noise+HILL_VOLUME -1.0).clamp(0.0, 1.0))*CLIFF_STEEPNESS).clamp(-PLATEAU_HEIGHT,PLATEAU_HEIGHT);
                let fy_pitt = (((noise-PIT_VOLUME+1.0).clamp(-1.0, 0.0))*CLIFF_STEEPNESS).clamp(-PLATEAU_HEIGHT,PLATEAU_HEIGHT);
                
                //Verticies height
                let fy = fy_hill + fy_pitt;
                
                let vertex_pos = [fx, fy as f32, fz];
                premesh.positions.push(vertex_pos);

            }
            //setup quads indices after the first two collum
            if(x!=0.0)
            {        
                for i in (0..subdivision)
                {
                    premesh.setup_quad((x/step) as u32,i,subdivision);
                }
            }
    
        }

        //necessary ?
        for i in (0..premesh.positions.len())
        {
        
            premesh.uvs.extend(vec![[premesh.positions[i][0] / size+0.5, premesh.positions[i][2] / size+0.5]]);
            
        }

        premesh.calculate_normals(subdivision);

        let mut mesh = Mesh::new( TriangleList );
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, premesh.positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, premesh.uvs);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, premesh.normals);
        mesh.set_indices(Some(Indices::U32(premesh.indices)));
        mesh

    }

    fn setup_quad(&mut self,x:u32,z:u32,sub:u32) 
    {
  
        let start_idx = z+(x-1)*(sub+1);

        self.indices.extend (&[start_idx, start_idx+1, start_idx+sub+1]); 
        self.indices.extend (&[start_idx+sub+2, start_idx+sub+1, start_idx+1]); 
  
    }

    fn calculate_normals(&mut self,sub:u32)
    {
        let sub = sub as usize;

        for i in 0..self.positions.len()-((sub+1)as usize)-1
        {
        
                //calculate normal for both triangle making up the quad
                let normals_first_tri = compute_normal(self.positions[i], self.positions[i+1], self.positions[i+sub+1]);
                let normals_second_tri = compute_normal(self.positions[i+sub+2], self.positions[i+sub+1], self.positions[i+1]);
                //combine the two normals shared by the two triangles
               let middle_normal_merge = vec3(normals_first_tri[0]+normals_second_tri[0],normals_first_tri[1]+normals_second_tri[1],normals_first_tri[2]+normals_second_tri[2]).normalize();
        
            
        
                self.normals[i] += vec3(normals_first_tri[0], normals_first_tri[1], normals_first_tri[2]).normalize();
                self.normals[i+1] += vec3(middle_normal_merge[0], middle_normal_merge[1], middle_normal_merge[2]).normalize();
                self.normals[i+sub+1] += vec3(middle_normal_merge[0], middle_normal_merge[1], middle_normal_merge[2]).normalize();
                self.normals[i+sub+2] += vec3(normals_second_tri[0], normals_second_tri[1], normals_second_tri[2]).normalize();
        
                self.normals[i] = self.normals[i].normalize();
                self.normals[i+1] = self.normals[i+1].normalize();
                self.normals[i+sub+1] = self.normals[i+sub+1].normalize();
                self.normals[i+sub+2] = self.normals[i+sub+2].normalize();
        
        }
    }

    //work in progress
    pub fn edit_terrain(

        mesh:&mut Mesh,
        SUBDIVISION:u32,
        NOISE_SCALE:f32,
        CLIFF_STEEPNESS:f32,
        PLATEAU_HEIGHT:f32,
        HILL_VOLUME:f32,
        PIT_VOLUME:f32,
    
    ){ 

        let mut premesh = Self::new();
        //rezise the normal vector for calculated normal insertion
        premesh.normals.resize(((SUBDIVISION+1)*(SUBDIVISION+1)) as usize, Vec3::ZERO);

        let positions = mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap();

        let pos_list = positions.as_float3().unwrap();

        for p in pos_list
        {
            
            let mut triangle:[f32;3] = p.to_owned();

            let (mut v_hill_height,mut v_pit_height);

            let noise = Perlin::new(1).get(([(triangle[0] as f64)*NOISE_SCALE as f64,(triangle[2] as f64)*NOISE_SCALE as f64]))as f32;
                
            v_hill_height = (((noise+HILL_VOLUME -1.0).clamp(0.0, 1.0))*CLIFF_STEEPNESS).clamp(-PLATEAU_HEIGHT,PLATEAU_HEIGHT);

            v_pit_height = (((noise-PIT_VOLUME+1.0).clamp(-1.0, 0.0))*CLIFF_STEEPNESS).clamp(-PLATEAU_HEIGHT,PLATEAU_HEIGHT);


            triangle = [triangle[0],v_hill_height + v_pit_height,triangle[2]];

            premesh.positions.push(triangle);
            
        }

        
        //println!("{:?}",premesh.positions);
        premesh.calculate_normals(SUBDIVISION);


        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, premesh.positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, premesh.normals)

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
 
 

 
