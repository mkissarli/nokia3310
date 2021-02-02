use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::image::LoadTexture;

use specs::*;

mod keyboard;
mod sdl_helpers;
mod components;
mod systems;
mod entity_creator;

const WINDOW_WIDTH: u32 = 84;
const WINDOW_HEIGHT: u32 = 48;
const FPS: f32 = 15.0;

fn main() -> Result<(), String> {
    
    let mut world = World::new();
    init_insert(&mut world);

    
    // SDL setup
    let (mut canvas, mut event_pump) = match sdl_helpers::sdl_init("Pong", WINDOW_WIDTH, WINDOW_HEIGHT) {
        Ok(x) => { x },
        Err(e) => { panic!("There was an error creating event_pump: {:?}", e); }
    };

    let texture_creator = canvas.texture_creator();
    let sprite_sheet = texture_creator.load_texture("assets/spritesheet.png");
        

    entity_creator::create_aeroplane(
        world.create_entity(),
        components::Position { x: 1.0, y: 1.0},
        components::Sprite {
            initial_position: components::Position { x: 0.0, y: 0.0 },
            animation_frames: vec![2, 2],
            time_between_frames: 1.0,
            current_frame: 0,
            width: 7,
            height: 8
        });
    
    let mut dispatcher = DispatcherBuilder::new()
        //.with(systems::UpdateScore, "update_score", &[])
    //.with(systems::TimeStepManager, "time_step", &[])
        //.with(BallCollision, "ball_collision", &["update_pos"])
        .build();


    let mut total_time: f32 = 0.0;
    
    // Game Loop
    'main: loop {
        // Clean Screen
        canvas.clear();

        // Resize
        sdl_helpers::sdl_rescale(&mut canvas, WINDOW_WIDTH, WINDOW_HEIGHT);
        
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
                    keyboard = Some(keyboard::Keyboard::Move(keyboard::Direction::Left));
                },
                Event::KeyDown { keycode: Some(Keycode::Right), repeat: false, .. } => {
                    keyboard = Some(keyboard::Keyboard::Move(keyboard::Direction::Right));
                },
                Event::KeyDown { keycode: Some(Keycode::Up), repeat: false, .. } => {
                    keyboard = Some(keyboard::Keyboard::Move(keyboard::Direction::Up));
                },
                Event::KeyDown { keycode: Some(Keycode::Down), repeat: false, .. } => {
                    keyboard = Some(keyboard::Keyboard::Move(keyboard::Direction::Down));
                },

                // Direction button up.
                Event::KeyUp { keycode: Some(Keycode::Left), repeat: false, .. } |
                Event::KeyUp { keycode: Some(Keycode::Right), repeat: false, .. } |
                Event::KeyUp { keycode: Some(Keycode::Up), repeat: false, .. } |
                Event::KeyUp { keycode: Some(Keycode::Down), repeat: false, .. } => {
                    keyboard = Some(keyboard::Keyboard::Stop);
                },

                // Accelerate
                Event::KeyDown { keycode: Some(Keycode::Space), repeat: false, .. } => {
                    keyboard = Some(keyboard::Keyboard::Accelerate);
                }
                
                _ => {}
            }
        }

        *world.write_resource() = keyboard;

        // Clean up.
        world.maintain();
        
        // Timestep
        let mut score = world.write_resource::<components::Score>();
        if score.time < (60.0 / FPS) {
            let time = world.read_resource::<components::DeltaTime>();
            score.time = score.time + time.0.elapsed().as_secs_f32();
            score.total_time = score.total_time + time.0.elapsed().as_secs_f32();
        }
        else {
            score.time = score.time - (60.0 / FPS);
            // Render Everything
            // render(..)
        }
   }

    println!("Total Time: {}", world.read_resource::<components::Score>().total_time);
    
    Ok(())
}

fn init_insert(world: &mut World) {
    // Insert
    world.register::<components::Asteroid>();
    world.register::<components::Player>();
    world.register::<components::Position>();
    world.register::<components::Velocity>();
    world.register::<components::Sprite>();
    world.register::<components::Gravity>();
    world.register::<components::Collider>();

    // Insert Resources
    world.insert(components::DeltaTime(std::time::Instant::now()));
    world.insert(components::Score { total_time: 0.0, time: 0.0 });
    
    let keyboard: Option<keyboard::Keyboard> = None;
    world.insert(keyboard);
}

