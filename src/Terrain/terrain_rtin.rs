
//Counts trailing zero bits
use bitintr::Lzcnt;
//correspond to the binary identification for triangles within the binary tree
pub type BinId = u32;
//use noisy_bevy::{simplex_noise_2d_seeded,fbm_simplex_2d_seeded};
use noise::{NoiseFn, Perlin, Seedable};

use super::terrain_plugin::TerrainMeshData;

use super::terrain_noise::{self, NoiseMap, TerrainParameters};


#[derive(Eq, PartialEq, Debug)]
pub enum PartitionStep {
    //topR and botL correspond to the first two triangles
    TopRight,
    BottomLeft,
    // left and right correspond to all the subsequent triangles
    Left, 
    Right
}  

// get the corresponding index of the first triangle of a given level
// 1u32 is equal to 1
// !1u32 invert it in binary to reach a u32 max value -1 : 4294967294
// x &(!1u32) equates to rounding the number at the inferior even value
pub fn get_index_level_start(level: u32) -> u32 {
    ( (2 << level) - 1 ) & (!1u32)
}
pub fn get_triangle_children_indices(bin_id: u32) -> (u32, u32) {
    let (right_index, left_index) = get_triangle_children_bin_ids(bin_id);
    (bin_id_to_index(right_index), bin_id_to_index(left_index))
}
pub fn get_triangle_children_bin_ids(bin_id: u32) -> (u32, u32) {
    let level = bin_id_to_level(bin_id);
    let right_bin_id = 
        bin_id + (1 << (level+2) ) - (1 << (level+1) );
    let left_bin_id = 
        bin_id + (1 << (level+2) );
    (right_bin_id, left_bin_id)
}
pub fn bin_id_to_index(bin_id: u32) -> u32 {
    let level = bin_id_to_level(bin_id);
    let index_level_start = get_index_level_start(level);
    let index_in_level = bin_id_to_index_in_level(bin_id);
    
    index_level_start + index_in_level
}
pub fn bin_id_to_level(bin_id: u32) -> u32 {
    (32-bin_id.lzcnt()) - 2
}
pub fn bin_id_to_index_in_level(bin_id: u32) -> u32 {
    bin_id - (1 << ((32-bin_id.lzcnt())-1) )
}
pub fn index_to_bin_id(index: u32) -> u32 {
    let mut level = 0;
    let mut index_level_start = 0;

    for i in 0..32 {
        let new_index_level_start = get_index_level_start(i);
        if index >= new_index_level_start {
            level = i;
            index_level_start = new_index_level_start;
        } else {
            break;
        }
    }

    ( 1 << (level+1) ) + (index - index_level_start)
}
//pas utilise ?
pub fn get_opposing_vertex(tri_vertices:[[u32; 2];3])->[u32; 2]
{
    let opposing_v = 
    [
        tri_vertices[2][0]+((tri_vertices[0][0]-tri_vertices[2][0])+(tri_vertices[1][0]-tri_vertices[2][0])),
        tri_vertices[2][1]+((tri_vertices[0][1]-tri_vertices[2][1])+(tri_vertices[1][1]-tri_vertices[2][1]))
    ];
    opposing_v
}
pub fn get_triangle_coords(bin_id: u32, grid_size: u32) -> [[u32; 2];3] {
    let mut a = [0; 2];
    let mut b = [0; 2];
    let mut c = [0; 2];


    for step in bin_id_to_partition_steps(bin_id) {
        match step {
            PartitionStep::TopRight => {
                // north east right-angle corner
                a[0] = 0; 
                a[1] = 0; 
                b[0] = grid_size-1; 
                b[1] = grid_size-1; 
                c[0] = grid_size-1; 
                c[1] = 0; 
            }
            PartitionStep::BottomLeft => {
                // north east right-angle corner
                a[0] = grid_size-1; 
                a[1] = grid_size-1; 
                b[0] = 0; 
                b[1] = 0; 
                c[0] = 0; 
                c[1] = grid_size-1; 

            }
            PartitionStep::Left => {
                let (new_a, new_b, new_c) = (
                    c, 
                    a, 
                    [(a[0]+b[0])/2,(a[1]+b[1])/2]
                );
                a = new_a;
                b = new_b;
                c = new_c;
            }
            PartitionStep::Right => {
                let (new_a, new_b, new_c) = (
                    b, 
                    c, 
                    [(a[0]+b[0])/2,(a[1]+b[1])/2]
                );
                a = new_a;
                b = new_b;
                c = new_c;
            }
        }
    }

    [a, b, c]
}
pub fn bin_id_to_partition_steps(bin_id: u32) -> Vec::<PartitionStep> {
    let mut steps = Vec::new();
    let triangle_level = bin_id_to_level(bin_id);

    if bin_id & 1 > 0 {
        steps.push(PartitionStep::TopRight);
    } else {
        steps.push(PartitionStep::BottomLeft);
    }

    for i in 1..(triangle_level+1) {
    if bin_id & (1 << i) > 0 {
        steps.push(PartitionStep::Left);
    } else {
        steps.push(PartitionStep::Right);
    }
    }

    steps
}
pub fn triangle_errors_vec_index(bin_id: BinId, grid_size: u32) -> usize {
    let triangle_midpoint = pixel_coords_for_triangle_mid_point(
        bin_id, grid_size);
    let midpoint_error_vec_index = 
        triangle_midpoint[1] * grid_size +
        triangle_midpoint[0];

    midpoint_error_vec_index as usize
}
pub fn pixel_coords_for_triangle_mid_point(bin_id: u32, grid_size: u32) -> [u32;2] {
    let triangle_coords = get_triangle_coords(bin_id, grid_size);
    let mid_point = [(triangle_coords[0][0]+triangle_coords[1][0])/2,(triangle_coords[0][1]+triangle_coords[1][1])/2];

    mid_point
}
pub fn count_bits(value:u32)->u32{
    //probably not the most optimized way to do it
    let mut bit_n = value;
    let mut count = 0;
    while bit_n > 0
    {
        count+=(bit_n&1);
        bit_n = bit_n >>1;
    }
    count
}
//FAILED ATTEMPT AT FINDING OPPOSITE TRIANGLE BIN ID
pub fn get_opposite_triangle_bin_id(bin_id: u32,leaf_lvl:u32)-> u32
{

    let mut opposite_bin_id: u32 = 0;

    let number_of_ones:u32 = count_bits(bin_id);
    let number_of_ones_past_3:u32 = count_bits(((2 as u32).pow(leaf_lvl-3)-1)&bin_id);

    ///See read_me at opposite bin id for more information

    ///Case 1 : invert first two digit
    ///-> distinctive motif of id: for first two digit, both 1 and 0 when leaf lvl is even 
    /// and either 0 or 1 when leaf lvl is odd
    ///  
    ///Case 2 : invert all digit
    ///-> distinctive motif of id: even or odd number of 1s alternate with leaf lvl
    /// 
    ///Case 3 : invert all digit exept last one
    /// -> distinctive motif of id: even or odd number of 1s alternate with leaf lvl
    /// offested by one with Case 2
    /// 
    //NON
    ///Case 4 : invert all digit exept two last
    /// -> distinctive motif of id: the first two digit alternate from 11 to 00 with the leaf lvl 
    /// 
    /// CASE 5 TO DO
    /// 
    ///difficult readability because made branchless



    //check if leaf lvl is even or odd
    //if((leaf_lvl&1)==1)
    {

    //Could set in one line but it would hurt readability  further more
    //Case 1
    opposite_bin_id = (3*(((bin_id&1)^((bin_id>>1)&1))^1))^bin_id;
    //Case 2 and 3
    opposite_bin_id = (((2 as u32).pow(leaf_lvl-((number_of_ones&1)^(((leaf_lvl+1)/2)&1)^((bin_id>>(leaf_lvl-1))^1)))-1)*((bin_id&1)^((bin_id>>1)&1))*(((number_of_ones_past_3)&1)^(((leaf_lvl-1)/2)&1)^((bin_id>>(leaf_lvl-1))^1))*((leaf_lvl)&1))^opposite_bin_id;
    //Case 1.5
    opposite_bin_id = (((2 as u32).pow(leaf_lvl-5)-1)*(((number_of_ones_past_3)&1)^(((leaf_lvl+1)/2)&1)^((bin_id>>(leaf_lvl-1))^1))*((bin_id&1)^((bin_id>>1)&1))*((leaf_lvl)&1))^opposite_bin_id;
    //Case 4 and 5 (5 is meant to be discarded)
    opposite_bin_id = (((((2 as u32).pow(leaf_lvl))-1)^(3<<leaf_lvl-2))*((bin_id&1)^((bin_id>>1)&1))*((leaf_lvl-1)&1))^opposite_bin_id;

    }
    //Case 5
    //opposite_bin_id = opposite_bin_id^((((number_of_ones+((bin_id>>leaf_lvl-1)^((bin_id>>leaf_lvl-2)&1)))&1)^(((leaf_lvl)/2)&1))*((((2 as u32).pow(leaf_lvl))-1)&(1<<leaf_lvl-1)));
    

    opposite_bin_id
}

pub fn rtin_identify_triangles(
    //add noise?
    grid_size:u32,
    errors_vec: &Vec::<f32>,
    triangles: &mut Vec::<u32>, 
    triangle_index: u32, 
    error_threshold: f32)  {
    
    let triangle_bin_id = index_to_bin_id(triangle_index);

    let (right_child_index, left_child_index) = 
    get_triangle_children_indices(triangle_bin_id);

    let side = grid_size-1;
    let number_of_last_level_triangles = side*side*2;
    let number_of_triangles = side * side * 2 - 2 + number_of_last_level_triangles;

    let has_children = right_child_index < number_of_triangles;

    let leaf_triangle = !has_children;

    let this_triangle_errors_vec_index = triangle_errors_vec_index(triangle_bin_id, grid_size);

    let valid_threshold = errors_vec[this_triangle_errors_vec_index] <= error_threshold;


    //println!("{}",errors_vec[this_triangle_errors_vec_index]);

    if valid_threshold || leaf_triangle {
        triangles.push(triangle_bin_id);
    } else {
        rtin_identify_triangles(
            grid_size,errors_vec, triangles, left_child_index, error_threshold);
        rtin_identify_triangles(
            grid_size,errors_vec, triangles, right_child_index, error_threshold);
    }
}


pub fn build_imperative_triangle_vec(
    
    coords:[u32;2],
    grid_size:u32,
    //size:f32,
    //noisemap:&NoiseMap,
    terrain_parameters:&TerrainParameters,
    noise_seed: u32

) -> Vec::<f32> {

    const size:f32 = 20.0;

    let number_of_triangles = (grid_size-1) * (grid_size-1) * 2 - 2;

    let number_of_levels = (grid_size-1).ilog2()*2;
    
    let last_level = number_of_levels - 1;

    let last_level_index_start = get_index_level_start(last_level);
    
    let mut height_diff_vec = Vec::new();
    height_diff_vec.resize( (grid_size*grid_size) as usize, 0.0);

    for triangle_index in (0..number_of_triangles).rev() {

        let triangle_bin_id = index_to_bin_id(triangle_index);

        let midpoint = pixel_coords_for_triangle_mid_point(triangle_bin_id, grid_size);

        let triangle_coords = get_triangle_coords(triangle_bin_id, grid_size);
        
        //attempt with noise map
        // let v0_height = noisemap.get_value(triangle_coords[0][0] as usize, triangle_coords[0][1] as usize);
        // let v1_height = noisemap.get_value(triangle_coords[1][0] as usize, triangle_coords[1][1] as usize);
        // let midpoint_height = 0.0;//noisemap.get_value(midpoint[0] as usize, midpoint[1] as usize);
      

        let v0_x = (triangle_coords[0][0] as f32 / (grid_size-1) as f32)*size-size/2.0;
        let v0_z = (triangle_coords[0][1] as f32 / (grid_size-1) as f32)*size-size/2.0;
        let v1_x = (triangle_coords[1][0] as f32 / (grid_size-1) as f32)*size-size/2.0;
        let v1_z = (triangle_coords[1][1] as f32 / (grid_size-1) as f32)*size-size/2.0;
        let vmid_x = (midpoint[0] as f32 / (grid_size-1) as f32)*size-size/2.0;
        let vmid_z = (midpoint[1] as f32 / (grid_size-1) as f32)*size-size/2.0;

        // let v0_height = terrain_noise::get_noise_value(v0_x, v0_z, terrain_parameters);
        // let v1_height = terrain_noise::get_noise_value(v1_x, v1_z, terrain_parameters);
        // let midpoint_height = terrain_noise::get_noise_value(vmid_x, vmid_z, terrain_parameters);
        let v0_height = terrain_noise::get_noise_value(triangle_coords[0][0]+coords[0], triangle_coords[0][1]+coords[1], terrain_parameters);
        let v1_height = terrain_noise::get_noise_value(triangle_coords[1][0]+coords[0], triangle_coords[1][1]+coords[1], terrain_parameters);
        let midpoint_height = terrain_noise::get_noise_value(midpoint[0]+coords[0], midpoint[1]+coords[1], terrain_parameters);
            

        // let v0_noise = Perlin::new(1).get(([(v0_x as f64)*terrain_data.NOISE_SCALE as f64,(v0_z as f64)*terrain_data.NOISE_SCALE as f64]))as f32;
        // let v1_noise = Perlin::new(1).get(([(v1_x as f64)*terrain_data.NOISE_SCALE as f64,(v1_z as f64)*terrain_data.NOISE_SCALE as f64]))as f32;
        // let vmid_noise = Perlin::new(1).get(([(vmid_x as f64)*terrain_data.NOISE_SCALE as f64,(vmid_z as f64)*terrain_data.NOISE_SCALE as f64]))as f32;
        

        // let v0_hill = (((v0_noise+terrain_data.HILL_VOLUME -1.0).clamp(0.0, 1.0))*terrain_data.CLIFF_STEEPNESS).clamp(-terrain_data.PLATEAU_HEIGHT,terrain_data.PLATEAU_HEIGHT);
        // let v1_hill = (((v1_noise+terrain_data.HILL_VOLUME -1.0).clamp(0.0, 1.0))*terrain_data.CLIFF_STEEPNESS).clamp(-terrain_data.PLATEAU_HEIGHT,terrain_data.PLATEAU_HEIGHT);
        // let v0_pit = (((v0_noise-terrain_data.PIT_VOLUME +1.0).clamp(-1.0, 0.0))*terrain_data.CLIFF_STEEPNESS).clamp(-terrain_data.PLATEAU_HEIGHT,terrain_data.PLATEAU_HEIGHT);
        // let v1_pit = (((v1_noise-terrain_data.PIT_VOLUME +1.0).clamp(-1.0, 0.0))*terrain_data.CLIFF_STEEPNESS).clamp(-terrain_data.PLATEAU_HEIGHT,terrain_data.PLATEAU_HEIGHT);
        // let midpoint_hill = (((vmid_noise+terrain_data.HILL_VOLUME -1.0).clamp(0.0, 1.0))*terrain_data.CLIFF_STEEPNESS).clamp(-terrain_data.PLATEAU_HEIGHT,terrain_data.PLATEAU_HEIGHT);
        // let midpoint_pit = (((vmid_noise-terrain_data.PIT_VOLUME +1.0).clamp(-1.0, 0.0))*terrain_data.CLIFF_STEEPNESS).clamp(-terrain_data.PLATEAU_HEIGHT,terrain_data.PLATEAU_HEIGHT);
        

        // let v0_height = v0_hill + v0_pit;
        // let v1_height = v1_hill + v1_pit;
        // let midpoint_height = midpoint_hill + midpoint_pit;
        
     
        let midpoint_interpolated = (v0_height+v1_height)/2.0; 

        let this_triangle_height_diff = (midpoint_interpolated - midpoint_height).abs();

        
        
        let this_triangle_mid_point_error_vec_index = 
            triangle_errors_vec_index(triangle_bin_id, grid_size);

        if triangle_index >= last_level_index_start {
            height_diff_vec[this_triangle_mid_point_error_vec_index] = this_triangle_height_diff;
        } else {
            let (right_child_bin_id, left_child_bin_id) = 
                get_triangle_children_bin_ids(triangle_bin_id);

            let right_errors_vec_index = 
                triangle_errors_vec_index(right_child_bin_id, grid_size);
            let left_errors_vec_index = 
                triangle_errors_vec_index(left_child_bin_id, grid_size);
                
            let prev_error = height_diff_vec[this_triangle_mid_point_error_vec_index];
            let right_error = height_diff_vec[right_errors_vec_index];
            let left_error = height_diff_vec[left_errors_vec_index];

            height_diff_vec[this_triangle_mid_point_error_vec_index] = 
                prev_error.max(left_error).max(right_error).max(this_triangle_height_diff);
        }
       
    }

    height_diff_vec
}