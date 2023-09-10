use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use crate::snake::{Snake, SNAKE_DIR_RIGHT, SNAKE_DIR_UP, SNAKE_DIR_LEFT, SNAKE_DIR_DOWN};

pub fn event_query(event_pump: &mut EventPump, running: &mut bool, snake: &mut Snake, debug: bool) {
  
  for event in event_pump.poll_iter() {
  
    match event {

      Event::KeyDown { keycode: Some(keycode), .. } => {
        if debug {
          match keycode {
            Keycode::Right => snake.change_head_direction(&SNAKE_DIR_RIGHT),
            Keycode::Left  => snake.change_head_direction(&SNAKE_DIR_LEFT),
            Keycode::Up    => snake.change_head_direction(&SNAKE_DIR_UP),
            Keycode::Down  => snake.change_head_direction(&SNAKE_DIR_DOWN),
            _ => {}
          }
        }
      }

      Event::Quit {..} |
      Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
        *running = false;
      },

      _ => {}
  
    }
  
  }

}