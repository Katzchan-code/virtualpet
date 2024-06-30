use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};



use crate::foodsys::HungerTime;
use crate::foodsys::HungerBar;
use crate::character_movement::StandingTime;

pub struct PlaytimePlugin;
impl Plugin for PlaytimePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, playtime_overlay);
        app.add_systems(Update, playtime_window);
    }
}

#[derive(Component)]
struct PlaytimeOverlay;

#[derive(Component)]
pub struct Activated {
    pub active: bool
}


fn playtime_overlay(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    commands.spawn((PlaytimeOverlay, MaterialMesh2dBundle {
        mesh: Mesh2dHandle(meshes.add(Rectangle::new(500.0, 500.0))),
        material: materials.add(Color::rgb(0.0, 0.0, 0.6)),
        visibility: Visibility::Hidden,
        ..default()
    },
        Activated {
            active:false
        }
));
}

fn playtime_window(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut foodsys_timer: Query<&mut HungerTime>,
    mut movement_timer: Query<&mut StandingTime>,
    mut hunger_bar: Query<(Entity/*Querying for hungerbar activated state causes crash on startup
                                    but only when queryed in this file , &mut Activated*/), With<HungerBar>>,

    mut playtime: Query<(Entity, &mut Activated), With <PlaytimeOverlay>>,

    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    if keyboard_input.just_pressed(KeyCode::KeyX) {
      //pause or unpause timers depending on if X is pressed to bring up minigame screen      
        for mut hunger_timer in &mut foodsys_timer {
            if hunger_timer.timer.paused() == false {
             hunger_timer.timer.pause();
             println!("X key was pressed to pause the timers")
            }
            else {
             hunger_timer.timer.unpause();
             println!("X key was pressed to unpause the timers")
            }

        for mut standing_timer in &mut movement_timer {
            if standing_timer.timer.paused() == false {
                standing_timer.timer.pause();
            }
            else {
                standing_timer.timer.unpause();
            }
        }   
    //end of pause/unpause code      cant seem to bundle them together, probably doing something wrong but heigh ho off to work we go

    for (bar) in &mut hunger_bar {
        
    }

    //overlay minigame screen
    for (playtime_overlay, mut active_window) in &mut playtime {
        if active_window.active == false {
            commands.entity(playtime_overlay).insert(Visibility::Visible);
            active_window.active = true;
        }
        else {
            commands.entity(playtime_overlay).insert(Visibility::Hidden);
            active_window.active = false;
        }
    }
    }
} 
}