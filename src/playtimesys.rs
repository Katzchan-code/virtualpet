use bevy::{prelude::*, window::{EnabledButtons, WindowResolution}};

pub struct PlaytimePlugin;
impl Plugin for PlaytimePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, );
    }
}

fn playtime_window (
    keyboard_input: Res<ButtonInput<KeyCode>>
) 
{
    if keyboard_input.just_pressed(KeyCode::KeyX) {
        
    }
}