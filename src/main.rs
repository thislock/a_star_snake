extern crate sdl2; 

use sdl2::pixels::{Color, PixelFormatEnum};

mod events;
use events::*;

mod draw;
use draw::*;

mod snake;
use snake::*;

mod pathfinding;
use pathfinding::*;

mod frame_culling;
use frame_culling::*;

const WIN_TITLE: &str = "A* algrorythim snake, but written by a dumbass teenager";
pub const WIN_DIM: [u32;2] = [800, 800];

pub const SNAKE_GRID_SIZE: [u32;2] = [30;2];
pub const SNAKE_GRID_SIZE_I32: [i32;2] = [SNAKE_GRID_SIZE[0] as i32, SNAKE_GRID_SIZE[1] as i32];

// im sorry in advance future me/random guy who has to read this 
pub fn main() {

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
 
    let window = video_subsystem.window(WIN_TITLE, WIN_DIM[0], WIN_DIM[1])
        .position_centered()
        .resizable()
        .build()
        .unwrap();
 
    let mut canvas = window.into_canvas().build().unwrap();
 
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let texture_creator = canvas.texture_creator();

    let mut rend_target = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGB24, PALLET_DIM[0] as u32, PALLET_DIM[1] as u32).unwrap();

    let mut pixel_buffer = PixelBuffer::new();
    
    let mut snake = Snake::new([255, 255, 0], [15, 15]);
    
    let mut frame_culling = FrameCulling::new(10);

    let mut test_path = Path::new(vec![]);

    test_path.generate_path(&mut snake);
        
    let mut reset = false;
    let mut running = true;
    while running {
        frame_culling.record_frame_start();
        
        snake.follow_path(&mut test_path);
        snake.move_snake_head(&mut reset, Some(&mut test_path));
        event_query(&mut event_pump, &mut running, &mut snake, false);

        // draw the backround of the snake board thing
        pixel_buffer.set_rect(25, 25, (SNAKE_GRID_SIZE[0]*5) as i32, (SNAKE_GRID_SIZE[1]*5) as i32, [0,0,0]);
        
        snake.draw_snake(&mut pixel_buffer);

        rend_target.update(None, &pixel_buffer.pixels, PALLET_DIM[0]*3).unwrap();
        canvas.copy(&rend_target, None, None).unwrap();

        canvas.present();

        if reset {
            running = false;
        }


        frame_culling.process_frame_end();
    }
}