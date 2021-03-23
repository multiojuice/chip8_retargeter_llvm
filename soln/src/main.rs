mod drivers;
mod processor;

use std::time::{Instant, Duration};
use std::thread::sleep;
use processor::CPU;

extern crate sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

/******************
 * CONFIG
 ******************/
 const SCALAR: u32 = 16;
 const VIDEO_WIDTH: usize = 64;
 const VIDEO_HEIGHT: usize = 32;
 const SDL_WIDTH: u32 = (VIDEO_WIDTH as u32) * SCALAR;
 const SDL_HEIGHT: u32 = (VIDEO_HEIGHT as u32) * SCALAR;

/******************
 * FUNCTIONS
 ******************/
pub fn main() -> Result<(), String> {
    let mut cpu: CPU = CPU::new("/Users/multiojuice/School/group06/soln/assets/chp8_IBM_logo.ch8");

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Chip-8 emu", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::RGB(255, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump()?;

    loop {
        let duration = Instant::now();
        let execution_rate = Duration::from_millis(16);
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break,
                _ => {}
            }
        }

        canvas.clear();
        canvas.present();
        if duration.elapsed() < execution_rate {
            sleep(execution_rate - duration.elapsed())
        }

        let thing = cpu.execute_next_opcode();
    }
}
