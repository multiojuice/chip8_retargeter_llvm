use std::fs::File;
use std::io::prelude::*;

pub struct FileDriver {
  pub rom: [u8; 4096],
  pub size: usize,
}

impl FileDriver {
  pub fn new(filename: &str) -> FileDriver {
    let mut f = File::open(filename).expect("Error: Cannot open");
    let mut buffer = [0u8; 4096];

    let bytes_read = if let Ok(bytes_read) = f.read(&mut buffer) {
      bytes_read
    } else {
      0
    };

    FileDriver {
      rom: buffer,
      size: bytes_read
    }
  }

  pub fn get_opcode(&self, location: u16) -> u16 {
    let loc: usize = location as usize;
    (self.rom[loc] as u16) << 8 | (self.rom[loc + 1]) as u16
  }
}