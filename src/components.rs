use specs::*;

// FLAGS

#[derive(Component, Default, Debug)]
#[storage(NullStorage)]
pub struct Asteroid; 

#[derive(Component, Default, Debug)]
#[storage(NullStorage)]
pub struct Player; 

#[derive(Component, Default, Debug)]
#[storage(NullStorage)]
pub struct AsteroidSpawner;

#[derive(Component, Default, Debug)]
#[storage(NullStorage)]
pub struct Bullet;

// Components

#[derive(Component, Debug)]
#[storage(DenseVecStorage)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Component, Debug)]
#[storage(DenseVecStorage)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Sprite {
    // Top left of first animation.
    pub initial_position: Position,
    // Number of frames in each animation.
    pub animation_frames: Vec<u32>,
    pub time_between_frames: f32,

    pub current_animation: u32,
    pub current_time: f32,
    pub current_frame: u32,

    // Animations are all the same size in this.
    pub width: u32,
    pub height: u32
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct GravityAffected {
    pub force: f32
}

impl Default for GravityAffected {
    fn default() -> GravityAffected {
        GravityAffected {
            force: -9.8
        }
    }
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Collider {
    // Relative from top left of sprite (so negatives would be larger.)
    pub relative_position: Position,
    pub width: f32,
    pub height: f32
}

#[derive(Component, Debug)]
#[storage(DenseVecStorage)]
pub struct FuelManager {
    pub amount_left: f32,
    pub upward_force: f32,
    pub cost_per_second: f32
}

// Resources

pub struct DeltaTime(pub std::time::Instant);

impl Default for DeltaTime {
    fn default() -> DeltaTime { DeltaTime(std::time::Instant::now()) }
}

pub struct Score{
    pub points: f32,
    pub total_time: f32,
    pub time: f32,
}

impl Default for Score {
    fn default() -> Score { Score{ total_time: 0.0, time: 0.0, points: 0.0 }} 
}

pub struct Accelerating(pub bool);

impl Default for Accelerating {
    fn default() -> Accelerating { Accelerating(false) }
}

#[derive(Clone, Copy)]
pub enum GameState {
    GamePlay,
    GameOver
}

impl Default for GameState {
    fn default() -> GameState { GameState::GamePlay }
}

pub struct GameOver(pub bool);

impl Default for GameOver {
    fn default() -> GameOver { GameOver(false) }
}

pub struct Spawner{
    pub can_spawn: bool,
    pub delay: f32
}

impl Default for Spawner {
    fn default() -> Spawner {
        Spawner {
            can_spawn: true,
            delay: 2.0
        }
    }
}

pub struct Shooting {
    pub is_shooting: bool,
    pub delay: f32,
    pub time: f32
}

impl Default for Shooting {
    fn default() -> Shooting {
        Shooting {
            is_shooting: true,
            delay: 4.0,
            time: 0.0
        }
    }
}
