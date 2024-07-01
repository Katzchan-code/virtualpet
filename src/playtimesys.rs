use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};


use crate::character::Rat;
use crate::foodsys::{HungerSprite, HungerTime};
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
pub struct StartActivated {
    pub visible: bool
}

#[derive(Component)]
pub struct StartDectivated {
    pub visible: bool
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
        StartDectivated {
            visible:false
        }
));
}

fn playtime_window(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut foodsys_timer: Query<&mut HungerTime>,
    mut movement_timer: Query<&mut StandingTime>,

    mut started_visible: Query<(Entity, &mut StartActivated)>,
    mut started_invisible: Query<(Entity, &mut StartDectivated)>,

    mut commands: Commands,
    //mut meshes: ResMut<Assets<Mesh>>,
    //mut materials: ResMut<Assets<ColorMaterial>>
) {
    if keyboard_input.just_pressed(KeyCode::KeyX) {
      //pause or unpause timers depending on if X is pressed to bring up minigame screen      
        for mut hunger_timer in &mut foodsys_timer {
            if hunger_timer.timer.paused() == false {
             hunger_timer.timer.pause();
            }
            else {
             hunger_timer.timer.unpause();
            }
        }

        for mut standing_timer in &mut movement_timer {
            if standing_timer.timer.paused() == false {
                standing_timer.timer.pause();
            }
            else {
                standing_timer.timer.unpause();
            }
        }   
    
    //hide everything
        for(entity, mut started_visible) in &mut started_visible {
            if started_visible.visible == true {
                commands.entity(entity).insert(Visibility::Hidden);
                started_visible.visible = false;
            }
            else {
                commands.entity(entity).insert(Visibility::Visible);
                started_visible.visible = true;
            }
        }

    //overlay minigame screen
        for (playtime_overlay, mut active_window) in &mut started_invisible {
            if active_window.visible == false {
                commands.entity(playtime_overlay).insert(Visibility::Visible);
                active_window.visible = true;
                println!("X key was pressed to open playtime menu")
            }
            else {
                commands.entity(playtime_overlay).insert(Visibility::Hidden);
                active_window.visible = false;
                println!("X key was pressed to close playtime menu")
            }
        }



    } //if x is pressed end
} 

fn rock_paper_scissors (
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    if keyboard_input.just_pressed(KeyCode::KeyX) {
        
    }
}