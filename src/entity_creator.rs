use specs::Builder;
use crate::components::*;

pub fn create_aeroplane(
    builder: specs::world::EntityBuilder,
    pos: Position,
    sprite: Sprite){

    builder
        .with(pos)
        .with(Velocity { x: 0.0, y: 0.0 })
        .with(GravityAffected { force: -9.8 })
        .with(sprite)
        .with(FuelManager {
            cost_per_second: 10.0,
            upward_force: 5.0,
            amount_left: 100.0,
        })
        .with(Player)
        .build();
}

pub fn create_asteroid(
    builder: specs::world::LazyBuilder,
    pos: Position,
    direction: Velocity){

    builder
        .with(pos)
        .with(direction)
        .with(Sprite {
            initial_position: Position { x: 0.0, y: 9.0 },
            animation_frames: vec![2],
            current_animation: 0,
            current_frame: 0,
            current_time: 0.0,
            time_between_frames: 1.0,
            width: 7,
            height: 8
        })
        .with(GravityAffected { force: -9.8 })
        .with(Asteroid)
        .build();
}
