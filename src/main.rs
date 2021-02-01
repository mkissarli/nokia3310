use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::render::{ Canvas, Texture };
use sdl2::rect::Rect;
use sdl2::image::{self, LoadTexture, InitFlag};

use specs::*;

mod keyboard::{ Keyboard, Direction };
mod sdl_helpers;
mod component;

const WINDOW_WIDTH: u32 = 84;
const WINDOW_HEIGHT: u32 = 48;

/*
struct HelloWorld;

impl<'a> System<'a> for HelloWorld {
    type SystemData = ReadStorage<'a, Position>;

    fn run(&mut self, position: Self::SystemData) {
        //use specs::Join;

        for position in position.join() {
            println!("Hello, {:?}", &position);
        }
    }
}
 */

fn main() -> Result<(), String> {
    
    let mut world = World::new();

    world.insert(DeltaTime(std::time::Instant::now()));

    let keyboard: Option<Keyboard> = None;
    world.insert(keyboard);
    // SDL setup
    let (mut canvas, mut event_pump) = match sdl_init("Pong", WINDOW_WIDTH, WINDOW_HEIGHT) {
        Ok(x) => { x },
        Err(e) => { panic!("There was an error creating event_pump: {:?}", e); }
    };

    let texture_creator = canvas.texture_creator();
   
    let mut dispatcher = DispatcherBuilder::new()
        //.with(BallCollision, "ball_collision", &["update_pos"])
        .build();

    
    // Game Loop
    'main: loop {
        // Clean Screen
        canvas.clear();

        // Resize
        sdl_helpers::sdl_rescale(&mut canvas);
        
        // Update DeltaTime
        {
            let mut delta = world.write_resource::<components::DeltaTime>();
            *delta = components::DeltaTime(std::time::Instant::now());
        }

        let mut keyboard = None;
        
        // Run our Systems
        dispatcher.dispatch(&mut world);

        // Keyboard
        for event in event_pump.poll_iter() {
            match event {

                // Window Management
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'main;
                },

                // Directional 
                Event::KeyDown { keycode: Some(Keycode::Left), repeat: false, .. } => {
                    keyboard = Some(Keyboard::Move(Direction::Left));
                },
                Event::KeyDown { keycode: Some(Keycode::Right), repeat: false, .. } => {
                    keyboard = Some(Keyboard::Move(Direction::Right));
                },
                Event::KeyDown { keycode: Some(Keycode::Up), repeat: false, .. } => {
                    keyboard = Some(Keyboard::Move(Direction::Up));
                },
                Event::KeyDown { keycode: Some(Keycode::Down), repeat: false, .. } => {
                    keyboard = Some(Keyboard::Move(Direction::Down));
                },

                // Direction button up.
                Event::KeyUp { keycode: Some(Keycode::Left), repeat: false, .. } |
                Event::KeyUp { keycode: Some(Keycode::Right), repeat: false, .. } |
                Event::KeyUp { keycode: Some(Keycode::Up), repeat: false, .. } |
                Event::KeyUp { keycode: Some(Keycode::Down), repeat: false, .. } => {
                    keyboard = Some(Keyboard::Stop);
                },

                // Accelerate
                Event::KeyDown { keycode: Some(Keycode::Space), repeat: false, .. } => {
                    keyboard = Some(Keyboard::Accelerate);
                }
                
                _ => {}
            }
        }

        *world.write_resource() = keyboard;

        // Clean up.
        world.maintain();
        
        // Render Everything.
        // render(&mut canvas, &paddle_texture, world.system_data());
   }

    Ok(())
}
