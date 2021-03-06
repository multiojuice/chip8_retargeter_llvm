/**
 * video_driver.rs
 * this file abstracts the graphics implementation out into just 
 * memory mapped io from the cpu. 
 */

use sdl2;
use sdl2::pixels;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use crate::SDL_HEIGHT;
use crate::SDL_WIDTH;
use crate::VIDEO_WIDTH;
use crate::VIDEO_HEIGHT;
use crate::SCALAR;

pub struct VideoDriver {
  canvas: Canvas<Window>,
}

impl VideoDriver {
  pub fn new(context: &sdl2::Sdl) -> Self {
    let vss = context.video().unwrap();
    let window = vss.window("Chip-8", SDL_WIDTH, SDL_HEIGHT)
      .position_centered()
      .opengl()
      .build()
      .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    // Set to empty canvas
    canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    VideoDriver { canvas: canvas }
  }

  /**
   * draw takes in a double array of pixels indicating if a pixel should be 
   *    colored or not colored.
   */
  pub fn draw(&mut self, pixels: &[[u8; VIDEO_WIDTH]; VIDEO_HEIGHT]) {
    for (y, row) in pixels.iter().enumerate() {
        for (x, &colored) in row.iter().enumerate() {
            // Scale up to correct pixel top left
            let x = (x as u32) * SCALAR;
            let y = (y as u32) * SCALAR;
            match colored {
              0 => self.canvas.set_draw_color(pixels::Color::RGB(0, 0, 0)),
              _ => self.canvas.set_draw_color(pixels::Color::RGB(115, 115, 115))
            }
            
            // Fill scaled up pixel
            self.canvas.fill_rect(Rect::new(x as i32, y as i32, SCALAR, SCALAR)).ok();
        }
    }
    self.canvas.present();
  }
}
