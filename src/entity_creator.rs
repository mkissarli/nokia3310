use specs::Builder;
use crate::components::*;

pub fn create_aeroplane(
    builder: specs::world::EntityBuilder,
    pos: Position,
    sprite: Sprite,
    col: Collider){

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
        .with(col)
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
            initial_position: Position { x: 0.0, y: 8.0 },
            animation_frames: vec![2],
            current_animation: 0,
            current_frame: 0,
            current_time: 0.0,
            time_between_frames: 1.0,
            width: 7,
            height: 8
        })
        .with(Collider {
            relative_position: Position { x: 1.0, y: 1.0},
            width: 5.0,
            height: 6.0
        })
        .with(GravityAffected { force: -9.8 })
        .with(Asteroid)
        .build();
}

pub fn create_bullet(
    builder: specs::world::LazyBuilder,
    pos: Position){
    builder
        .with(pos)
        .with(Sprite {
            initial_position: Position { x: 0.0, y: 17.0 },
            animation_frames: vec![1],
            current_animation: 0,
            current_frame: 0,
            current_time: 0.0,
            time_between_frames: 1.0,
            width: 1,
            height: 1
        })
        .with(Collider {
            relative_position: Position { x: 0.0, y: 0.0 },
            width: 1.0,
            height: 1.0
        })
        .with(Bullet)
        .build();
}
