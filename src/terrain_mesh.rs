
use bevy::{prelude::Mesh, ecs::component};
use bevy::render::mesh::Indices;

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

    fn add_triangle(&mut self, positions: [[f32; 3]; 3], uvs: [[f32; 2]; 3]) {
        // Add vertices and indices
        for psn in &positions {
         //   println!("psn {:?}", psn);
            self.positions.push(*psn);
        }
        let start_idx = self.positions.len() as u32 - 3;
        self.indices.extend (&[start_idx, start_idx + 1, start_idx + 2]);   
        
        //stubbed in for now ... 
        let normal = compute_normal(positions[0], positions[1], positions[2]);
        self.normals.extend([normal, normal, normal]);
        
        
        self.uvs.extend( uvs ) ; 
    }
    
     
    
    pub fn build_terrain(

        //if I want to do a rectangle
        //size:[u32;2],
        size:f32,
        subdivision:u32
    
    ) -> Mesh{ 
            

        let mut premesh = Self::new();

        let step = size /subdivision as f32;

        //println!("{}",step);

        //marche sauf que les decimal rend le mesh plus petit qu il ne devrait etre
        for x in (0..subdivision).map(|x| x as f32 * step){
            for z in (0..subdivision).map(|x| x as f32 * step){
                 

                //println!("{}",x);

                //let fx = (x*(size[0]/subdivision)) as f32;
                //let fz = (z*(size[1]/subdivision)) as f32;

                //center le mesh around its origin
                let fx = (x-size *0.5) ;
                let fz = (z-size *0.5) ;


                const NOISE_SCALE:f32 = 0.05;
                const NOISE_HEIGHT_SCALE:f32 = 10.0; 
                const NOISE_CLAMPING:f32 = 2.0;
                const HILL_HEIGHT:f32 = 0.5;
                const PIT_DEPTH:f32 = 0.0;

                let lb_hill = ((fbm_simplex_2d_seeded(Vec2{x:(fx)*NOISE_SCALE,y:fz*NOISE_SCALE}, 3, 1.0, 1.0, 4.0)*1.28255+HILL_HEIGHT-1.0).clamp(0.0, 1.0)*NOISE_HEIGHT_SCALE).clamp(-NOISE_CLAMPING,NOISE_CLAMPING); 
                let lf_hill = ((fbm_simplex_2d_seeded(Vec2{x:(fx)*NOISE_SCALE,y:(fz+step)*NOISE_SCALE}, 3, 1.0, 1.0, 4.0)*1.28255+HILL_HEIGHT-1.0).clamp(0.0, 1.0)*NOISE_HEIGHT_SCALE).clamp(-NOISE_CLAMPING,NOISE_CLAMPING); 
                let rb_hill = ((fbm_simplex_2d_seeded(Vec2{x:(fx+step)*NOISE_SCALE,y:(fz)*NOISE_SCALE}, 3, 1.0, 1.0, 4.0)*1.28255+HILL_HEIGHT-1.0).clamp(0.0, 1.0)*NOISE_HEIGHT_SCALE).clamp(-NOISE_CLAMPING,NOISE_CLAMPING);
                let rf_hill = ((fbm_simplex_2d_seeded(Vec2{x:(fx+step)*NOISE_SCALE,y:(fz+step)*NOISE_SCALE}, 3, 1.0, 1.0, 4.0)*1.28255+HILL_HEIGHT-1.0).clamp(0.0, 1.0)*NOISE_HEIGHT_SCALE).clamp(-NOISE_CLAMPING,NOISE_CLAMPING);

                let lb_pit = ((fbm_simplex_2d_seeded(Vec2{x:(fx)*NOISE_SCALE,y:fz*NOISE_SCALE}, 3, 1.0, 1.0, 4.0)*1.28255-PIT_DEPTH+1.0).clamp(-1.0, 0.0)*NOISE_HEIGHT_SCALE).clamp(-NOISE_CLAMPING,NOISE_CLAMPING); 
                let lf_pit = ((fbm_simplex_2d_seeded(Vec2{x:(fx)*NOISE_SCALE,y:(fz+step)*NOISE_SCALE}, 3, 1.0, 1.0, 4.0)*1.28255-PIT_DEPTH+1.0).clamp(-1.0, 0.0)*NOISE_HEIGHT_SCALE).clamp(-NOISE_CLAMPING,NOISE_CLAMPING); 
                let rb_pit = ((fbm_simplex_2d_seeded(Vec2{x:(fx+step)*NOISE_SCALE,y:(fz)*NOISE_SCALE}, 3, 1.0, 1.0, 4.0)*1.28255-PIT_DEPTH+1.0).clamp(-1.0, 0.0)*NOISE_HEIGHT_SCALE).clamp(-NOISE_CLAMPING,NOISE_CLAMPING);
                let rf_pit = ((fbm_simplex_2d_seeded(Vec2{x:(fx+step)*NOISE_SCALE,y:(fz+step)*NOISE_SCALE}, 3, 1.0, 1.0, 4.0)*1.28255-PIT_DEPTH+1.0).clamp(-1.0, 0.0)*NOISE_HEIGHT_SCALE).clamp(-NOISE_CLAMPING,NOISE_CLAMPING);



                
                //les hauteurs des vertices
                let lb = lb_hill+lb_pit;
                let lf = lf_hill+lf_pit;
                let rb = rb_hill+rb_pit;
                let rf = rf_hill+rf_pit;
 
                // let lb = ((fbm_simplex_2d_seeded(Vec2{x:(x)*NOISE_SCALE,y:z*NOISE_SCALE}, 3, 1.0, 1.0, 4.0)*1.28255+HILL_HEIGHT-1.0).clamp(0.0, 1.0)*NOISE_HEIGHT_SCALE).clamp(-NOISE_CLAMPING,NOISE_CLAMPING); 
                // let lf = ((fbm_simplex_2d_seeded(Vec2{x:(x)*NOISE_SCALE,y:(z+step)*NOISE_SCALE}, 3, 1.0, 1.0, 4.0)*1.28255+HILL_HEIGHT-1.0).clamp(0.0, 1.0)*NOISE_HEIGHT_SCALE).clamp(-NOISE_CLAMPING,NOISE_CLAMPING); 
                // let rb = ((fbm_simplex_2d_seeded(Vec2{x:(x+step)*NOISE_SCALE,y:(z)*NOISE_SCALE}, 3, 1.0, 1.0, 4.0)*1.28255+HILL_HEIGHT-1.0).clamp(0.0, 1.0)*NOISE_HEIGHT_SCALE).clamp(-NOISE_CLAMPING,NOISE_CLAMPING);
                // let rf = ((fbm_simplex_2d_seeded(Vec2{x:(x+step)*NOISE_SCALE,y:(z+step)*NOISE_SCALE}, 3, 1.0, 1.0, 4.0)*1.28255+HILL_HEIGHT-1.0).clamp(0.0, 1.0)*NOISE_HEIGHT_SCALE).clamp(-NOISE_CLAMPING,NOISE_CLAMPING);

                //UV setting not at the best currently
                let test_bounds:[[f32 ; 2]  ;2 ] = [[0.0,0.0],[1.0,1.0]];
                
                let uv_lb = compute_uv(x as f32, z as f32, test_bounds, size);
                let uv_rb = compute_uv(x as f32+step, z as f32, test_bounds, size);
                let uv_rf = compute_uv(x as f32+step, z as f32+step, test_bounds, size);
                let uv_lf = compute_uv(x as f32, z as f32+step, test_bounds, size);
                //REMPLACE WHEN ABLE TO REPEATE THE TEXTURE INSTEAD OF EXTEND
                // let uv_lb = compute_uv(x as f32, z as f32, test_bounds, size*8.0/subdivision as f32);
                // let uv_rb = compute_uv(x as f32+step, z as f32, test_bounds, size*8.0/subdivision as f32);
                // let uv_rf = compute_uv(x as f32+step, z as f32+step, test_bounds, size*8.0/subdivision as f32);
                // let uv_lf = compute_uv(x as f32, z as f32+step, test_bounds, size*8.0/subdivision as f32);


                //println!("{}",fx);



                let left_back = [fx, lb, fz];
                let right_back = [(fx+step) , rb, fz];
                let right_front = [(fx+step) , rf, (fz+step)];
                let left_front = [fx , lf, (fz+step)];
    

                premesh.add_triangle([left_front, right_back, left_back], [uv_lf, uv_rb, uv_lb]);
                premesh.add_triangle([right_front, right_back, left_front], [uv_rf, uv_rb, uv_lf]);
            }
        }




        println!("{:?}",premesh.positions[0]);


            let mut mesh = Mesh::new( TriangleList );
            mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, premesh.positions);
            mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, premesh.uvs);
            mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, premesh.normals);
            mesh.set_indices(Some(Indices::U32(premesh.indices)));
            mesh
    
    }

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


        println!("{:?}",new_positions[0]);
        println!("{}",new_positions.len());
        
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, new_positions);

        //return mesh;

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
        edge1[1] * edge2[2] - edge1[2] * edge2[1],
        edge1[2] * edge2[0] - edge1[0] * edge2[2],
        edge1[0] * edge2[1] - edge1[1] * edge2[0]
    ]
}
 
 //is this right !!?? 
 fn compute_uv(x: f32, y: f32, bounds: [[f32; 2]; 2], texture_dimensions: f32) -> [f32; 2] {
     
     let start_bounds_x = bounds[0][0];
     let end_bounds_x = bounds[1][0];
     
     let start_bounds_y = bounds[0][1];
     let end_bounds_y = bounds[1][1];
     
     //x and y are the origin coords 
     
    let uv_worldspace = [
        (x ) / (end_bounds_x - start_bounds_x),
        (y ) / (end_bounds_y - start_bounds_y)
    ];
    
    let uv = [
        uv_worldspace[0] / texture_dimensions,
        uv_worldspace[1] / texture_dimensions,  
        
    ];
    
   // println!("uv {:?}", uv);
     
    
    uv
}
  