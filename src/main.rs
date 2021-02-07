use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::image::LoadTexture;
use sdl2::render::{ Canvas, Texture };

use specs::*;
use emscripten_main_loop::*;

mod keyboard;
mod sdl_helpers;
mod components;
mod systems;
mod entity_creator;
//mod externs;

const WINDOW_WIDTH: u32 = 48;
const WINDOW_HEIGHT: u32 = 84;
const FPS: f32 = 240.0;
const INIT_SCALE: u32 = 4;

fn build_main_dispatcher() -> Dispatcher<'static, 'static> {
    DispatcherBuilder::new()
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
        .build()
}

fn main(){
    let mut game = Game::new().unwrap();
    //run();
    // Game Loop

    // Desktop
    
    'main: loop{
        let res = game.main_loop();
        match res {
            MainLoopEvent::Continue => {continue 'main;},
            MainLoopEvent::Terminate => {break 'main;},
        }
    }
}

/*#[no_mangle]
pub fn run() {
    let mut game = Game::new().unwrap();
    externs::set_main_loop_callback(||{
        web(&mut game);
    });
    
}

pub fn web(game: &mut Game){
    let res = game.main_loop();
    match res {
        MainLoopEvent::Continue => {},
        MainLoopEvent::Terminate => {externs::cancel_main_loop();}
    }
}*/

/*impl MainLoop for Game{
    fn main_loop(&mut self) -> MainLoopEvent { MainLoopEvent::Terminate }
}*/

impl  MainLoop for Game{
    fn main_loop(&mut self) -> MainLoopEvent {
        // Clean Screen
         self.canvas.clear();       
        
        // Resize
        sdl_helpers::sdl_rescale(&mut self.canvas, WINDOW_WIDTH, WINDOW_HEIGHT);
        
        if self.world.read_resource::<components::Startup>().0{
            self.canvas.copy(&self.splashscreen, None, None);
            self.canvas.present();
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        return MainLoopEvent::Terminate;
                    },
                    Event::KeyDown { .. } => {
                        let mut s = self.world.write_resource::<components::Startup>();
                        *s = components::Startup(false);
                    },
                    _ => { return MainLoopEvent::Continue; }
                }
            }
        }
        // Keyboard

        // Main
        //if !world.read_resource::<components::GameOver>().0 {
        // Update DeltaTime        
        {
            let mut delta = self.world.write_resource::<components::DeltaTime>();
            *delta = components::DeltaTime(std::time::Instant::now());
        }
        
        let mut keyboard = None;
        
        
        for event in self.event_pump.poll_iter() {
            match event {
                // Window Management
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    return MainLoopEvent::Terminate;
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
               
                Event::KeyDown { keycode: Some(Keycode::Space), repeat: false, .. } => {
                    keyboard = Some(keyboard::Keyboard::Other);
                    let mut shooting = self.world.write_resource::<components::Shooting>();
                    shooting.is_shooting = true;
                },
                Event::KeyUp { keycode: Some(Keycode::Space), repeat: false, .. } => {
                    keyboard = Some(keyboard::Keyboard::Other);
                    let mut shooting = self.world.write_resource::<components::Shooting>();
                    shooting.is_shooting = false;
                },
                
                _ => {
                    //keyboard = Some(keyboard::Keyboard::Other);
                }
            }
        }

        *self.world.write_resource() = keyboard;

        if !self.world.read_resource::<components::GameOver>().0 {
            // Run our Systems
            self.dispatcher.dispatch(&mut self.world);
            
            // Timestep
            let mut score = self.world.write_resource::<components::Score>();
            let time = self.world.read_resource::<components::DeltaTime>();
            if score.time < (60.0 / FPS) {
                score.time = score.time + time.0.elapsed().as_secs_f32();
                score.total_time = score.total_time + time.0.elapsed().as_secs_f32();
            }
            else {
                score.time = score.time - (60.0 / FPS);
                let mut spawn = self.world.write_resource::<components::Spawner>();
                spawn.can_spawn = true;
                // Render Everything
                // render(..)
                sdl_helpers::render(&mut self.canvas, &self.spritesheet, self.world.system_data());
            }
            let mut shooting = self.world.write_resource::<components::Shooting>();
            shooting.time = shooting.time - time.0.elapsed().as_secs_f32();
        }
        else {
            self.canvas.clear();

            
            render_text(&mut self.canvas, &self.fonts, "Score:".to_string(), components::Position{ x: 2.0, y: 10.0 });
            render_text(&mut self.canvas, &self.fonts, self.world.read_resource::<components::Score>().points.to_string(), components::Position{ x: 2.0, y: 20.0});
            
            self.canvas.present();
            match keyboard.clone() {
                Some(k) => {
                    match k {
                        keyboard::Keyboard::Stop => {},
                        _ => {
                            self.world = Game::reset();
                        }
                    }
                },
                _=> {}
            }
        }
        
        // Clean up.
        self.world.maintain();

        MainLoopEvent::Continue
    }
}

fn render_text(canvas: &mut Canvas<sdl2::video::Window>,
               fonts: &Texture,
               s: String, pos: components::Position){
    sdl_helpers::render_font(
        canvas,
        &fonts,
        components::Position{x:0.0, y:8.0},
        components::Position{x:0.0, y:0.0},
        7,8,32,
        s,
        pos);
}

impl Game {
    pub fn new() -> Result<Self, String> {
        let mut world = Game::reset();
        
        // SDL setup
        let (mut canvas, mut event_pump) = match sdl_helpers::sdl_init("Pong", WINDOW_WIDTH, WINDOW_HEIGHT, INIT_SCALE) {
            Ok(x) => { x },
            Err(e) => { panic!("There was an error creating event_pump: {:?}", e); }
        };

        let mut texture_creator = canvas.texture_creator();
        let spritesheet = texture_creator.load_texture("assets/spritesheet.png")?;
        let splashscreen = texture_creator.load_texture("assets/splashscreen.png")?;
        let fonts = texture_creator.load_texture("assets/joshs_font.png")?;
        
        //let keyboard: Option<keyboard::Keyboard> = None;
        //world.insert(keyboard);
        
        let dispatcher = build_main_dispatcher();

        let mut total_time: f32 = 0.0;
        
        canvas.clear();
        canvas.copy(&splashscreen, None, None);
        canvas.present();

        Ok(Game{
            canvas: canvas,
            event_pump: event_pump,
            world: world,
            //texture_creator: &mut texture_creator, 
            spritesheet: spritesheet,
            splashscreen: splashscreen,
            fonts: fonts,
            dispatcher: dispatcher,
            total_time: total_time
        })
    }

    fn reset() -> World {
        let mut world = World::new();

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
        world.insert(components::Startup(true));
        
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

        world
    }
}

pub struct Game {
    canvas: Canvas<sdl2::video::Window>, 
    event_pump: sdl2::EventPump,
    world: World,
    //texture_creator: &'a mut sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    spritesheet: Texture,
    splashscreen: Texture,
    fonts: Texture,
    dispatcher: Dispatcher<'static, 'static>,
    total_time: f32
}
