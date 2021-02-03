use crate::components::*;
use crate::keyboard;
use crate::{ WINDOW_WIDTH, WINDOW_HEIGHT };

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

const FUEL_FORCE: f32 = 10.0;
pub struct PlayerUseFuel;

impl <'a> System<'a> for PlayerUseFuel {
    type SystemData = (
        ReadStorage<'a, Player>,
        WriteStorage<'a, Velocity>,
        WriteStorage<'a, FuelManager>,
        Read<'a, Accelerating>,
        Read<'a, DeltaTime>);

    fn run(&mut self, data: Self::SystemData){
        let (players, mut velocities, mut fuels, acc, time) = data;
        let delta = time.0;
        
        for (_p, vel, fuel) in (&players, &mut velocities, &mut fuels).join(){
            match acc.0 {
                true => {
                    // Fuel management code here.
                    if fuel.amount_left > 0.0 {
                        vel.y = -fuel.upward_force;
                        fuel.amount_left = fuel.amount_left - fuel.cost_per_second * delta.elapsed().as_secs_f32();
                    }
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

pub struct AsteroidBoundaryCheck;
pub struct AsteroidSpawner;

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

pub struct BoundaryCheck;

impl<'a> System <'a> for BoundaryCheck {
    type SystemData = (
        ReadStorage<'a, Player>,
        ReadStorage<'a, Sprite>,
        WriteStorage<'a, Position>,
        Write<'a, GameOver>);

    fn run(&mut self, data: Self::SystemData){
        let (players, sprites, mut positions, mut game_over) = data;

        for (_p, sprite, pos) in (&players, &sprites, &mut positions).join(){
            if pos.x < 0.0 {
                pos.x = 0.0;
            }
            else if pos.x > (WINDOW_WIDTH as f32 - sprite.width as f32) {
                pos.x = WINDOW_WIDTH as f32 - sprite.width as f32;
            }

            if pos.y < 0.0 {
                pos.y = 0.0;
            }
            else if pos.y > WINDOW_HEIGHT as f32 - sprite.height as f32 {
                game_over.0 = true;
            }
        }
    }
}

pub struct GameOverCheck;

impl<'a> System<'a> for GameOverCheck {
    type SystemData = Read<'a, GameOver>;

    fn run(&mut self, game_over: Self::SystemData){
        if game_over.0 == true {
            println!("GameOver");
        }
    }
}
