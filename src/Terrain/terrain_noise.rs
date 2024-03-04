
use crate::TerrainMeshData;
use bevy::{asset::Handle, ecs::{component::Component, system::Resource}, render::{render_asset::RenderAssetUsages, texture::Image}};
use noise::{NoiseFn, Perlin, Seedable};
use wgpu::{TextureDimension, TextureFormat};
use std::collections::HashMap;
use crate::Extent3d;

#[derive(Component)]
pub struct NoiseImageData {
    pub image_handle:Handle<Image>
}

#[derive(Default,Resource)]
pub struct TerrainParameters {
    pub size:f32,
    pub subdivision_pow:u32,
    pub NOISE_SCALE:f32,
    pub CLIFF_STEEPNESS:f32,
    pub PLATEAU_HEIGHT:f32,
    pub HILL_VOLUME:f32,
    pub PIT_VOLUME:f32,
    pub perlin:Perlin,
 }

//#[derive(Resource)]
pub struct NoiseMap {
    size: u32,
    map: Vec<f32>
}



impl NoiseMap {


    pub fn build(size:f32,subdiv_pow: u32,terrain_parameters:&TerrainParameters) -> Self {


        let sub = (2 as u32).pow(terrain_parameters.subdivision_pow);

        let grid_size = sub*sub;
        let map_size = grid_size;
        let mut map_values = Vec::new();

        for z in (0..grid_size)
        {
            for x in (0..grid_size)
            {

                let x_ajusted = (x as f32 / (grid_size-1) as f32)*size-size/2.0;
                let z_ajusted = (z as f32 / (grid_size-1) as f32)*size-size/2.0;
                
                //let noise = Perlin::new(1).get(([(x_ajusted as f64)*terrain_parameters.NOISE_SCALE as f64,(z_ajusted as f64)*terrain_parameters.NOISE_SCALE as f64]))as f32;
                let noise = terrain_parameters.perlin.get(([(x_ajusted as f64)*terrain_parameters.NOISE_SCALE as f64,(z_ajusted as f64)*terrain_parameters.NOISE_SCALE as f64]))as f32;
                    
                let hill_value = (((noise+terrain_parameters.HILL_VOLUME -1.0).clamp(0.0, 1.0))*terrain_parameters.CLIFF_STEEPNESS).clamp(-terrain_parameters.PLATEAU_HEIGHT,terrain_parameters.PLATEAU_HEIGHT);
                let pit_value = (((noise-terrain_parameters.PIT_VOLUME +1.0).clamp(-1.0, 0.0))*terrain_parameters.CLIFF_STEEPNESS).clamp(-terrain_parameters.PLATEAU_HEIGHT,terrain_parameters.PLATEAU_HEIGHT);
                let value = hill_value + pit_value;

                map_values.push(value);

            }
        }
    
        Self {
            size: map_size,
            map: map_values,
        }
    }


    // pub fn get_value(&self, x: usize, z: usize) -> &f32 {

    //     //(self.map.get(&(x + z * self.size))).unwrap()
    //     &(self.map[0])
    // }


    pub fn write_to_image(&self)->Image {
        
        let mut pixels: Vec<u8> = Vec::new();
    
        for i in &self.map {

            let value = (255.0-(127.0-((i) * 127.0))) as u8;
                
            pixels.push(value); // Red channel
            pixels.push(value); // Green channel
            pixels.push(value); // Blue channel
            pixels.push(255);   // Alpha channel (fully opaque)
            

        }
    
    
        let image = Image::new_fill(
            Extent3d {
                width: (self.size as u32),
                height: (self.size as u32),
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            &pixels,
            TextureFormat::Rgba8UnormSrgb,
            RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
        );
        
        image
    }


}


//temporary, yet to find the best way
pub fn get_noise_value(x: f32, z: f32,terrain_parameters:&TerrainParameters) -> f32 {

    let noise = terrain_parameters.perlin.get(([(x as f64)*terrain_parameters.NOISE_SCALE as f64,(z as f64)*terrain_parameters.NOISE_SCALE as f64]))as f32;
                   
    let hill_value = (((noise+terrain_parameters.HILL_VOLUME -1.0).clamp(0.0, 1.0))*terrain_parameters.CLIFF_STEEPNESS).clamp(-terrain_parameters.PLATEAU_HEIGHT,terrain_parameters.PLATEAU_HEIGHT);
    let pit_value = (((noise-terrain_parameters.PIT_VOLUME +1.0).clamp(-1.0, 0.0))*terrain_parameters.CLIFF_STEEPNESS).clamp(-terrain_parameters.PLATEAU_HEIGHT,terrain_parameters.PLATEAU_HEIGHT);
    let value = hill_value + pit_value;

    value
}

