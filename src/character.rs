use std::time::Duration;
use bevy::prelude::*;
use crate::components::{Rat, Direction, StandingTime, StartActivated};


pub struct CharacterPlugin;
impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_character);
    }
}

fn spawn_character(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((Rat, SpriteBundle {
        texture: asset_server.load("rat.png"),
        sprite: Sprite {
            flip_x: false,
            flip_y: false,
            ..default()
            },
        ..default()
    },
    //starting movement direction
    Direction::Stand,
    //timer for choosing when to move
    StandingTime {
        timer: {
            Timer::new(Duration::from_secs(3), TimerMode::Repeating)
        }
    },
    StartActivated
    ));
}