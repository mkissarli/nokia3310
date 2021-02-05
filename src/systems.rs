use crate::components::*;
use crate::entity_creator;
use crate::keyboard;
use crate::{ WINDOW_WIDTH, WINDOW_HEIGHT };

use rand::Rng;

    
use specs::{ Write, Read, WriteStorage, ReadStorage, System, Join, Entities, LazyUpdate };
use specs::world::EntitiesRes;

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
                        keyboard::Direction::Up => { vel.y = -15.0; },
                        keyboard::Direction::Down => { vel.y = 15.0; }
                        _ => { vel.x = 0.0; vel.y = 0.0 }
                    }
                },
                _ => { vel.x = 0.0; vel.y = 0.0; }
            }
        }
    }
}

pub struct PlayerShoot;

impl <'a> System<'a> for PlayerShoot {
    type SystemData = (
        ReadStorage<'a, Player>,
        ReadStorage<'a, Sprite>,
        ReadStorage<'a, Position>,
        Write<'a, Shooting>,
        Read<'a, LazyUpdate>,
        Read<'a, EntitiesRes>);

    fn run(&mut self, data: Self::SystemData){
        let (players, sprites, positions, mut shooting, lazy, d_e) = data;
        for (player, sprite, pos) in (&players, &sprites, &positions).join(){
            if shooting.time <= 0.0 && shooting.is_shooting{
                //println!("shoot shoot");
                shooting.time = shooting.delay;
                entity_creator::create_bullet(
                    lazy.create_entity(&d_e),
                    Position { x: pos.x + (sprite.width / 2) as f32, y: pos.y - 5.0 },
                    Velocity { x: 0.0, y: -15.0 });
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

impl <'a> System<'a> for AsteroidCollision {
    type SystemData = (
        ReadStorage<'a, Player>,
        ReadStorage<'a, Asteroid>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Sprite>,
        ReadStorage<'a, Collider>,
        Write<'a, GameOver>);

    fn run(&mut self, data: Self::SystemData){
        let (players, asteroids, positions, sprites, colliders, mut game_over) = data;

        // Could probs optimise this out?
        for (_p, p_pos, p_sprite, p_collider) in (&players, &positions, &sprites, &colliders).join(){
            for (_a, a_pos, a_sprite, a_collider) in (&asteroids, &positions, &sprites, &colliders).join(){
                if p_pos.x < a_pos.x + a_collider.relative_position.x + a_collider.width &&
                    p_pos.x + p_collider.relative_position.x + p_collider.width > a_pos.x &&
                    p_pos.y < a_pos.y + a_collider.relative_position.y + a_collider.height &&
                    p_pos.y + p_collider.relative_position.y + p_collider.height > a_pos.y {
                        game_over.0 = true;
                    }
            }
        }
    }
}

pub struct BulletCollision;

impl <'a> System<'a> for BulletCollision {
    type SystemData = (
        ReadStorage<'a, Asteroid>,
        ReadStorage<'a, Bullet>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Sprite>,
        ReadStorage<'a, Collider>,
        Entities<'a>,
        Write<'a, Score>);

    fn run(&mut self, data: Self::SystemData){
        let (asteroids, bullets, positions, sprites, colliders, d_e, mut score) = data;

        for(_a, a_pos, a_collider, a_e) in (&asteroids, &positions, &colliders, &d_e).join(){
            for(_b, b_pos, b_collider, b_e) in (&bullets, &positions, &colliders, &d_e).join(){
                if a_pos.x < b_pos.x + b_collider.relative_position.x + b_collider.width &&
                    a_pos.x + a_collider.relative_position.x + a_collider.width > b_pos.x &&
                    a_pos.y < b_pos.y + b_collider.relative_position.y + b_collider.height &&
                    a_pos.y + a_collider.relative_position.y + a_collider.height > b_pos.y {

                        score.points = score.points + 10.0;
                        d_e.delete(a_e);
                        d_e.delete(b_e);
                        //println!("Asteroid got hit.");
                    } 
            }
        }
    }
}

pub struct PickupCollision;

pub struct AsteroidBoundaryCheck;

pub struct AsteroidSpawner;

impl <'a> System<'a> for AsteroidSpawner {
    type SystemData = (
        Read<'a, DeltaTime>,
        Read<'a, Score>,
        Read<'a, EntitiesRes>,
        Read<'a, LazyUpdate>,
        Write<'a, Spawner>);

    fn run(&mut self, data: Self::SystemData){
        let (time, mut score, d_e, d_l, mut spawner) = data;
        let delta = time.0;
        let mut rng = rand::thread_rng();

        //println!("Spawner: {}", 2.0 - (score.total_time % 2.0));
        if spawner.delay - (score.total_time % spawner.delay) < 0.05 && spawner.can_spawn {
            spawner.can_spawn = false;
            //for i in [0,1].iter(){
                entity_creator::create_asteroid(
                    d_l.create_entity(&d_e),
                    Position { x: rng.gen_range(0..WINDOW_WIDTH - 7) as f32, y: 0.0 },
                    Velocity { x: 0.0, y: 0.0 });
            //}
        }
    }
}
    
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
                //game_over.0 = true;
                pos.y = WINDOW_HEIGHT as f32 - sprite.height as f32;
            }
        }
    }
}

pub struct GameOverCheck;

impl<'a> System<'a> for GameOverCheck {
    type SystemData = Read<'a, GameOver>;

    fn run(&mut self, game_over: Self::SystemData){
        if game_over.0 == true {
            //println!("GameOver");
        }
    }
}
