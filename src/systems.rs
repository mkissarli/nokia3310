use crate::components::*;
use crate::keyboard;

use specs::{ Write, Read, WriteStorage, ReadStorage, System, Join };

pub struct Gravity;

impl <'a> System<'a> for Gravity {
    type SystemData = (
        ReadStorage<'a, GravityAffected>,
        WriteStorage<'a, Velocity>,
        Read<'a, DeltaTime>);

    fn run(&mut self, data: Self::SystemData){
        let (gravities, mut velocities, time) = data;
        let delta = time.0;
        
        for (g, vel) in (&gravities, &mut velocities).join() {
            vel.y = vel.y - g.force * delta.elapsed().as_secs_f32();
        }
    }
}

pub struct AirResistance;

pub struct PlayerMovement;

impl <'a> System<'a> for PlayerMovement {
    type SystemData = (
        ReadStorage<'a, Player>,
        WriteStorage<'a, Velocity>,
        Read<'a, Option<keyboard::Keyboard>>);

    fn run(&mut self, data: Self::SystemData){
        let (players, mut velocities, d_keyboard) = data;

        let keyboard = match &*d_keyboard {
            Some(k) => k,
            None => return, // no change
        };
 
        
        for (_p, vel) in (&players, &mut velocities).join(){
            match keyboard {
                keyboard::Keyboard::Move(direction) => {
                    match direction {
                        keyboard::Direction::Left => { vel.x = -15.0; },
                        keyboard::Direction::Right => { vel.x = 15.0; },
                        _ => { vel.x = 0.0; }
                    }
                },
                _ => { vel.x = 0.0; }
            }
        }
    }
}

const FUEL_FORCE: f32 = 15.0;
pub struct PlayerUseFuel;

impl <'a> System<'a> for PlayerUseFuel {
    type SystemData = (
        ReadStorage<'a, Player>,
        WriteStorage<'a, Velocity>,
        Read<'a, Option<keyboard::Keyboard>>,
        Read<'a, DeltaTime>);

    fn run(&mut self, data: Self::SystemData){
        let (players, mut velocities, d_keyboard, time) = data;
        let delta = time.0;
        
        let keyboard = match &*d_keyboard {
            Some(k) => k,
            None => return, // no change
        };
        
        for (_p, vel) in (&players, &mut velocities).join(){
            match keyboard {
                keyboard::Keyboard::Accelerate => {
                    // Fuel management code here.
                    vel.y = vel.y - FUEL_FORCE;
                },
                _ => {}
            }
        }
        
    }
}

// If each asteroid/pickup does the collision against the player, then we have
// fewer checks? 
pub struct AsteroidCollision;
pub struct PickupCollision;

pub struct UpdatePosition;

impl <'a> System<'a> for UpdatePosition {
    type SystemData = (
        ReadStorage<'a, Velocity>,
        WriteStorage<'a, Position>,
        Read<'a, DeltaTime>);

    fn run(&mut self, data: Self::SystemData){
        let (velocity, mut position, time) = data;
        let delta = time.0;

        for (vel, pos) in (&velocity, &mut position).join() {
            pos.x = pos.x + vel.x * delta.elapsed().as_secs_f32();
            pos.y = pos.y + vel.y * delta.elapsed().as_secs_f32();
        }
    }
}

pub struct UpdateAnimation;
