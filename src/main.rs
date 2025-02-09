mod components;
mod camera;
mod character;
mod character_movement;
mod debug;
mod foodsys;
mod playtimesys;


use bevy::{prelude::*, window::{EnabledButtons, WindowResolution}};
use camera::CameraPlugin;
use character::CharacterPlugin;
use character_movement::MovementPlugin;
use foodsys::FoodSysPlugin;
use playtimesys::PlaytimePlugin;
//use debug::DebugPlugin;

fn main() {
    App::new()
        //Create 400x400 window and set the background color
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "VirtualPet v1.0.0".into(),
                resolution: WindowResolution::new(400.0, 400.0).with_scale_factor_override(1.0),
                resizable: false,
                enabled_buttons: EnabledButtons {
                    minimize: true,
                    maximize: false,
                    close: true
                },
                name: Some("virtualpet".to_string()),
                decorations: true,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::rgb(0.7, 0.7, 0.7)))

        //tyler defined plugins
        .add_plugins(CameraPlugin)
        .add_plugins(CharacterPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(FoodSysPlugin)
        .add_plugins(PlaytimePlugin)
        
        //debug plugin for debug reasons
        //.add_plugins(DebugPlugin)
        .run();
}
