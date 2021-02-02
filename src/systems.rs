use crate::components::*;
use specs::{ Read, System, Join };

pub struct TimeStepManager;

impl<'a> System<'a> for TimeStepManager {
    type SystemData = Read<'a, DeltaTime>;

    fn run(&mut self, time: Self::SystemData) {
        while time.0.elapsed().as_secs_f32() < 60.0 / 15.0 {}
    }
}

