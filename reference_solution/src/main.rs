mod drivers;

use std::env;
use drivers::FileDriver;

fn get_opcode(file_driver: FileDriver) -> u16 {
    (file_driver.rom[0] as u16) << 8 | (file_driver.rom[1]) as u16
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_driver = FileDriver::new("/Users/multiojuice/Projects/chip8_retargeter_llvm/src/assets/chp8_IBM_logo.ch8");

    let opcode = get_opcode(file_driver);
    match opcode {
        0x00e0 => println!("HERE"),
        _ => println!("{:?}", opcode)
    }
}
