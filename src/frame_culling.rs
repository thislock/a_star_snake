
// just a reusable fps culling/counter thing that can be used in any future project

use std::{time, thread};

pub struct FrameCulling {
  target_fps: u32,
  wait_time: f32,
  frame_start: time::Instant,
  // for fps rate
  fps_count: u32,
  sec_counter: time::Instant,
}

impl FrameCulling {

  // woah, look i did it again!
  pub fn new(fps: u32) -> FrameCulling {
    FrameCulling{
      target_fps: fps,
      wait_time: 1.0/(fps as f32),
      frame_start: time::Instant::now(),
      fps_count: 0,
      sec_counter: time::Instant::now(),
    }
  }

  // frame culling
  pub fn record_frame_start(&mut self) {
    self.frame_start = time::Instant::now();
  }

  pub fn process_frame_end(&mut self) {
    let frame_time = time::Instant::now().duration_since(self.frame_start).as_secs_f32();
    if frame_time < self.wait_time {
      thread::sleep(time::Duration::from_secs_f32(self.wait_time-frame_time));
    }
  }

  // fps counting
  pub fn print_fps(&mut self) {
    self.fps_count += 1;
    if time::Duration::as_secs(&time::Instant::duration_since(&time::Instant::now(), self.sec_counter)) >= 1 {
      println!("fps: {}", self.fps_count);
      self.sec_counter = time::Instant::now();
      self.fps_count = 0;
    }
  }

  pub fn reset_fps(&mut self, fps: u32) {
    self.target_fps = fps;
    self.wait_time = 1.0/(fps as f32);
  }



}