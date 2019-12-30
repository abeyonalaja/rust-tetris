extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use std::thread::sleep;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::rect::Rect;
use sdl2::video::{Window, WindowContext};

#[derive(Clone, Copy)]
enum TextureColor {
    Green,
    Blue,
}

pub fn main() {
    let sdl_context = sdl2::init().expect("SDL initialization failed");
    let video_subsystem = sdl_context.video().expect("Couldn't gat SDL video subsystem");

    let window = video_subsystem.window("sust-sdl2 demo: Video", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .expect("Failed to create window");

//    let mut canvas = window.into_canvas().build().expect("Failed to convert window to canvas");

    let mut canvas = window.into_canvas()
        .target_texture()
        .present_vsync()
        .build()
        .expect("Couldnt get windows canvas\
        ");




    let texture_creator: TextureCreator<_> = canvas.texture_creator();
    const TEXTURE_SIZE: u32 = 32;
    let mut square_texture: Texture =
    texture_creator.create_texture_target(None, TEXTURE_SIZE, TEXTURE_SIZE)
        .expect("Failed to create a texture");

    canvas.with_texture_canvas(&mut square_texture, |texture|{
        texture.set_draw_color(Color::RGB(0, 255, 0));
        texture.clear();
    });



    canvas.set_draw_color(Color::RGB(255,0,0));
    canvas.clear();
    canvas.copy(&square_texture, None, Rect::new(0,0, TEXTURE_SIZE, TEXTURE_SIZE))
        .expect("Couldnt copy texture to window");
    canvas.present();

    let mut event_pump = sdl_context.event_pump().expect("Failed to get SDL event pump");

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } =>
                    {
                        break 'running
                    },
                _ => {}
            }
        }
        sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn create_texture_rect<'a>(Canvas: &mut Canvas<Window>, texture_creator: &'a TextureCreator)