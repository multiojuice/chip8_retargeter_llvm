mod drivers;
mod processor;

use std::time::{Instant, Duration};
use std::thread::sleep;

extern crate sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

use processor::CPU;
use drivers::{InputDriver, VideoDriver};

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
    let context = sdl2::init()?;
    let mut video_driver = VideoDriver::new(&context);
    let mut input_driver = InputDriver::new(&context);
    let mut cpu: CPU = CPU::new("/Users/multiojuice/School/group06/soln/assets/chp8_IBM_logo.ch8");

    loop {
        let duration = Instant::now();
        let execution_rate = Duration::from_millis(16);

        let input = input_driver.get_input();
        match input {
            Ok(mem) => cpu.mmio.input_memory = mem,
            Err(_) => return Ok(())
        }

        cpu.execute_next_opcode();
        video_driver.draw(&cpu.mmio.video_memory);

        if duration.elapsed() < execution_rate {
            sleep(execution_rate - duration.elapsed())
        }
    }
}
