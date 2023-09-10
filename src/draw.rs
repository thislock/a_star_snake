
pub const PALLET_DIM: [usize;2] = [200;2];
pub const PALLET_SIZE: usize = PALLET_DIM[0]*PALLET_DIM[1];

pub type Color = [u8;3];
pub struct PixelBuffer {
  pub pixels: [u8; PALLET_SIZE*3],
}

const BG_SHADE: u8 = 100;

impl PixelBuffer {
  pub fn new() -> PixelBuffer {
    PixelBuffer { pixels: [BG_SHADE; PALLET_SIZE*3] }
  }

  pub fn set_pixel(&mut self, x: i32, y: i32, color: Color) {
    let offset = (((PALLET_DIM[1] as i32 * y) + x) * 3) as usize + 3;

    // i know +0 does nothing, BUT LOOK AT HOW LINED UP IT IS!
    self.pixels[offset+0] = color[0];
    self.pixels[offset+1] = color[1];
    self.pixels[offset+2] = color[2];
  }

  pub fn set_rect(&mut self, x:i32, y:i32, w:i32, h:i32, color: Color) {
    for i in 0..w {
      for u in 0..h {
        self.set_pixel(i + x, u + y, color)
      }
    }
  }
}