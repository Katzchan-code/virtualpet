use std::time::Duration;
use bevy::{prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}};
use rand::Rng;

#[derive(Component)]
pub struct HungerTime {
   pub timer: Timer,
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
));

}

 fn hunger(
    mut commands: Commands,
    mut bar_data: Query<(Entity, & mut HungerTime)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for (bar, mut hunger_timer) in &mut bar_data{
        /*if keyboard_input.just_pressed(KeyCode::KeyX) {
                commands.entity(bar).insert(materials.add(Color::from(Color::GREEN)));
                commands.entity(bar).insert(Mesh2dHandle(meshes.add(Rectangle::new(25.0, 100.0))));
                commands.entity(bar).insert(Transform::from_xyz(
                -170.0,
                -100.0, 
                0.0   
                ));
            }
        if keyboard_input.just_pressed(KeyCode::KeyZ) {
                commands.entity(bar).insert(materials.add(Color::from(Color::RED)));
                commands.entity(bar).insert(Mesh2dHandle(meshes.add(Rectangle::new(100.0, 25.0))));
                commands.entity(bar).insert(Transform::from_xyz(
                    -170.0,
                    -150.0, 
                    0.0   
                    ));
            }*/
        hunger_timer.timer.tick(time.delta());
        if hunger_timer.timer.finished() {
            commands.entity(bar).insert(materials.add(Color::from(Color::GREEN)));
            commands.entity(bar).insert(Mesh2dHandle(meshes.add(Rectangle::new(25.0, 100.0))));
            commands.entity(bar).insert(Transform::from_xyz(
            -170.0,
            -100.0, 
            0.0   
            ));
        }
    }
}