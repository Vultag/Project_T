
use bevy::app::AppExit;
use bevy::{prelude::Mesh, ecs::component};
use bevy::render::mesh::Indices;
use bevy::math;

use std::any::Any;

//use crate::terrain_heightmap::{HeightMapU16, SubHeightMapU16};

use bevy::prelude::*;

use bevy::render::render_resource::PrimitiveTopology::TriangleList; 


use noisy_bevy::{simplex_noise_2d_seeded,fbm_simplex_2d_seeded};

#[derive(Component)]
pub struct TerrainMesh {
    pub positions: Vec<[f32; 3]>,
    uvs: Vec<[f32;2]>,
    normals: Vec<[f32;3]>,
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

        let step = size /subdivision as f32;

        for x in (0..subdivision+1).map(|x| x as f32 * step){
            for z in (0..subdivision+1).map(|x| x as f32 * step)
            {

                let fx = (x-size *0.5) ;
                let fz = (z-size *0.5) ;
                
                //println!("{}", x);
                
                // 2 type of noise scale : 1 for hills, 1 for pits
                const NOISE_SCALE:f32 = 0.05;
                const NOISE_HEIGHT_SCALE:f32 = 10.0; 
                const NOISE_CLAMPING:f32 = 2.0;
                const HILL_HEIGHT:f32 = 0.5;
                const PIT_DEPTH:f32 = 0.0;
                
                let fy_hill = ((fbm_simplex_2d_seeded(Vec2{x:fx*NOISE_SCALE,y:fz*NOISE_SCALE}, 3, 1.0, 1.0, 4.0)*1.28255+HILL_HEIGHT-1.0).clamp(0.0, 1.0)*NOISE_HEIGHT_SCALE).clamp(-NOISE_CLAMPING,NOISE_CLAMPING);
                let fy_pitt = ((fbm_simplex_2d_seeded(Vec2{x:fx*NOISE_SCALE,y:fz*NOISE_SCALE}, 3, 1.0, 1.0, 4.0)*1.28255-PIT_DEPTH+1.0).clamp(-1.0, 0.0)*NOISE_HEIGHT_SCALE).clamp(-NOISE_CLAMPING,NOISE_CLAMPING);
                
                //les hauteurs des vertices
                let fy = fy_hill + fy_pitt;
                
                let vertex_pos = [fx, fy, fz];
                //println!("{:?}",vertex_pos);
                premesh.positions.push(vertex_pos);

            }

            if(x!=0.0)
            {        
                for i in (0..subdivision)
                {
                    premesh.setup_quad((x/step) as u32,i,subdivision);
                }
            }
    
        }

        for i in (0..premesh.positions.len())
        {
        
            premesh.uvs.extend(vec![[premesh.positions[i][0] / size+0.5, premesh.positions[i][2] / size+0.5]]);
            
        }


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

        //Probably very defect for non flat quads
        //two triangle forming a quad could have different normals
        let normal_first = compute_normal(self.positions[start_idx as usize], self.positions[start_idx as usize+1], self.positions[start_idx as usize+sub as usize+1]);
        self.normals.extend([normal_first, normal_first, normal_first,normal_first]);

        
    }

    //work in progress
    pub fn edit_terrain(

        mesh:&mut Mesh,
        SUBDIVISION:u32,
        NOISE_SCALE:f32,
        NOISE_HEIGHT_SCALE:f32,
        NOISE_CLAMPING:f32,
        HILL_HEIGHT:f32,
        PIT_DEPTH:f32,
    
    ){ 

        //let mut premesh = Self::new();


        let positions = mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap();

        let mut new_positions = Vec::new();

        // println!("{}",positions.len());
        // println!("{}",positions.as_float3().unwrap().len());


        let pos_list = positions.as_float3().unwrap();


        println!("");
        println!("");
        println!("");

        for p in pos_list
        {
            
            let mut triangle:[f32;3] = p.to_owned();

            let (mut v_hill_height,mut v_pit_height);


            //((fbm_simplex_2d_seeded(Vec2{x:(x)*NOISE_SCALE,y:z*NOISE_SCALE}, 3, 1.0, 1.0, 4.0)*1.28255+HILL_HEIGHT-1.0).clamp(0.0, 1.0)*NOISE_HEIGHT_SCALE).clamp(-NOISE_CLAMPING,NOISE_CLAMPING); 

            v_hill_height = ((fbm_simplex_2d_seeded(Vec2{x:triangle[0]*NOISE_SCALE,y:triangle[2]*NOISE_SCALE}, 3, 1.0, 1.0, 4.0)*1.28255+HILL_HEIGHT-1.0).clamp(0.0, 1.0)*NOISE_HEIGHT_SCALE).clamp(-NOISE_CLAMPING,NOISE_CLAMPING); 

            v_pit_height = ((fbm_simplex_2d_seeded(Vec2{x:triangle[0]*NOISE_SCALE,y:triangle[2]*NOISE_SCALE}, 3, 1.0, 1.0, 4.0)*1.28255-PIT_DEPTH+1.0).clamp(-1.0, 0.0)*NOISE_HEIGHT_SCALE).clamp(-NOISE_CLAMPING,NOISE_CLAMPING);


            triangle = [triangle[0],v_hill_height+v_pit_height,triangle[2]];

            new_positions.push(triangle);

            
        }


        //println!("{:?}",new_positions[0]);
        //println!("{}",new_positions.len());
        
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, new_positions);

        //return mesh;

    }



}
 
 
//Normal calculation defect for quads
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
        edge1[1] * edge2[2] - edge1[2] * edge2[1],
        edge1[2] * edge2[0] - edge1[0] * edge2[2],
        edge1[0] * edge2[1] - edge1[1] * edge2[0]
    ]
}
 
  