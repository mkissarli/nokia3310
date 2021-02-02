use crate::components::*;
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
            println!("Vel is: {}", vel.y)
        }
    }
}

pub struct AirResistance;

pub struct PlayerMovement;

// If each asteroid/pickup does the collision against the player, then we have
// fewer checks? 
pub struct AsteroidCollision;
pub struct PickupCollision;

pub struct UpdatePosition;

pub struct UpdateAnimation;
