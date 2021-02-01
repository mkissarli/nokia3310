#[derive(Debug)]
pub enum Keyboard {
    Stop,
    Move(Direction),
    Accelerate
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

