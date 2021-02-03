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
