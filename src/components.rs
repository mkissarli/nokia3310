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
    // Need some kind of animating/sprite selection.
    pub width: f32,
    pub height: f32
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

