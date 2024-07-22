use bevy::prelude::*;
use rand::Rng;
use crate::components::{Direction, StandingTime};

pub struct MovementPlugin;
impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, character_movement);
    }
}

fn character_movement(
    time: Res<Time>,
    mut character_data: Query<(&mut Direction, &mut Transform, &mut StandingTime, &mut Sprite)>,
    keyboard_input: Res<ButtonInput<KeyCode>>
) {
    for (mut character_direction, mut transform, mut standing_timer, mut flip) in &mut character_data {
        match *character_direction {
            Direction::Stand => transform.translation.x += 0.0,
            Direction::Die => transform.translation.y += 100.0 * time.delta_seconds(),
        };
         if keyboard_input.just_pressed(KeyCode::Space) {
            *character_direction = Direction::Die;
        }

        let x_location = transform.translation.x;
        let y_location = transform.translation.y;
        let xdirchance = rand::thread_rng().gen_range(1..=2);
        let ydirchance = rand::thread_rng().gen_range(1..=2);

        if x_location > -400.0 && x_location < 400.0 {
            if xdirchance == 1 {
                move_left(&mut transform, &mut standing_timer, &mut flip, &time);
                }else {
                move_right(&mut transform, &mut standing_timer, &mut flip, &time);
                }
            }
        if x_location == -400.0 {
            move_right(&mut transform, &mut standing_timer, &mut flip, &time)
        }
        if x_location == 400.0 {
            move_left(&mut transform, &mut standing_timer, &mut flip, &time)
        }
        
        if y_location > -80.0 && y_location < 80.0 {
            if ydirchance == 1 {
                move_up(&mut transform, &mut standing_timer, &time)
                }else {
                move_down(&mut transform, &mut standing_timer, &time)    
                }
        } 
        if y_location == -80.0 {
            move_up(&mut transform, &mut standing_timer, &time)
        }
        if y_location == 80.0 {
            move_down(&mut transform, &mut standing_timer, &time)
        }
        if y_location > 400.0 {
            *character_direction = Direction::Stand;
            transform.translation.x = 0.0;
            transform.translation.y = 0.0;
        }
    } 
}


fn move_left(transform: &mut Transform, standing_timer: &mut StandingTime, flip: &mut Sprite, time: &Res<Time>){
    standing_timer.timer.tick(time.delta());
    if standing_timer.timer.finished() {
        transform.translation.x -= 10.0;
        flip.flip_x = true;
    }
}
fn move_right(transform: &mut Transform, standing_timer: &mut StandingTime, flip: &mut Sprite, time: &Res<Time>){
    standing_timer.timer.tick(time.delta());
    if standing_timer.timer.finished() {
        transform.translation.x += 10.0;
        flip.flip_x = false;
    }
}
fn move_up(transform: &mut Transform, standing_timer: &mut StandingTime, time: &Res<Time>){
    standing_timer.timer.tick(time.delta());
    if standing_timer.timer.finished() {
        transform.translation.y += 10.0
    }
}
fn move_down(transform: &mut Transform, standing_timer: &mut StandingTime, time: &Res<Time>){
    standing_timer.timer.tick(time.delta());
    if standing_timer.timer.finished() {
        transform.translation.y -= 10.0
    }
}