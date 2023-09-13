use crate::{snake::{Snake, SNAKE_DIR_RIGHT, SNAKE_DIR_LEFT, SNAKE_DIR_UP, SNAKE_DIR_DOWN}, SNAKE_GRID_SIZE};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Path {
  pub steps: Vec<[i32;2]>,
  steps_taken: u32,
  pub emergency_regenerage_path: bool,
}

impl Path {

  pub fn new(path: Vec<[i32;2]>) -> Self {
    Path {
      steps: path,
      steps_taken: 0,
      emergency_regenerage_path: false,
    }
  }

  pub const PATH_TYPE_FAILED: i32 = -1;
  pub const PATH_TYPE_SUCCESS: i32 = 1;

  // generates every possible path, that brings the snake closer to the apple, and picks the shortest one
  pub fn generate_path(&mut self, snake: &mut Snake) -> i32 {
    
    // maximum possible path size, if it getts bigger than this, we have an issue
    const PATH_LIMIT: u32 = SNAKE_GRID_SIZE[0]*SNAKE_GRID_SIZE[1];

    let mut found_apple = false;

    let mut paths: HashMap<u32, (Self, bool)> = HashMap::new();
    let mut on_id = 0;
    
    // create the first branching path(s)
    self.check_outcomes_from(snake).iter().for_each(|i| {
      let possible_path = Self::new(vec![*i]);
      let mut result= [[0, 0];4];
      possible_path.ensure_safety(snake, &mut result);
      if !possible_path.simulate_path(snake) && *i != [0,0] {
        paths.insert(on_id, (possible_path, false));
        on_id += 1;
      }
      if *i == [0, 0] {
        for u in result.iter() {
          let possible_path = Self::new(vec![*u]);
          if !possible_path.simulate_path(snake) {
            paths.insert(on_id, (possible_path, false));
            on_id += 1;
          }
        }
      }
    });
    
    'find_path: while !found_apple {
      if paths.len() as u32 == PATH_LIMIT {
        println!("pathfinding overflow: attempted to create a path larger than all board pieces");
        break 'find_path;
      }
      
      let mut changes: Vec<Path> = vec![];
      
      let mut deletions = vec![];

      // this shit is the thing being run like, 99999 or something times per cycle, so keep that in mind i guess
      paths.iter_mut().for_each(|i| {
        
        // make sure this path is still valid, if not mark if for deletion.
        if i.1.0.simulate_path(snake) || i.1.0.steps.contains(&[0,0]) {
          deletions.push(*i.0);
        } else {
          self.generate_path_branch(&mut changes, snake, &mut found_apple, i);
        }
        
      });
      
      changes.iter().for_each(|y| {
        paths.insert(on_id, (y.clone(), false));
        on_id += 1;
      });
      
      deletions.iter().for_each(|i| {
        paths.remove(i);
      });

    }
    
    let path_type;
    
    let successful_path = paths.iter().find(|i| i.1.1);
    if successful_path.is_some() {
      let success = {let mut path = successful_path.unwrap().1.0.clone(); path.reverse(); path};
      println!("found path, while checking {} paths", on_id);
      println!("{:?}", success);
      *self = success;
      path_type = Self::PATH_TYPE_SUCCESS;
    } else {
      println!("could not find viable path for snake");
      path_type = Self::PATH_TYPE_FAILED;
    }

    path_type
    
  }
  
  fn generate_path_branch(&self, changes: &mut Vec<Path>, snake: &Snake, found_apple: &mut bool, i: (&u32, &mut (Path, bool))) {

    let mut reversed_self = i.1.0.clone(); reversed_self.reverse();
    let reversed_self = reversed_self;

    let movement_options;
    if let Some(snake_path) = snake.add_path(&reversed_self) {
      movement_options = i.1.0.check_outcomes_from(&snake_path);
    } else {
      let mut result = [[0;2];4];
      self.ensure_safety(snake, &mut result);
      movement_options = result;
    }
          
    for u in 0..movement_options.len() {
      if movement_options[u] != [0,0] {
        if u == 0 {
          i.1.0.steps.push(movement_options[u]);
        } else {
          let mut new_branch = i.1.0.steps.clone();
          new_branch.push(movement_options[u]);
          changes.push(i.1.0.clone())
        }
      }
    }
          
    if reversed_self.path_finished(snake) {
      i.1.1 = true;
      *found_apple = true;
    }

  }
  
  // returns the movements possible from where the snake is now
  fn check_outcomes_from(&self, snake: &Snake) -> [[i32;2];4] {

    let mut result = [[0;2];4];

    let possible_snake_heads = [
      (self.closer(snake, snake.snake_head.0, add_arrays(snake.snake_head.0, SNAKE_DIR_LEFT)),  SNAKE_DIR_LEFT),
      (self.closer(snake, snake.snake_head.0, add_arrays(snake.snake_head.0, SNAKE_DIR_RIGHT)), SNAKE_DIR_RIGHT),
      (self.closer(snake, snake.snake_head.0, add_arrays(snake.snake_head.0, SNAKE_DIR_UP)),    SNAKE_DIR_UP),
      (self.closer(snake, snake.snake_head.0, add_arrays(snake.snake_head.0, SNAKE_DIR_DOWN)),  SNAKE_DIR_DOWN),
    ];

    let mut l = 0;
    possible_snake_heads.iter().for_each(|i| {
      if i.0 {
        result[l] = i.1;
        l += 1;
      }
    });

    // a broken feature
    //self.ensure_safety(snake, &mut result);  
    
    result
    
  }

  // checks if a given path has reached the apple
  fn path_finished(&self, snake: &Snake) -> bool {

    let mut result = false;

    let possible_snake = snake.add_path(self);
    if possible_snake.is_some() {
      let possible_snake = possible_snake.unwrap();
      if possible_snake.snake_head.0 == snake.apple_pos {
        result = true
      }
    }

    result

  }

  fn reverse(&mut self) {
    let mut steps_bucket = vec![];
    self.steps.iter().rev().for_each(|i| {
      steps_bucket.push(*i);
    });
    self.steps = steps_bucket;
  }

  // if the b is closer to the apple in snake than a, then return true
  fn closer(&self, snake: &Snake, a: [i32;2], b: [i32;2]) -> bool {

    let mut result = false;

    // using the good ol pythagerian therum, get the distance from each point to the apple
    let a_appledst: f32 = pythagerian_therum(a, snake.apple_pos);
    let b_appledst: f32 = pythagerian_therum(b, snake.apple_pos);
    
    if b_appledst < a_appledst {
      result = true
    }

    result

  }

  // make sure none of the options would result in death, and if so, just change it to something random to survive
  fn ensure_safety(&self, snake: &Snake, result: &mut [[i32;2];4]) {
    let fake_path = {let mut fp = self.clone(); fp.reverse(); fp};
    result.iter_mut().for_each(|i| {
      let fakepath_addition = {let mut e = fake_path.clone(); e.steps.push(*i); e};
      
      // make sure that the path taken would result in survival, but if not just die i guess
      if fakepath_addition.simulate_path(snake) {
        let directions = [
          SNAKE_DIR_LEFT,
          SNAKE_DIR_RIGHT,
          SNAKE_DIR_UP,
          SNAKE_DIR_DOWN
        ];

        let mut any_done = false;
        directions.iter().for_each(|u| {
          let fake_path_check = {let mut e = fake_path.clone(); e.steps.push(*i); e};
          if !fake_path_check.simulate_path(snake) {
            any_done = true;
            *i = *u;
          }
        });

        if !any_done {
          *i = [0,0];
        }

      }

    });
  }

  // simulate the entire inputed path as if it were in the real game, return false if there were no issues
  fn simulate_path(&self, snake: &Snake) -> bool {

    let mut result = false;

    let mut simulated_snake = snake.clone();
    
    self.steps.iter().for_each(|i| {
      simulated_snake.change_head_direction(i);
      simulated_snake.move_snake_head(&mut result, None);
    });

    result

  }

}

impl Snake {

  pub fn follow_path(&mut self, path: &mut Path) {
    if ((path.steps.len()) as u32) > path.steps_taken {
      self.snake_head.2 = path.steps[path.steps_taken as usize];
    }
    path.steps_taken += 1;
  }

  pub fn add_path(&self, path: &Path) -> Option<Self> {
    
    let mut result = self.clone();

    let mut error = false;

    path.steps.iter().for_each(|i| {
      result.change_head_direction(i);
      result.move_snake_head(&mut error, None);
    });

    if !error {
      return Some(result)
    } else {
      return None;
    }

  }

}

fn add_arrays(a: [i32;2], b: [i32;2]) -> [i32;2] {
  [a[0] + b[0], a[1] + b[1]]
} 

macro_rules! dst_from {
  ($a: expr, $b: expr) => {
    ($a - $b).abs()
  };
}

fn pythagerian_therum(a: [i32;2], b:[i32;2]) -> f32 {
  ((dst_from!(a[0], b[0]).pow(2) + dst_from!(a[1], b[1]).pow(2)) as f32).sqrt()
}