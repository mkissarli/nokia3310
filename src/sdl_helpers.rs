use sdl2::pixels::Color;
use sdl2::render::{ Canvas, Texture };
use sdl2::rect::Rect;
use sdl2::image::{self, InitFlag };//LoadTexture, InitFlag};
use specs::{ ReadStorage, Join };

use crate::components;

pub fn sdl_init(title: &str, width: u32, height: u32) ->
    Result<(Canvas<sdl2::video::Window>, sdl2::EventPump), String>{

        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;

        let _image_context = image::init(InitFlag::PNG | InitFlag::JPG)?;
        
        let window = video_subsystem.window(title, width, height)
            .position_centered()
            .resizable()
            .build()
            .expect("could not initialize video subsystem");

        
        let mut canvas = window.into_canvas().build()
            .expect("could not make a canvas");
        
        canvas.set_draw_color(Color::RGB(199, 240, 216));
        canvas.clear();
        canvas.present();
        
        Ok((canvas, sdl_context.event_pump()?))
    }

pub fn sdl_rescale(canvas: &mut Canvas<sdl2::video::Window>, window_width:u32, window_height: u32){
    let (window_w, window_h) = canvas.window().size();

    // The size width of your original screen, in this case 84x48
    let sx: f32 = window_w  as f32 / window_width as f32;
    let sy: f32 = window_h as f32 / window_height as f32;

    // Determin the smallest scale and use that so that is fits inside the window
    let scale: f32 = if sx < sy { sx }  else { sy };

    canvas.set_scale(scale, scale);

    let viewport: Rect = Rect::new(
        (((window_w as f32 - (window_width as f32 * scale as f32)) / 2.0) / scale) as i32,
        (((window_h as f32 - (window_height as f32 * scale as f32)) / 2.0) / scale) as i32,
        window_width,
        window_height);

    canvas.set_viewport(viewport);
    canvas.present();
}


type RenderType<'a> = (
        ReadStorage<'a, components::Position>,
        ReadStorage<'a, components::Sprite>);

pub fn render(
    canvas: &mut Canvas<sdl2::video::Window>,
    texture: &Texture,
    data: RenderType) -> Result<(), String> {

    //canvas.clear();
        
    for (pos, sprite) in (&data.0, &data.1).join() {
        //println!("Blitz at {:?}", Position { x: pos.x, y: pos.y });
        canvas.copy(
            texture, None,
            //Rect::new(sprite.initial_position.x as i32 + (sprite.width * sprite.current_frame) as i32,
            //          sprite.initial_position.y as i32 + (sprite.height * sprite.current_animation) as i32,
            //          sprite.width as u32,
            //          sprite.height as u32),
            Rect::new(pos.x as i32, pos.y as i32, sprite.width as u32, sprite.height as u32));
    }

    //println!("Present");
    canvas.present();

    Ok(())
}
