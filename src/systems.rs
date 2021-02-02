use crate::components::*;
use specs::{ Write, Read, System, Join };

pub struct TimeStepManager;

impl<'a> System<'a> for TimeStepManager {
    type SystemData = Read<'a, DeltaTime>;

    fn run(&mut self, time: Self::SystemData) {
        while time.0.elapsed().as_secs_f32() < 60.0 / 15.0 {}
    }
}

pub struct Gravity;

pub struct PlayerMovement;

// If each asteroid/pickup does the collision against the player, then we have
// fewer checks? 
pub struct AsteroidCollision;
pub struct PickupCollision;

pub struct UpdatePosition;

pub struct UpdateAnimation;

// Doesn't work as a dispatch...
pub struct UpdateScore;
impl<'a> System<'a> for UpdateScore {
    type SystemData = (
        Write<'a, Score>,
        Read<'a, DeltaTime>);

    fn run(&mut self, data: Self::SystemData){
        let (mut score, time) = data;

        //score.total_time = score.total_time + time.0.elapsed().as_secs_f32();
    }
}
