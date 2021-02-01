// FLAGS

#[derive(Component, Default, Debug)]
#[storage(NullStorage)]
struct Asteroid; 

#[derive(Component, Default, Debug)]
#[storage(NullStorage)]
struct Player; 

// Components

#[derive(Component, Debug)]
#[storage(DenseVecStorage)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Component, Debug)]
#[storage(DenseVecStorage)]
struct Velocity {
    x: f32,
    y: f32,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Sprite {
    // Need some kind of animating/sprite selection.
    width: f32,
    height: f32
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Gravity {
    force: f32
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
struct Collider {
    // Relative from top left of sprite (so negatives would be larger.)
    relative_position: Position,
    width: f32,
    height: f32
}

// Resources

struct DeltaTime(std::time::Instant);

impl Default for DeltaTime {
    fn default() -> DeltaTime { DeltaTime(std::time::Instant::now()) }
}

