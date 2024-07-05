use std::time::Duration;
use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use rand::Rng;


use crate::foodsys:: HungerTime;
use crate::character_movement::StandingTime;

pub struct PlaytimePlugin;
impl Plugin for PlaytimePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, playtime_overlay);
        app.add_systems(Update, playtime_window);
        app.add_systems(Update, rock_paper_scissors);
    }
}

#[derive(Component)]
struct GameTextTimer {
    timer: Timer
}

#[derive(Component)]
struct Playtime {
    opponent_rps: i32
}

#[derive(Component)]
pub struct StartActivated;

#[derive(Component)]
pub struct StartDectivated;

#[derive(Component)]
pub struct Toggle {
    pub toggle: bool
}

#[derive (Component)]
struct MainGameText;

#[derive (Component)]
struct WinGameText;

#[derive (Component)]
struct PlaytimeTimer {
    ptimer: Timer
}

fn playtime_overlay(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    commands.spawn((MaterialMesh2dBundle {
        mesh: Mesh2dHandle(meshes.add(Rectangle::new(500.0, 500.0))),
        material: materials.add(Color::rgb(0.0, 0.0, 0.6)),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        visibility: Visibility::Hidden,
        ..default()
    },
        StartDectivated,
        Toggle {
            toggle: false
        }
    ));
    
    commands.spawn((MainGameText, Text2dBundle { 
        text: Text::from_section("Lets play Rock Paper Scissors!\n1 = Rock, 2 = Paper, 3 = Scissors!",TextStyle {
            font_size: 20.0,
            ..default()
        }),
        transform: Transform::from_xyz(-20.0, 175.0, 10.0),
        visibility: Visibility::Hidden,
        ..default() 
        },
        StartDectivated,

        Playtime {
            opponent_rps: rand::thread_rng().gen_range(1..=3)
        }
    ));

    commands.spawn((MaterialMesh2dBundle {
        mesh: Mesh2dHandle(meshes.add(Rectangle::new(25.0, 100.0))),
        material: materials.add(Color::rgb(0.0, 0.5, 1.0)),
        transform: Transform::from_xyz (
            170.0,
            -100.0,
            0.0),
        visibility: Visibility::Visible,
        ..default()
        },
    PlaytimeTimer {
        ptimer: {
            Timer::new(Duration::from_secs(1), TimerMode::Repeating)
        }
    }));
}


fn playtime_window(
    time: Res<Time>,
    mut keyboard_input: ResMut<ButtonInput<KeyCode>>,
    mut foodsys_timer: Query<&mut HungerTime>,
    mut movement_timer: Query<&mut StandingTime>,
    mut playtime_timer: Query<&mut PlaytimeTimer>,

    mut toggle: Query<&mut Toggle>,
    mut started_visible: Query<Entity, With<StartActivated>>,
    mut started_invisible: Query<Entity, With <StartDectivated>>,
    mut win_text: Query<(Entity, &mut GameTextTimer)>,
    mut commands: Commands,
    //mut meshes: ResMut<Assets<Mesh>>,
    //mut materials: ResMut<Assets<ColorMaterial>>
) {
    if keyboard_input.clear_just_pressed(KeyCode::KeyX) {
        for mut toggled in &mut toggle {
            if toggled.toggle == false {
                toggled.toggle = true;
            } else {
                toggled.toggle = false;
            }
        }
    }

    for toggled in &mut toggle {
        if toggled.toggle == true {
            hide_pet(&mut foodsys_timer, &mut movement_timer, &mut started_visible, &mut started_invisible, &mut commands);
        } else {
            show_pet(&mut foodsys_timer, &mut movement_timer, &mut started_visible, &mut started_invisible, &mut commands);
        }      
    }
    for(text, mut text_timer) in &mut win_text {
        text_timer.timer.tick(time.delta());
        if text_timer.timer.finished() {
            commands.entity(text).despawn();
        }
    }
}

fn hide_pet (
    foodsys_timer: &mut Query<&mut HungerTime>,
    movement_timer: &mut Query<&mut StandingTime>,
    started_visible: &mut Query<Entity, With<StartActivated>>,
    started_invisible: &mut Query<Entity, With <StartDectivated>>,
    commands: &mut Commands,
) {
    for mut hunger_timer in foodsys_timer {
        hunger_timer.timer.pause();
        }

    for mut standing_timer in movement_timer {
        standing_timer.timer.pause();
        }   

    for entity in started_visible {
        commands.entity(entity).insert(Visibility::Hidden);
        }

    for playtime_overlay in started_invisible {
        commands.entity(playtime_overlay).insert(Visibility::Visible);
        }
} 


fn show_pet (
    foodsys_timer: &mut Query<&mut HungerTime>,
    movement_timer: &mut Query<&mut StandingTime>,
    started_visible: &mut Query<Entity, With<StartActivated>>,
    started_invisible: &mut Query<Entity, With <StartDectivated>>,
    commands: &mut Commands,
) {
    for mut hunger_timer in foodsys_timer {
        hunger_timer.timer.unpause();
    }

    for mut standing_timer in movement_timer {
        standing_timer.timer.unpause();
    }

    for entity in started_visible {
        commands.entity(entity).insert(Visibility::Visible);
    }

    for playtime_overlay in started_invisible {
        commands.entity(playtime_overlay).insert(Visibility::Hidden);
    }
}

fn rock_paper_scissors (
    keyboard_input: ResMut<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut ai_rps: Query<(Entity, &mut Playtime)>,

) {
    let rock = 1;
    let paper = 2;
    let scissors = 3;
    let input_rock = KeyCode::Digit1;
    let input_paper = KeyCode::Digit2;
    let input_scissors = KeyCode::Digit3;

    for (opponent, mut rps) in &mut ai_rps {
        if rps.opponent_rps == rock {
            if keyboard_input.just_pressed(input_paper) {
                rps_win(&mut commands);
                commands.entity(opponent).insert(Playtime {opponent_rps: rand::thread_rng().gen_range(1..=3)});
            } else if keyboard_input.just_pressed(input_rock) {
                rps_lose(&mut commands);
            } else if keyboard_input.just_pressed(input_scissors) {
                rps_lose(&mut commands);
            }
    
        } else if rps.opponent_rps == paper {
            if keyboard_input.just_pressed(input_scissors) {
                rps_win(&mut commands);
                rps.opponent_rps = rand::thread_rng().gen_range(1..=3);
                commands.entity(opponent).insert(Playtime {opponent_rps: rand::thread_rng().gen_range(1..=3)});
            } else if keyboard_input.just_pressed(input_rock) {
                rps_lose(&mut commands);
            } else if keyboard_input.just_pressed(input_paper) {
                rps_lose(&mut commands);
            }
    
        } else if rps.opponent_rps == scissors{
            if keyboard_input.just_pressed(input_rock) {
                rps_win(&mut commands);
                rps.opponent_rps = rand::thread_rng().gen_range(1..=3);
                commands.entity(opponent).insert(Playtime {opponent_rps: rand::thread_rng().gen_range(1..=3)});
            } else if keyboard_input.just_pressed(input_rock) {
                rps_lose(&mut commands);
            } else if keyboard_input.just_pressed(input_paper) {
                rps_lose(&mut commands);
            }
        }
    }
}


fn rps_win (
    commands: &mut Commands,
) {
    commands.spawn((WinGameText, Text2dBundle { 
        text: Text::from_section("You win!",TextStyle {
            font_size: 20.0,
            ..default()
        }),
        transform: Transform::from_xyz(0.0, 0.0, 2.0),
        ..default() 
    },
    GameTextTimer {
        timer: {
            Timer::new(Duration::from_secs(1), TimerMode::Once)
        }
    }
));
}

fn rps_lose (
    commands: &mut Commands,
) {
    commands.spawn((WinGameText, Text2dBundle { 
        text: Text::from_section("Try again!",TextStyle {
            font_size: 20.0,
            ..default()
        }),
        transform: Transform::from_xyz(0.0, 0.0, 2.0),
        ..default() 
    },
    GameTextTimer {
        timer: {
            Timer::new(Duration::from_secs(1), TimerMode::Once)
        }
    }
));
}