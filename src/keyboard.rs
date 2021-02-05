#[derive(Copy, Clone, Debug)]
pub enum Keyboard {
    Stop,
    Move(Direction),
    Accelerate,
    Other
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

