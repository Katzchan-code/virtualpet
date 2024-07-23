use std::time::Duration;
use bevy::{prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}};
use crate::components::{HungerTime, HungerAmount, StartingPosition, StartActivated, Rat, HungerBar, PlaytimeBar};

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
    StartActivated
    ));
        commands.spawn((HungerBar, MaterialMesh2dBundle {
        mesh: Mesh2dHandle(meshes.add(Rectangle::new(25.0, 100.0))),
        material: materials.add(Color::rgb(0.9, 0.1, 0.1)),
        transform: Transform::from_xyz(
            -170.0,
            -100.0, 
            0.0),
        visibility: Visibility::Visible,
            ..default()
            
    },
    HungerTime {
        timer: {
            Timer::new(Duration::from_secs(240), TimerMode::Repeating)
        }
    }, 
    HungerAmount {
       amount: 100.0
    },
    StartingPosition {
        y: -100.0
    },
    StartActivated
));

}

 fn hunger(
    mut commands: Commands,
    mut bar_data: Query<(Entity, &mut HungerTime, &mut HungerAmount, &mut StartingPosition), Without<PlaytimeBar>>,
    mut rat_data: Query<Entity, With<Rat>>,
    mut meshes: ResMut<Assets<Mesh>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for (bar, mut hunger_timer, mut hunger, mut position) in &mut bar_data {
        hunger_timer.timer.tick(time.delta());
        if hunger_timer.timer.finished() {
            position.y -= 12.5;
            hunger.amount -= 25.0;
            }

        if keyboard_input.just_pressed(KeyCode::KeyZ) {
            hunger.amount += 25.0;
            println!("Z key was pressed to feed the pet");
            if hunger.amount > 100.0 {
                hunger.amount = 100.0;
            } else {
                position.y += 12.5;
            }
        }

        if hunger.amount == 0.0 {
            commands.entity(bar).despawn();
            for rat in &mut rat_data{
                commands.entity(rat).despawn();
            }
        } else {
            commands.entity(bar).insert(Mesh2dHandle(meshes.add(Rectangle::new(25.0, hunger.amount))));
            commands.entity(bar).insert(Transform::from_xyz(
            -170.0,
            position.y, 
            0.0   
            ));
        }
    }
}
