mod drivers;
mod processor;

use std::time::{Instant, Duration};
use std::thread::sleep;
use std::env;

use processor::CPU;


fn main() {
    let args: Vec<String> = env::args().collect();
    let mut cpu: CPU = CPU::new("/Users/multiojuice/School/group06/soln/assets/chp8_IBM_logo.ch8");
    let mut counter = 1;
    loop {
        let duration = Instant::now();
        let execution_rate = Duration::from_millis(16);
        let thing = cpu.execute_next_opcode();

        if duration.elapsed() < execution_rate {
            sleep(execution_rate - duration.elapsed())
        }

        counter += 1;
    }
}
