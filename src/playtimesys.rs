use std::time::Duration;
use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use rand::Rng;

use crate::components::{
    StartActivated, StartDectivated, MainGameText, WinGameText,
    Playtime, PlaytimeTimer, GameTextTimer, Toggle, HungerTime,
    StandingTime, PlaytimeAmount, StartingPosition, PlaytimeBar,
    HungerBar, Rat
};

pub struct PlaytimePlugin;
impl Plugin for PlaytimePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, playtime_overlay);
        app.add_systems(Update, playtime_window);
        app.add_systems(Update, rock_paper_scissors);
        app.add_systems(Update, playtime_decay);
    }
}


fn playtime_overlay(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    commands.spawn((MaterialMesh2dBundle {
        mesh: Mesh2dHandle(meshes.add(Rectangle::new(500.0, 500.0))),
        material: materials.add(Color::rgb(0.0, 0.0, 0.6)),
        transform: Transform::from_xyz(0.0, 0.0, 1.0),
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
        transform: Transform::from_xyz(-20.0, 175.0, 2.0),
        visibility: Visibility::Hidden,
        ..default() 
        },
        StartDectivated,

        Playtime {
            opponent_rps: rand::thread_rng().gen_range(1..=3)
        }
    ));

    commands.spawn((PlaytimeBar, MaterialMesh2dBundle {
        mesh: Mesh2dHandle(meshes.add(Rectangle::new(25.0, 100.0))),
        material: materials.add(Color::rgb(0.0, 0.5, 1.0)),
        transform: Transform::from_xyz (
            170.0,
            -100.0,
            0.0),
        visibility: Visibility::Visible,
        ..default()
        },
    StartActivated,
    PlaytimeTimer {
        timer: {
            Timer::new(Duration::from_secs(120), TimerMode::Repeating)
        }
    },
    PlaytimeAmount {
        amount: 100.0
    },
    StartingPosition {
        y: -100.0
    }
    ));
}

fn playtime_decay(
    time: Res<Time>,
    mut bar_data: Query<(Entity, &mut PlaytimeTimer, &mut PlaytimeAmount, &mut StartingPosition), Without<HungerBar>>,
    mut rat_data: Query<Entity, With<Rat>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for (bar, mut play_timer, mut play, mut position) in &mut bar_data {
        play_timer.timer.tick(time.delta());
        if play_timer.timer.finished() {
            position.y -= 12.5;
            play.amount -= 25.0;
            }

        if play.amount == 0.0 {
            commands.entity(bar).despawn();
            for rat in &mut rat_data {
                commands.entity(rat).despawn();
            }
        } else {
            commands.entity(bar).insert(Mesh2dHandle(meshes.add(Rectangle::new(25.0, play.amount))));
            commands.entity(bar).insert(Transform::from_xyz(
            170.0,
            position.y, 
            0.0   
            ));
        }
    }
}

fn playtime_window(
    time: Res<Time>,
    mut keyboard_input: ResMut<ButtonInput<KeyCode>>,
    mut foodsys_timer: Query<&mut HungerTime>,
    mut movement_timer: Query<&mut StandingTime>,

    mut toggle: Query<&mut Toggle>,
    mut started_visible: Query<Entity, With<StartActivated>>,
    mut started_invisible: Query<Entity, With <StartDectivated>>,
    mut win_text: Query<(Entity, &mut GameTextTimer)>,
    mut commands: Commands,
    //mut meshes: ResMut<Assets<Mesh>>,
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
        commands.entity(entity).try_insert(Visibility::Hidden);
        }

    for playtime_overlay in started_invisible {
        commands.entity(playtime_overlay).try_insert(Visibility::Visible);
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
        commands.entity(entity).try_insert(Visibility::Visible);
    }

    for playtime_overlay in started_invisible {
        commands.entity(playtime_overlay).try_insert(Visibility::Hidden);
    }
}

fn rock_paper_scissors (
    keyboard_input: ResMut<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut ai_rps: Query<(Entity, &mut Playtime)>,
    mut toggle: Query<&mut Toggle>,
    mut playtime_bar_data: Query<(&mut PlaytimeAmount, &mut StartingPosition), With<PlaytimeBar>>,
) {
    //rock = 1;
    //paper = 2;
    //scissors = 3;
    let input_rock =  keyboard_input.just_pressed(KeyCode::Digit1);
    let input_paper = keyboard_input.just_pressed(KeyCode::Digit2);
    let input_scissors = keyboard_input.just_pressed(KeyCode::Digit3);

    for toggled in &mut toggle {
        if toggled.toggle == true {
            for (opponent, rps) in &mut ai_rps {
                match rps.opponent_rps {
                    1 => {
                        if input_paper {
                            rps_win(&mut commands, &mut playtime_bar_data);
                            commands.entity(opponent).insert(Playtime {opponent_rps: rand::thread_rng().gen_range(1..=3)});
                        } else if input_rock | input_scissors {
                            rps_lose(&mut commands);
                        } 
                    }
                    2 => {
                        if input_scissors {
                            rps_win(&mut commands, &mut playtime_bar_data);
                            commands.entity(opponent).insert(Playtime {opponent_rps: rand::thread_rng().gen_range(1..=3)});
                        } else if input_rock | input_scissors {
                            rps_lose(&mut commands);
                        }
                    }
                    3 => {
                        if input_rock {
                            rps_win(&mut commands, &mut playtime_bar_data);
                            commands.entity(opponent).insert(Playtime {opponent_rps: rand::thread_rng().gen_range(1..=3)});
                        } else if input_paper | input_scissors {
                            rps_lose(&mut commands);
                        }
                    }
                    _ =>{}
                }
            }
        } 
    } 
}


fn rps_win (
    commands: &mut Commands,
    playtime_bar_data: &mut Query<(&mut PlaytimeAmount, &mut StartingPosition), With<PlaytimeBar>>,
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
    for (mut play, mut position) in playtime_bar_data {
        play.amount += 25.0;
        if play.amount > 100.0 {
            play.amount = 100.0;
        } else {
            position.y += 12.5;
        }
    }
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