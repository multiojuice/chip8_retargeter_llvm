use std::fs::File;
use std::io::prelude::*;

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
    let mut index = 0x200;
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
}