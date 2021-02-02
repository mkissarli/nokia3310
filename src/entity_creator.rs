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
        .with(Player)
        .build();
}
