use std::time::Duration;
use bevy::{prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}};
use rand::Rng;
use crate::character::Rat;

#[derive(Component)]
pub struct HungerTime {
   pub timer: Timer,
}

#[derive(Component)]
pub struct HungerAmount {
   amount: f32
}

#[derive(Component)]
pub struct StartingPosition {
   y: f32
}

#[derive(Component)]
pub struct HungerBar; 

pub struct FoodSysPlugin;
impl Plugin for FoodSysPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, hunger);
        app.add_systems(Startup, bread_and_timer);
    }
} 

fn bread_and_timer(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((SpriteBundle {
        texture: asset_server.load("bread.png"),
        transform: Transform::from_xyz(
            -170.0,
            -170.0, 
            0.0
        ),
        ..default()
    },
    ));
        commands.spawn((HungerBar, MaterialMesh2dBundle {
        mesh: Mesh2dHandle(meshes.add(Rectangle::new(25.0, 100.0))),
        material: materials.add(Color::rgb(0.9, 0.1, 0.1)),
        transform: Transform::from_xyz(
            -170.0,
            -100.0, 
            0.0),
            ..default()
            
    },
    HungerTime {
        timer: {
            Timer::new(Duration::from_secs(rand::thread_rng().gen_range(1..=2)), TimerMode::Repeating)
        }
    }, 
    HungerAmount {
       amount: 100.0
    },
    StartingPosition {
        y: -100.0
    }
));

}

 fn hunger(
    mut commands: Commands,
    mut bar_data: Query<(Entity, &mut HungerTime, &mut HungerAmount, &mut StartingPosition)>,
    mut rat_data: Query<Entity, With<Rat>>,
    mut meshes: ResMut<Assets<Mesh>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for (bar, mut hunger_timer, mut hunger, mut position) in &mut bar_data{
        hunger_timer.timer.tick(time.delta());
        if hunger_timer.timer.finished() {
            hunger_timer.timer.tick(time.delta());
            position.y -= 12.5;
            hunger.amount -= 25.0;
            }
        if keyboard_input.just_pressed(KeyCode::KeyZ) {
            hunger.amount += 25.0;
            if hunger.amount > 100.0 {
                hunger.amount = 100.0;
            } else {
                position.y += 12.5;
            }
        }
        match hunger.amount {
            100.0 => {
                hunger_bar_modifications(&mut commands, bar, &mut hunger, &mut position, &mut meshes)
            }
            75.0 => {
                hunger_bar_modifications(&mut commands, bar, &mut hunger, &mut position, &mut meshes)
            },
            50.0 => {
                hunger_bar_modifications(&mut commands, bar, &mut hunger, &mut position, &mut meshes)
            },
            25.0 => {
                hunger_bar_modifications(&mut commands, bar, &mut hunger, &mut position, &mut meshes)
            },
            0.0 => {
                commands.entity(bar).despawn();
                for rat in &mut rat_data{
                    commands.entity(rat).despawn();
                }
            }
            _ => {
                commands.entity(bar).insert(Mesh2dHandle(meshes.add(Rectangle::new(25.0, 100.0))));
                commands.entity(bar).insert(Transform::from_xyz(
                -170.0,
                -100.0, 
                0.0   
                ));
            }
        }
    }
}

fn hunger_bar_modifications (
    commands: &mut Commands,
    bar: Entity,
    hunger: &mut HungerAmount,
    position: &mut StartingPosition,
    meshes: &mut ResMut<Assets<Mesh>>,
)
{
    commands.entity(bar).insert(Mesh2dHandle(meshes.add(Rectangle::new(25.0, hunger.amount))));
    commands.entity(bar).insert(Transform::from_xyz(
    -170.0,
    position.y, 
    0.0   
    ));
}

