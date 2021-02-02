use crate::components::*;
use specs::{ Write, Read, System, Join };

pub struct Gravity;

pub struct AirResistance;

pub struct PlayerMovement;

// If each asteroid/pickup does the collision against the player, then we have
// fewer checks? 
pub struct AsteroidCollision;
pub struct PickupCollision;

pub struct UpdatePosition;

pub struct UpdateAnimation;
