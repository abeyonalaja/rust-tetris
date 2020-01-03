extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::{Duration, SystemTime};
use std::thread::sleep;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::rect::Rect;
use sdl2::video::{Window, WindowContext};

const TEXTURE_SIZE: u32 = 32;

#[derive(Clone, Copy)]
enum TextureColor {
    Green,
    Blue,
}

type Piece = Vec<Vec<u8>>;
type States = Vec<Piece>;

struct Tetrimino {
    states: States,
    x: isize,
    y: usize,
    current_state: u8
}

trait TetriminoGenerator {
    fn new() -> Tetrimino;
}

struct TetriminoI;

impl TetriminoGenerator for TetriminoI {
    fn new() -> Tetrimino {
        Tetrimino{
            states: vec![vec![vec![1, 1, 1, 1],
                              vec![0, 0, 0, 0],
                              vec![0, 0, 0, 0],
                              vec![0, 0, 0, 0]],
                         vec![vec![0, 1, 0, 0],
                              vec![0, 1, 0, 0],
                              vec![0, 1, 0, 0],
                              vec![0, 1, 0, 0]]],
            x: 4,
            y: 0,
            current_state: 0,
        }
    }
};

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

    let green_square = create_texture_rect(&mut canvas, &texture_creator,
                                           TextureColor::Green,TEXTURE_SIZE)
        .expect("Failed to create a texture");

    let blue_square = create_texture_rect(&mut canvas, &texture_creator,
    TextureColor::Blue, TEXTURE_SIZE)
        .expect("Failed to create a texture");

    let timer = SystemTime::now();


//    let mut square_texture: Texture =
//    texture_creator.create_texture_target(None, TEXTURE_SIZE, TEXTURE_SIZE)
//        .expect("Failed to create a texture");
//
//    canvas.with_texture_canvas(&mut square_texture, |texture|{
//        texture.set_draw_color(Color::RGB(0, 255, 0));
//        texture.clear();
//    });
//
//
//
//    canvas.set_draw_color(Color::RGB(255,0,0));
//    canvas.clear();
//    canvas.copy(&square_texture, None, Rect::new(0,0, TEXTURE_SIZE, TEXTURE_SIZE))
//        .expect("Couldnt copy texture to window");
//    canvas.present();

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

        // fill the window with red.
        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas.clear();

        let display_green = match timer.elapsed(){
            Ok(elapsed) => elapsed.as_secs() % 2 == 0,
            Err(_)=>{
                true
            }
        };
        let square_texture = if display_green{
            &green_square
        }else{
            &blue_square
        };

        // Copy the texture into the window
        canvas.copy(square_texture, None, Rect::new(0,0, TEXTURE_SIZE, TEXTURE_SIZE))
            .expect("Couldint copy texture into window");
        canvas.present();
        sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn create_texture_rect<'a>(canvas: &mut Canvas<Window>, texture_creator: &'a TextureCreator<WindowContext>, color: TextureColor, size: u32) ->
Option<Texture<'a>>{

    if let Ok(mut square_texture) =
    texture_creator.create_texture_target(None, size,size){
        canvas.with_texture_canvas(&mut square_texture, |texture|{
            match color {
                TextureColor::Green =>
                    texture.set_draw_color(Color::RGB(0,255,0)),
                TextureColor::Blue =>
                    texture.set_draw_color(Color::RGB(0,0,255)),
            }
            texture.clear();
        }).expect("Failed tp color a texture");
        Some(square_texture)
    } else{
        None
    }
}