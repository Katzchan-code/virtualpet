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

    mut rat_sprite: Query<(Entity, &mut Activated), (With<Rat>, Without<PlaytimeOverlay>, Without<HungerSprite>, Without<HungerBar>)>,

    mut hunger_bar: Query<(Entity, &mut Activated), (With<HungerBar>, Without<PlaytimeOverlay>, Without<HungerSprite>, Without<Rat>)>,
    mut hunger_sprite: Query<(Entity, &mut Activated), (With<HungerSprite>, Without<HungerBar>, Without<PlaytimeOverlay>, Without<Rat>)>,

    mut playtime: Query<(Entity, &mut Activated), (With <PlaytimeOverlay>, Without<HungerBar>, Without<HungerSprite>, Without<Rat>)>,

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

        for mut standing_timer in &mut movement_timer {
            if standing_timer.timer.paused() == false {
                standing_timer.timer.pause();
            }
            else {
                standing_timer.timer.unpause();
            }
        }   
    
    //hide everything
        for (rat, mut active_rat) in &mut rat_sprite {
            if active_rat.active == true {
                commands.entity(rat).insert(Visibility::Hidden);
                active_rat.active = false;
            }
            else {
                commands.entity(rat).insert(Visibility::Visible);
                active_rat.active = true;
            }
        }

        for (bar, mut active_bar) in &mut hunger_bar {
            if active_bar.active == true {
                commands.entity(bar).insert(Visibility::Hidden);
                active_bar.active = false;
            }
            else {
                commands.entity(bar).insert(Visibility::Visible);
                active_bar.active = true;
            }
        }

        for (hunger_sprite, mut active_hunger_icon) in &mut hunger_sprite {
            if active_hunger_icon.active == true {
                commands.entity(hunger_sprite).insert(Visibility::Hidden);
                active_hunger_icon.active = false;
            }
            else {
                commands.entity(hunger_sprite).insert(Visibility::Visible);
                active_hunger_icon.active = true;
            }
        }

    //overlay minigame screen
        for (playtime_overlay, mut active_window) in &mut playtime {
            if active_window.active == false {
                commands.entity(playtime_overlay).insert(Visibility::Visible);
                active_window.active = true;
                println!("X key was pressed to open playtime menu")
            }
            else {
                commands.entity(playtime_overlay).insert(Visibility::Hidden);
                active_window.active = false;
                println!("X key was pressed to close playtime menu")
            }
        }
    }
} 
}