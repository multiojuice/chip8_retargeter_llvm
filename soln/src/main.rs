mod drivers;
mod processor;

use std::time::{Instant, Duration};
use std::thread::sleep;
use std::env;

extern crate sdl2;
use processor::CPU;
use drivers::{InputDriver, VideoDriver};

/******************
 * CONFIG
 ******************/
 pub const SCALAR: u32 = 16;
 pub const VIDEO_WIDTH: usize = 64;
 pub const VIDEO_HEIGHT: usize = 32;
 pub const SDL_WIDTH: u32 = (VIDEO_WIDTH as u32) * SCALAR;
 pub const SDL_HEIGHT: u32 = (VIDEO_HEIGHT as u32) * SCALAR;

/******************
 * FUNCTIONS
 ******************/
pub fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let context = sdl2::init()?;
    let mut video_driver = VideoDriver::new(&context);
    let mut input_driver = InputDriver::new(&context);
    let mut cpu: CPU = CPU::new(&args[1]);

    loop {
        let duration = Instant::now();
        let execution_rate = Duration::from_millis(16);

        let input = input_driver.get_input();
        match input {
            Ok(mem) => cpu.mmio.input_memory = mem,
            Err(_) => return Ok(())
        }

        cpu.execute_next_opcode();
        
        if cpu.get_draw_flag() {
            video_driver.draw(&cpu.mmio.video_memory);
        }

        cpu.update_timers();

        if duration.elapsed() < execution_rate {
            sleep(execution_rate - duration.elapsed())
        }
    }
}
