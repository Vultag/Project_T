use std::process::Child;

use bevy::{prelude::*, transform::{commands, self}, reflect::{Enum, VariantType}, utils::error};
use bevy::window::PrimaryWindow;
use noise::Clamp;

use super::terrain_mesh::TerrainMesh;

use crate::TerrainMeshData;


pub struct TerrainUIPlugin;

#[derive(Component)]
struct SlidderCursorData {

    pub slider_id:u8,
    pub slider_value:f32,
    pub grabbed:bool,

}

impl Plugin for TerrainUIPlugin{
    fn build(&self, app: &mut App)
    {
        app
        .add_systems(Startup, spawn_ui)
        .add_systems(Update, handle_sliders);
    }
}

fn spawn_ui(
    mut commands:Commands,
    assets:Res<AssetServer>
)
{

    let startvalue:f32 = 50.0;

    commands.spawn(NodeBundle
    {
        style: Style{
            position_type:PositionType::Absolute,
            display:Display::Grid,
            justify_items:JustifyItems::Center,
            align_items:AlignItems::Center,
            top:Val::Percent(1.0),
            right:Val::Percent(1.0),
            width:Val::Percent(20.0),
            height:Val::Percent(50.0),

            border:UiRect{
                left: Val::Percent(0.5),
                right: Val::Percent(0.5),
                top: Val::Percent(0.5),
                bottom: Val::Percent(0.5),
            },
            ..Default::default()
        },
        background_color: Color::rgba(0.1, 0.1, 0.1, 0.6).into(),
        border_color: Color::rgba(0.4, 0.4, 0.4, 0.8).into(),
        ..default()
    }).with_children(|parent0|
    {
        //Noise scale slider
        parent0.spawn(NodeBundle
        {
            style: Style{
                display:Display::Grid,
                position_type:PositionType::Relative,
                justify_items:JustifyItems::Center,

                align_items:AlignItems::Center,
                width:Val::Percent(80.0),
                height:Val::VMin(0.5),
                ..Default::default()
            },
            background_color: Color::RED.into(),
            ..default()
        }).with_children(|parent|{
            parent.spawn(ButtonBundle{

                style:Style{
                    position_type:PositionType::Absolute,
                    align_self:AlignSelf::Center,
                    justify_self:JustifySelf::Center,
                    left:Val::Percent(startvalue),
                    width:Val::VMin(2.0),
                    height:Val::VMin(2.0),
                    ..default()
                },
                background_color: Color::GREEN.into(),
                ..default()
            }).insert(SlidderCursorData{slider_id:0,slider_value:startvalue,grabbed:false});

        }).with_children(|parent0|{
            parent0.spawn(TextBundle{

                style:Style{
                    position_type:PositionType::Absolute,
                    align_self:AlignSelf::Center,
                    justify_self:JustifySelf::Center,
                    left:Val::Percent(40.0),
                    width:Val::VMin(50.0),
                    height:Val::VMin(10.0),
                    ..default()
                },
                text:Text::from_section(
                    
                    "NOISE SCALE",
                    TextStyle {
                        font_size: 15.0,
                        color: Color::WHITE,
                        ..default()
                    },
                )
                .with_alignment(TextAlignment::Center).with_no_wrap(),
                ..default()
                
            });


        });

    })
    .with_children(|parent0|
        {
            //CLIFF STEEPNESS slider
            parent0.spawn(NodeBundle
            {
                style: Style{
                    display:Display::Grid,
                    position_type:PositionType::Relative,
                    justify_items:JustifyItems::Center,
    
                    align_items:AlignItems::Center,
                    width:Val::Percent(80.0),
                    height:Val::VMin(0.5),
                    ..Default::default()
                },
                background_color: Color::RED.into(),
                ..default()
            }).with_children(|parent|{
                parent.spawn(ButtonBundle{
    
                    style:Style{
                        position_type:PositionType::Absolute,
                        align_self:AlignSelf::Center,
                        justify_self:JustifySelf::Center,
                        left:Val::Percent(startvalue),
                        width:Val::VMin(2.0),
                        height:Val::VMin(2.0),
                        ..default()
                    },
                    background_color: Color::GREEN.into(),
                    ..default()
                }).insert(SlidderCursorData{slider_id:1,slider_value:startvalue,grabbed:false});
    
            }).with_children(|parent0|{
                parent0.spawn(TextBundle{
    
                    style:Style{
                        position_type:PositionType::Absolute,
                        align_self:AlignSelf::Center,
                        justify_self:JustifySelf::Center,
                        left:Val::Percent(30.0),
                        width:Val::VMin(50.0),
                        height:Val::VMin(10.0),
                        ..default()
                    },
                    text:Text::from_section(
                        
                        "CLIFF STEEPNESS",
                        TextStyle {
                            font_size: 15.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    )
                    .with_alignment(TextAlignment::Center).with_no_wrap(),
                    ..default()
                    
                });
    
    
            });
    
        })
        .with_children(|parent0|
            {
                //PLATEAU HEIGHT slider
                parent0.spawn(NodeBundle
                {
                    style: Style{
                        display:Display::Grid,
                        position_type:PositionType::Relative,
                        justify_items:JustifyItems::Center,
        
                        align_items:AlignItems::Center,
                        width:Val::Percent(80.0),
                        height:Val::VMin(0.5),
                        ..Default::default()
                    },
                    background_color: Color::RED.into(),
                    ..default()
                }).with_children(|parent|{
                    parent.spawn(ButtonBundle{
        
                        style:Style{
                            position_type:PositionType::Absolute,
                            align_self:AlignSelf::Center,
                            justify_self:JustifySelf::Center,
                            left:Val::Percent(startvalue),
                            width:Val::VMin(2.0),
                            height:Val::VMin(2.0),
                            ..default()
                        },
                        background_color: Color::GREEN.into(),
                        ..default()
                    }).insert(SlidderCursorData{slider_id:2,slider_value:startvalue,grabbed:false});
        
                }).with_children(|parent0|{
                    parent0.spawn(TextBundle{
        
                        style:Style{
                            position_type:PositionType::Absolute,
                            align_self:AlignSelf::Center,
                            justify_self:JustifySelf::Center,
                            left:Val::Percent(40.0),
                            width:Val::VMin(50.0),
                            height:Val::VMin(10.0),
                            ..default()
                        },
                        text:Text::from_section(
                            
                            "PLATEAU HEIGHT",
                            TextStyle {
                                font_size: 15.0,
                                color: Color::WHITE,
                                ..default()
                            },
                        )
                        .with_alignment(TextAlignment::Center).with_no_wrap(),
                        ..default()
                        
                    });
        
        
                });
        
            })
            .with_children(|parent0|
                {
                    //HILL VOLUME slider
                    parent0.spawn(NodeBundle
                    {
                        style: Style{
                            display:Display::Grid,
                            position_type:PositionType::Relative,
                            justify_items:JustifyItems::Center,
            
                            align_items:AlignItems::Center,
                            width:Val::Percent(80.0),
                            height:Val::VMin(0.5),
                            ..Default::default()
                        },
                        background_color: Color::RED.into(),
                        ..default()
                    }).with_children(|parent|{
                        parent.spawn(ButtonBundle{
            
                            style:Style{
                                position_type:PositionType::Absolute,
                                align_self:AlignSelf::Center,
                                justify_self:JustifySelf::Center,
                                left:Val::Percent(startvalue),
                                width:Val::VMin(2.0),
                                height:Val::VMin(2.0),
                                ..default()
                            },
                            background_color: Color::GREEN.into(),
                            ..default()
                        }).insert(SlidderCursorData{slider_id:3,slider_value:startvalue,grabbed:false});
            
                    }).with_children(|parent0|{
                        parent0.spawn(TextBundle{
            
                            style:Style{
                                position_type:PositionType::Absolute,
                                align_self:AlignSelf::Center,
                                justify_self:JustifySelf::Center,
                                left:Val::Percent(40.0),
                                width:Val::VMin(50.0),
                                height:Val::VMin(10.0),
                                ..default()
                            },
                            text:Text::from_section(
                                
                                "HILL VOLUME",
                                TextStyle {
                                    font_size: 15.0,
                                    color: Color::WHITE,
                                    ..default()
                                },
                            )
                            .with_alignment(TextAlignment::Center).with_no_wrap(),
                            ..default()
                            
                        });
            
            
                    });
            
                }).with_children(|parent0|
                    {
                        //PIT VOLUME slider
                        parent0.spawn(NodeBundle
                        {
                            style: Style{
                                display:Display::Grid,
                                position_type:PositionType::Relative,
                                justify_items:JustifyItems::Center,
                
                                align_items:AlignItems::Center,
                                width:Val::Percent(80.0),
                                height:Val::VMin(0.5),
                                ..Default::default()
                            },
                            background_color: Color::RED.into(),
                            ..default()
                        }).with_children(|parent|{
                            parent.spawn(ButtonBundle{
                
                                style:Style{
                                    position_type:PositionType::Absolute,
                                    align_self:AlignSelf::Center,
                                    justify_self:JustifySelf::Center,
                                    left:Val::Percent(startvalue),
                                    width:Val::VMin(2.0),
                                    height:Val::VMin(2.0),
                                    ..default()
                                },
                                background_color: Color::GREEN.into(),
                                ..default()
                            }).insert(SlidderCursorData{slider_id:4,slider_value:startvalue,grabbed:false});
                
                        }).with_children(|parent0|{
                            parent0.spawn(TextBundle{
                
                                style:Style{
                                    position_type:PositionType::Absolute,
                                    align_self:AlignSelf::Center,
                                    justify_self:JustifySelf::Center,
                                    left:Val::Percent(40.0),
                                    width:Val::VMin(50.0),
                                    height:Val::VMin(10.0),
                                    ..default()
                                },
                                text:Text::from_section(
                                    
                                    "PIT VOLUME",
                                    TextStyle {
                                        font_size: 15.0,
                                        color: Color::WHITE,
                                        ..default()
                                    },
                                )
                                .with_alignment(TextAlignment::Center).with_no_wrap(),
                                ..default()
                                
                            });
                
                
                        });
                
                    });

}

fn handle_sliders(
    mut commands:Commands,
    mouse_button: Res<Input<MouseButton>>,    
    transform_query: Query<&GlobalTransform,With<Style>>,
    mut cursor_evr: EventReader<CursorMoved>,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut terrain_query: Query<(&Handle<Mesh>,&mut TerrainMeshData)>, 
    mut interaction_query: Query<(Entity,&Interaction,&mut SlidderCursorData, &mut Style, &Parent)>//, Changed<Interaction>>
){

    for (p_entity,interation,mut sliderdata, mut style,parent) in interaction_query.iter_mut()
    {

        if *interation == Interaction::Pressed{
            sliderdata.grabbed = true;

            for ev in cursor_evr.iter() {
                
                
                //range from 0.0 to 100.0 
                let mut new_s_pos = ((ev.position.x-transform_query.get(parent.get()).unwrap().translation().x+100.0)*0.5).clamp(0.0,100.0);
                sliderdata.slider_value = new_s_pos;
                style.left = Val::Percent(new_s_pos);
            
                new_s_pos*=0.01;

                match sliderdata.slider_id
                {
                    // NOISE_SCALE slider
                    0 => {
                        for (mut mesh_handle, mut terrain_data) in terrain_query.iter_mut() {
    
            
                            let mut mesh = mesh_assets.get_mut(mesh_handle.id()).unwrap();
                
                            //start 0.05
                            terrain_data.NOISE_SCALE = 0.10*new_s_pos;
                
                            TerrainMesh::edit_terrain(mesh,terrain_data.subdivision, terrain_data.NOISE_SCALE, terrain_data.CLIFF_STEEPNESS, terrain_data.PLATEAU_HEIGHT, terrain_data.HILL_VOLUME, terrain_data.PIT_VOLUME);
                    
                        }
                    }
                    // CLIFF_STEEPNESS slider
                    1 => {
                        for (mut mesh_handle, mut terrain_data) in terrain_query.iter_mut() {
    
            
                            let mut mesh = mesh_assets.get_mut(mesh_handle.id()).unwrap();
                
                            //start 15.0
                            terrain_data.CLIFF_STEEPNESS = 30.0*new_s_pos;
                
                            TerrainMesh::edit_terrain(mesh,terrain_data.subdivision, terrain_data.NOISE_SCALE, terrain_data.CLIFF_STEEPNESS, terrain_data.PLATEAU_HEIGHT, terrain_data.HILL_VOLUME, terrain_data.PIT_VOLUME);
                    
                        }
                    }
                    // PLATEAU_HEIGHT slider
                    2 => {
                        for (mut mesh_handle, mut terrain_data) in terrain_query.iter_mut() {
    
            
                            let mut mesh = mesh_assets.get_mut(mesh_handle.id()).unwrap();
                
                            //start 2.0
                            terrain_data.PLATEAU_HEIGHT = 4.0*new_s_pos;
                
                            TerrainMesh::edit_terrain(mesh,terrain_data.subdivision, terrain_data.NOISE_SCALE, terrain_data.CLIFF_STEEPNESS, terrain_data.PLATEAU_HEIGHT, terrain_data.HILL_VOLUME, terrain_data.PIT_VOLUME);
                    
                        }
                    }
                    // HILL_VOLUME slider
                    3 => {
                        for (mut mesh_handle, mut terrain_data) in terrain_query.iter_mut() {
    
            
                            let mut mesh = mesh_assets.get_mut(mesh_handle.id()).unwrap();
                
                            //start 0.5
                            terrain_data.HILL_VOLUME = 1.0*new_s_pos;
                
                            TerrainMesh::edit_terrain(mesh,terrain_data.subdivision, terrain_data.NOISE_SCALE, terrain_data.CLIFF_STEEPNESS, terrain_data.PLATEAU_HEIGHT, terrain_data.HILL_VOLUME, terrain_data.PIT_VOLUME);
                    
                        }
                    }
                    // PIT_VOLUME slider
                    4 => {
                        for (mut mesh_handle, mut terrain_data) in terrain_query.iter_mut() {
    
            
                            let mut mesh = mesh_assets.get_mut(mesh_handle.id()).unwrap();
                
                            //start 0.5
                            terrain_data.PIT_VOLUME = 1.0*new_s_pos;
                
                            TerrainMesh::edit_terrain(mesh,terrain_data.subdivision, terrain_data.NOISE_SCALE, terrain_data.CLIFF_STEEPNESS, terrain_data.PLATEAU_HEIGHT, terrain_data.HILL_VOLUME, terrain_data.PIT_VOLUME);
                    
                        }
                    }
                    _ =>
                    {
                        panic!("slider id outside range of available sliders")
                    }

                }
            
            
            }

        }
        else {
            if(sliderdata.grabbed == true)
            {
    
                if (mouse_button.just_released(MouseButton::Left))
                {
                    sliderdata.grabbed = false;
                    break;
                }
    
            }
        }


    }





}