use bevy::prelude::*;

#[derive(Component)]
pub struct Rat; 

#[derive(Component)]
pub struct HungerBar; 

#[derive(Component)]
pub struct PlaytimeBar; 

#[derive(Component)]
pub enum Direction {
    Stand,
    Die, 
}

#[derive(Component)]
pub struct StandingTime {
    pub timer: Timer
} 

#[derive(Component)]
pub struct HungerTime {
   pub timer: Timer,
}

#[derive (Component)]
pub struct PlaytimeTimer {
    pub timer: Timer
}

#[derive(Component)]
pub struct GameTextTimer {
    pub timer: Timer
}

#[derive(Component)]
pub struct HungerAmount {
   pub amount: f32
}

#[derive(Component)]
pub struct PlaytimeAmount {
    pub amount: f32
}

#[derive(Component)]
pub struct StartingPosition {
   pub y: f32
}

#[derive(Component)]
pub struct Toggle {
    pub toggle: bool
}

#[derive(Component)]
pub struct Playtime {
    pub opponent_rps: i32
}

#[derive (Component)]
pub struct WinGameText;

#[derive (Component)]
pub struct MainGameText;

#[derive(Component)]
pub struct StartActivated;

#[derive(Component)]
pub struct StartDectivated;