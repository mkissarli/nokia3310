use specs::*;

// FLAGS

#[derive(Component, Default, Debug)]
#[storage(NullStorage)]
pub struct Asteroid; 

#[derive(Component, Default, Debug)]
#[storage(NullStorage)]
pub struct Player; 

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
pub struct Gravity {
    pub force: f32
}

impl Default for Gravity {
    fn default() -> Gravity {
        Gravity {
            force: 9.8
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

// Resources

pub struct DeltaTime(pub std::time::Instant);

impl Default for DeltaTime {
    fn default() -> DeltaTime { DeltaTime(std::time::Instant::now()) }
}

pub struct Score {
    pub total_time: f32,
    pub time: f32,
}

impl Default for Score {
    fn default() -> Score { Score{ total_time: 0.0, time: 0.0 }} 
}
