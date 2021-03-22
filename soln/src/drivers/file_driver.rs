use std::fs::File;
use std::io::prelude::*;

const FONT_SET: [u8; 80] = [
  0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
  0x20, 0x60, 0x20, 0x20, 0x70, // 1
  0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
  0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
  0x90, 0x90, 0xF0, 0x10, 0x10, // 4
  0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
  0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
  0xF0, 0x10, 0x20, 0x40, 0x40, // 7
  0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
  0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
  0xF0, 0x90, 0xF0, 0x90, 0x90, // A
  0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
  0xF0, 0x80, 0x80, 0x80, 0xF0, // C
  0xE0, 0x90, 0x90, 0x90, 0xE0, // D
  0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
  0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

pub struct FileDriver {
  pub rom: [u8; 4096],
  pub size: usize,
}

impl FileDriver {
  pub fn new(filename: &str) -> FileDriver {
    let mut f = File::open(filename).expect("Error: Cannot open");
    let mut file_buffer = [0u8; 4096 - 512];

    let bytes_read = if let Ok(bytes_read) = f.read(&mut file_buffer) {
      bytes_read
    } else {
      0
    };

    let mut rom = [0u8; 4096];
    let mut index = 0x0;
    while index < 80 {
      rom[index] = FONT_SET[index];
      index += 1;
    }
    index = 0x200;
    while index < 4096 {
      rom[index] = file_buffer[index - 0x200];
      index += 1;
    }
    
    FileDriver {
      rom,
      size: bytes_read
    }
  }

  pub fn get_opcode(&self, location: u16) -> u16 {
    if location > 4090 {
      return 0;
    }
    let loc: usize = location as usize;
    (self.rom[loc] as u16) << 8 | (self.rom[loc + 1]) as u16
  }

  pub fn write_byte(&mut self, location: u16, byte: u8) {
    if location > 4095 {
      panic!("Invalid memory location: {}", location);
    }
    let loc: usize = location as usize;
    self.rom[loc] = byte;
  }

  pub fn read_byte(&self, location: u16) -> u8 {
    if location > 4095 {
      panic!("Invalid memory location: {}", location);
    }
    let loc: usize = location as usize;
    self.rom[loc]
  }
}