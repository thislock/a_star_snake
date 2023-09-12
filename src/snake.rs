use crate::{draw::{Color, PixelBuffer}, SNAKE_GRID_SIZE_I32, pathfinding::Path, SNAKE_GRID_SIZE};
use std::collections::HashMap;
use rand::prelude::*;

// just to make the code pretty
pub const SNAKE_DIR_RIGHT:[i32;2] = [ 1, 0];
pub const SNAKE_DIR_LEFT: [i32;2] = [-1, 0];
pub const SNAKE_DIR_UP:   [i32;2] = [ 0,-1];
pub const SNAKE_DIR_DOWN: [i32;2] = [ 0, 1];

type SnakePos = [i32;2];
type SnakeVel = [i32;2];
type LastSnakePos = [i32;2];

type SnakeBodyPart = (SnakePos, LastSnakePos, SnakeVel);
type BodyPartID = u32;

#[derive(Clone, Debug)]
pub struct Snake {
  pub snake_head: SnakeBodyPart,
  pub snake_body: HashMap<BodyPartID, SnakeBodyPart>,
  pub apple_pos: [i32;2],
}

impl Snake {
  // truely remarkable, i wrote it IN the class this time instead of fn SnAke_NeW() or some nonsence outside the class
  pub fn new(snake_pos: [i32;2]) -> Self {
    let mut rng = rand::thread_rng();
    Snake {
      snake_head: (snake_pos, [0, 0], [0,0]),
      snake_body: HashMap::new(),
      apple_pos: [rng.gen_range(0..SNAKE_GRID_SIZE[0] as i32), rng.gen_range(0..SNAKE_GRID_SIZE[0] as i32)],
    }
  }

  // takes in something and returns it as a snake grid position
  fn to_grid_pos(input: i32) -> i32 {
    input*5 + 25
  }

  pub fn draw_snake(&self, pixel_buffer: &mut PixelBuffer) {
    // draw the apple
    pixel_buffer.set_rect(Self::to_grid_pos(self.apple_pos[0]), Self::to_grid_pos(self.apple_pos[1]), 5, 5, [255, 0, 0]);
    // draw the head
    pixel_buffer.set_rect(Self::to_grid_pos(self.snake_head.0[0]), Self::to_grid_pos(self.snake_head.0[1]), 5, 5, [255, 255, 0]);
    // draw the leading body
    self.snake_body.iter().for_each(|f| {
      pixel_buffer.set_rect(((f.1.0[0]*5)+25) as i32, ((f.1.0[1]*5)+25) as i32, 5, 5, [255, 255, 0]);
    });
  }

  pub fn change_head_direction(&mut self, direction: &[i32;2]) {

    self.snake_head.2 = *direction;
    
  }

  fn grow(&mut self) {
    let largest_key = self.snake_body.iter().max();
    let mut new_pos = self.snake_head.0;
    let mut new_key = 0;
    if largest_key != None {
      new_key = largest_key.unwrap().0 + 1;
      new_pos = self.snake_body.get(&largest_key.unwrap().0).unwrap().0;
    }

    self.snake_body.insert(new_key, (new_pos, [0,0], [0,0]));

  }

  fn check_death(&mut self) -> bool {
    let mut result = false;
    let body_len = self.snake_body.len();
    self.snake_body.iter().for_each(|i| {
      if i.1.0 == self.snake_head.0 && body_len > 1 {
        result = true;
      }
    });

    // if out of bounds
    if (self.snake_head.0[0] > SNAKE_GRID_SIZE_I32[0]-1 || self.snake_head.0[1] > SNAKE_GRID_SIZE_I32[1]-1)
    || (self.snake_head.0[0] < 0 || self.snake_head.0[1] < 0) {
      result = true
    }

    result

  }

  fn eat_apple(&mut self, path: Option<&mut Path>) -> bool {
    if self.snake_head.0 == self.apple_pos {
      
      self.grow();
      let mut rng = rand::thread_rng();
      let mut new_apple_pos = [rng.gen_range(0..SNAKE_GRID_SIZE[0]) as i32, rng.gen_range(0..SNAKE_GRID_SIZE[1] as i32)];
      while new_apple_pos == self.snake_head.0 {
        new_apple_pos = [rng.gen_range(0..SNAKE_GRID_SIZE[0] as i32), rng.gen_range(0..SNAKE_GRID_SIZE[1]) as i32];
      }
      self.apple_pos = new_apple_pos;
      
      if path.is_some() {
        let unwraped_path = path.unwrap();
        let path_result = unwraped_path.generate_path(self);
        // for if, at that time a new path needs to be generated
        if path_result == Path::PATH_TYPE_FAILED {
          unwraped_path.emergency_regenerage_path = true;
        }
        self.follow_path(unwraped_path);
      } else {
        // some broken code meant to regen a path if it stinks, but instead just slows everything down
        if false {
          let unwraped_path = path.unwrap();
          unwraped_path.generate_path(self);
          self.follow_path(unwraped_path);
        }
      }

      return true;
    }

    if path.is_some() {
      let unwraped_path = path.unwrap();
      unwraped_path.generate_path(self);
      self.follow_path(unwraped_path);
    }

    false

  }

  // bet you could never guess what this does!
  pub fn move_snake_head(&mut self, reset: &mut bool, path: Option<&mut Path>) {

    // check if you can eat an apple, and does some growing if you can.
    self.eat_apple(path);
    
    // set the old snake position as the current, before moving the snake
    self.snake_head.1 = self.snake_head.0;
    // move da snake head
    self.snake_head.0[0] += self.snake_head.2[0];
    self.snake_head.0[1] += self.snake_head.2[1];
    
    // check if you rammed into yourself
    if self.check_death() {
      *reset = true;
    }

    // change the position of the body parts
    if !self.snake_body.is_empty() {
      self.cycle_body();
    }
    
  }

  fn cycle_body(&mut self) {
    
    let mut changes = vec![];

    self.snake_body.iter_mut().for_each(|i| {
      i.1.1 = i.1.0;
      if *i.0 == 0 {
        i.1.0 = self.snake_head.1;
      } else {
        changes.push(*i.0)
      }
    });
    changes.iter().for_each(|i| {
      self.snake_body.get_mut(&i).unwrap().0 = self.snake_body.get(&(i-1)).unwrap().1
    })
    
  }

}