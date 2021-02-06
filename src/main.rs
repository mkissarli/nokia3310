use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::image::LoadTexture;
use sdl2::render::{ Canvas, Texture };

use specs::*;

mod keyboard;
mod sdl_helpers;
mod components;
mod systems;
mod entity_creator;

const WINDOW_WIDTH: u32 = 48;
const WINDOW_HEIGHT: u32 = 84;
const FPS: f32 = 240.0;
const INIT_SCALE: u32 = 4;

fn main() -> Result<(), String> {

    let mut world = World::new();
    init_insert(&mut world);

    
    // SDL setup
    let (mut canvas, mut event_pump) = match sdl_helpers::sdl_init("Pong", WINDOW_WIDTH, WINDOW_HEIGHT, INIT_SCALE) {
        Ok(x) => { x },
        Err(e) => { panic!("There was an error creating event_pump: {:?}", e); }
    };

    let texture_creator = canvas.texture_creator();
    let spritesheet = texture_creator.load_texture("assets/spritesheet.png")?;
    let splashscreen = texture_creator.load_texture("assets/splashscreen.png")?;
    let fonts = texture_creator.load_texture("assets/real_fonts.png")?;
    
    let keyboard: Option<keyboard::Keyboard> = None;
    world.insert(keyboard);
   
    let mut dispatcher = DispatcherBuilder::new()
        .with(systems::Gravity, "gravity", &[])
        .with(systems::PlayerMovement, "player_movement", &[])
        //.with(systems::PlayerUseFuel, "player_use_fuel", &[])
        .with(systems::UpdatePosition, "update_position", &["player_movement",])
        .with(systems::BoundaryCheck, "boundary_check", &["update_position"])
        //.with(systems::GameOverCheck, "game_over_check", &[])
        .with(systems::AsteroidSpawner, "asteroid_spawner", &[])
        .with(systems::AsteroidCollision, "asteroid_collision", &[])
        .with(systems::PlayerShoot, "player_shoot", &["asteroid_collision"])
        .with(systems::BulletCollision, "bullet_collision", &["update_position"])
        .build();


    let mut total_time: f32 = 0.0;
   
    canvas.clear();
    canvas.copy(&splashscreen, None, None);
    canvas.present();


    // Splash Screen
    'startup: loop {
        sdl_helpers::sdl_rescale(&mut canvas, WINDOW_WIDTH, WINDOW_HEIGHT);

        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown { .. } => {
                    break 'startup;
                },
                _ => { }
            }
        }

        canvas.present();
    }
    
    // Game Loop
    'main: loop {
        // Clean Screen
        canvas.clear();       
        
        // Resize
        sdl_helpers::sdl_rescale(&mut canvas, WINDOW_WIDTH, WINDOW_HEIGHT);
       
        // Keyboard

        // Main
        //if !world.read_resource::<components::GameOver>().0 {
            // Update DeltaTime        
        {
            let mut delta = world.write_resource::<components::DeltaTime>();
            *delta = components::DeltaTime(std::time::Instant::now());
        }
        
        let mut keyboard = None;
        
       
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
                    //keyboard = Some(keyboard::Keyboard::Accelerate);
                    //*world.write_resource() = components::Accelerating(true);
                    keyboard = Some(keyboard::Keyboard::Move(keyboard::Direction::Up))
                },
                Event::KeyDown { keycode: Some(Keycode::Down), repeat: false, .. } => {
                    keyboard = Some(keyboard::Keyboard::Move(keyboard::Direction::Down));
                },
                
                // Direction button up.
                Event::KeyUp { keycode: Some(Keycode::Left), repeat: false, .. } |
                Event::KeyUp { keycode: Some(Keycode::Right), repeat: false, .. } |
                Event::KeyUp { keycode: Some(Keycode::Down), repeat: false, .. } |
                Event::KeyUp { keycode: Some(Keycode::Up), repeat: false, ..} => {
                    keyboard = Some(keyboard::Keyboard::Stop);
                },
                
                //Event::KeyUp { keycode: Some(Keycode::Up), repeat: false, .. } => {
                //    *world.write_resource() = components::Accelerating(false);
                //},
                    
                Event::KeyDown { keycode: Some(Keycode::Space), repeat: false, .. } => {
                    keyboard = Some(keyboard::Keyboard::Other);
                    let mut shooting = world.write_resource::<components::Shooting>();
                    shooting.is_shooting = true;
                },
                Event::KeyUp { keycode: Some(Keycode::Space), repeat: false, .. } => {
                    keyboard = Some(keyboard::Keyboard::Other);
                    let mut shooting = world.write_resource::<components::Shooting>();
                    shooting.is_shooting = false;
                },
                    
                _ => {
                    //keyboard = Some(keyboard::Keyboard::Other);
                }
            }
        }

        *world.write_resource() = keyboard;

        if !world.read_resource::<components::GameOver>().0 {
            // Run our Systems
            dispatcher.dispatch(&mut world);
             
            // Timestep
            let mut score = world.write_resource::<components::Score>();
            let time = world.read_resource::<components::DeltaTime>();
            if score.time < (60.0 / FPS) {
                score.time = score.time + time.0.elapsed().as_secs_f32();
                score.total_time = score.total_time + time.0.elapsed().as_secs_f32();
            }
            else {
                score.time = score.time - (60.0 / FPS);
                let mut spawn = world.write_resource::<components::Spawner>();
                spawn.can_spawn = true;
                // Render Everything
                // render(..)
                sdl_helpers::render(&mut canvas, &spritesheet, world.system_data());
            }
            let mut shooting = world.write_resource::<components::Shooting>();
            shooting.time = shooting.time - time.0.elapsed().as_secs_f32();
        }
        else {
            canvas.clear();

            
            render_text(&mut canvas, &fonts, "Sco".to_string(), components::Position{ x: 2.0, y: 10.0 });
            render_text(&mut canvas, &fonts, "re:".to_string(), components::Position{ x: 2.0, y: 29.0 });
            render_text(&mut canvas, &fonts, world.read_resource::<components::Score>().points.to_string(), components::Position{ x: 2.0, y: 48.0});
            
            canvas.present();
            match keyboard.clone() {
                Some(k) => {
                    match k {
                        keyboard::Keyboard::Stop => {},
                        _ => {
                            println!("k: {:?}", k);
                            world = World::new();
                            init_insert(&mut world);
                        }
                    }
                },
                _=> {}
            }
        }
           
        // Clean up.
        world.maintain();
       
   }

    println!("Total Time: {}", world.read_resource::<components::Score>().total_time);
    
    Ok(())
}

fn render_text(canvas: &mut Canvas<sdl2::video::Window>,
               fonts: &Texture,
               s: String, pos: components::Position){
    sdl_helpers::render_font(
        canvas,
        &fonts,
        components::Position{x:2.0, y:2.0},
        components::Position{x:4.0, y:4.0},
        10,14,18,
        s,
        pos);
}

fn init_insert(world: &mut World) {
    // Insert
    world.register::<components::Asteroid>();
    world.register::<components::Player>();
    world.register::<components::Position>();
    world.register::<components::Velocity>();
    world.register::<components::Sprite>();
    world.register::<components::GravityAffected>();
    world.register::<components::Collider>();
    world.register::<components::FuelManager>();
    world.register::<components::Bullet>();
    
    // Insert Resources
    world.insert(components::DeltaTime(std::time::Instant::now()));
    world.insert(components::Score { total_time: 0.0, time: 60.0 / FPS, points: 0.0 });
    world.insert(components::GameOver(false));
    world.insert(components::Spawner {
        can_spawn: true,
        delay: 2.0
    });
    world.insert(components::Shooting{
        is_shooting: false,
        delay: 60.0 / FPS,
        time: 0.0
    });
    
    let keyboard: Option<keyboard::Keyboard> = None;
    world.insert(keyboard);

    world.insert(components::Accelerating(false));
    
    entity_creator::create_aeroplane(
        world.create_entity(),
        components::Position { x: 1.0, y: WINDOW_HEIGHT as f32 - 10.0 },
        components::Sprite {
            initial_position: components::Position { x: 0.0, y: 0.0 },
            animation_frames: vec![2],
            time_between_frames: 1.0,
            current_frame: 0,
            current_time: 0.0,
            current_animation: 0,
            width: 7,
            height: 8
        },
        components::Collider {
            relative_position: components::Position { x: 0.0, y: 0.0 },
            width: 7.0,
            height: 8.0
        });
 
}

