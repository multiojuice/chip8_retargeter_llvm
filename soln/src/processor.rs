use crate::drivers::FileDriver;
use rand::random;

/******************
 * CONFIG
 ******************/
 const SCALAR: u32 = 16;
 const VIDEO_WIDTH: usize = 64;
 const VIDEO_HEIGHT: usize = 32;
 const SDL_WIDTH: u32 = (VIDEO_WIDTH as u32) * SCALAR;
 const SDL_HEIGHT: u32 = (VIDEO_HEIGHT as u32) * SCALAR;

pub struct MMIO {
    pub video_memory: [[u8; VIDEO_WIDTH]; VIDEO_HEIGHT],
    pub input_memory: [bool; 16]
}

pub struct CPU {
    // Memmory mapped Input Output
    pub mmio: MMIO,
    // General purpose registers
    gp_registers: [u8; 16],
    // Special registers
    I: u16,
    DT: u8,
    ST: u8,
    PC: u16,
    SP: usize,
    d_flag: bool,
    stack: [u16; 16],
    memory: FileDriver
}
impl CPU {
    pub fn new(file_name: &str) -> CPU {
        CPU {
            mmio: MMIO {
                video_memory: [[0; VIDEO_WIDTH]; VIDEO_HEIGHT],
                input_memory: [false; 16]
            },
            gp_registers: [0; 16],
            I: 0,
            DT: 0,
            ST: 0,
            PC: 0x200,
            SP: 0,
            d_flag: false,
            stack: [0; 16],
            memory: FileDriver::new(file_name)
        }
    }

    pub fn execute_next_opcode(&mut self) {
        let opcode = self.memory.get_opcode(self.PC);
        println!("{:?}", opcode);
        self.d_flag = false;

        // Parts of the opcode that are used by various instructions
        let addr = opcode & 0x0FFF;
        let nibble = opcode & 0x000F;
        let x_val = ((opcode & 0x0F00) >> 8) as usize;
        let y_val = ((opcode & 0x00F0) >> 4) as usize;
        let byte = (opcode & 0x00FF) as u8;

        match opcode {
            0x00E0 => {
                // CLS: Clear screen
                for i in 0..VIDEO_HEIGHT {
                    for j in 0..VIDEO_WIDTH {
                        self.mmio.video_memory[i][j] = 0;        
                    }
                }
                self.PC += 2;
                return
            },
            0x00EE => {
                // RET: Return from subroutine
                self.PC = self.stack[self.SP];
                self.SP -= 1;
                return
            }
            _ => {
                match opcode & 0xF000 {
                    0x0000 => {
                        // SYS: This instruction is ignored in modern interpreters
                        self.PC += 2;
                        return
                    },
                    0x1000 => {
                        // JP addr: Jump to addr
                        self.PC = addr;
                        return
                    },
                    0x2000 => {
                        // CALL addr: Call subroutine
                        self.SP += 1;
                        self.stack[self.SP] = self.PC;
                        self.PC = addr;
                        return
                    },
                    0x3000 => {
                        //SE Vx, byte: Skip next instruction if register[x_val] == byte
                        if self.gp_registers[x_val] == byte{
                            self.PC += 2;
                        }
                        self.PC += 2;
                        return
                    },
                    0x4000 => {
                        //SNE Vx, byte: Skip next instruction if register[x_val] != byte
                        if self.gp_registers[x_val] != byte{
                            self.PC += 2;
                        }
                        self.PC += 2;
                        return
                    },
                    0x5000 => {
                        //SE Vx, Vy: Skip next instruction if register[x_val] == register[y_val]
                        if self.gp_registers[x_val] == self.gp_registers[y_val] {
                            self.PC += 2;
                        }
                        self.PC += 2;
                        return
                    },
                    0x6000 => {
                        // LD Vx, byte: Load byte into register[x_val]
                        self.gp_registers[x_val] = byte;
                        self.PC += 2;
                        return
                    },
                    0x7000 => {
                        // Add Vx, byte: Add byte to register[x_val]
                        self.gp_registers[x_val] += byte;
                        self.PC += 2;
                        return
                    },
                    0x8000 => {
                        match opcode & 0x000F {
                            0x0000 => {
                                // LD Vx, Vy: Store val of register[x_val] in register[y_val]
                                self.gp_registers[y_val] = self.gp_registers[x_val];
                                self.PC += 2;
                                return
                            },
                            0x0001 => {
                                // OR Vx, Vy: Perform bitwise OR on register[x_val] and register[y_val] and store in regX
                                self.gp_registers[x_val] = self.gp_registers[x_val] | self.gp_registers[y_val];
                                self.PC += 2;
                                return
                            },
                            0x0002 => {
                                // AND Vx, Vy: Perform bitwise AND on regX and regY and store in regX
                                self.gp_registers[x_val] = self.gp_registers[x_val] & self.gp_registers[y_val];
                                self.PC += 2;
                                return
                            },
                            0x0003 => {
                                // XOR Vx, Vy: XOR on regX and regY store in regX
                                self.gp_registers[x_val] = self.gp_registers[x_val] ^ self.gp_registers[y_val];
                                self.PC += 2;
                                return
                            },
                            0x0004 => {
                                // Add Vx, Vy: Set regX = regX + regY, set regF to 1 if the value is greater than 8 bits
                                let temp = self.gp_registers[x_val] as u16 + self.gp_registers[y_val] as u16;
                                self.gp_registers[0xF] = if temp > 255 { 1 } else { 0 };
                                self.gp_registers[x_val] = temp as u8;
                                self.PC += 2;
                                return
                            },
                            0x0005 => {
                                // SUB Vx, Vy: Set regX = regX - regY, set regF to 1 if there is no borrow (regX > regY)
                                self.gp_registers[0xF] = if self.gp_registers[x_val] > self.gp_registers[y_val] { 1 } else { 0 };
                                self.gp_registers[x_val] -= self.gp_registers[y_val];
                                self.PC += 2;
                                return
                            },
                            0x0006 => {
                                // SHR Vx: If least-significant digit of regX is 1, set VF to 1, else 0. Divide regX by 2
                                self.gp_registers[0xF] = self.gp_registers[x_val] & 0x01;
                                self.gp_registers[x_val] /= 2;
                                self.PC += 2;
                                return
                            },
                            0x0007 => {
                                // SUBN Vx, Vy: Set regX = regY - regX, set regF to 1 if there is no borrow (regY > regX)
                                self.gp_registers[0xF] = if self.gp_registers[y_val] > self.gp_registers[x_val] { 1 } else { 0 };
                                self.gp_registers[x_val] = self.gp_registers[y_val] - self.gp_registers[x_val];
                                self.PC += 2;
                                return
                            },
                            0x000E => {
                                // SHL Vx: If most-significant digit of regX is 1, set VF to 1, else 0. Multiply regX by 2
                                self.gp_registers[0xF] = self.gp_registers[x_val] & 0x80 >> 7;
                                self.gp_registers[x_val] *= 2;
                                self.PC += 2;
                                return
                            },
                            _ => {println!("Unknown opcode: {}", opcode); self.PC += 2; return}
                        }
                    },
                    0x9000 => {
                        // SNE Vx, Vy: Skip instruction is regX != regY
                        if self.gp_registers[x_val] != self.gp_registers[y_val] {
                            self.PC += 2;
                        }
                        self.PC += 2;
                        return
                    },
                    0xA000 => {
                        // LD I, addr: Set I = addr
                        self.I = addr;
                        self.PC += 2;
                        return
                    },
                    0xB000 => {
                        // JP V0, addr: Jump to location addr + V0
                        self.PC = self.gp_registers[0] as u16 + addr;
                        return
                    },
                    0xC000 => {
                        // RND Vx: Set regX = random byte AND byte
                        let rand_num: u8 = random();
                        self.gp_registers[x_val] = rand_num & byte;
                        self.PC += 2;
                        return
                    },
                    0xD000 => {
                        // DRW Vx, Vy, nibble: Display nibble-byte sprite stored at mem loc I at
                        // (regX, regY) on the screen. Set VF to 1 if there is a collision between pixels
                        self.gp_registers[0x0f] = 0;
                        for current in 0..(nibble as usize) {
                            // read up to nibble bytes
                            let y = (self.gp_registers[y_val] as usize + current) % VIDEO_HEIGHT;
                            for bit in 0..8 {
                                let x = (self.gp_registers[x_val] as usize + bit) % VIDEO_WIDTH;
                                // get bit and shift to place
                                let colored = self.memory.read_byte(self.I + (byte as u16)) >> (7-bit) & 1;
                                // set Vf
                                self.gp_registers[0x0f] |= colored & self.mmio.video_memory[y][x];
                                // set actual color
                                self.mmio.video_memory[y][x] ^= colored;
                            }
                        }
                        
                        self.PC += 2;
                        return
                    },
                    0xE000 => {
                        match opcode & 0x00FF {
                            0x009E => {
                                // SKP Vx: Skip next instruction if key with value regX is pressed
                                let key = self.gp_registers[x_val] as usize;
                                if self.mmio.input_memory[key] {
                                    self.PC += 4
                                } else {
                                    self.PC += 2;
                                }
                            },
                            0x00A1 => {
                                // SKNP Vx: Skip next instruction if key with value regX is not pressed
                                let key = self.gp_registers[x_val] as usize;
                                if !self.mmio.input_memory[key] {
                                    self.PC += 4
                                } else {
                                    self.PC += 2;
                                }
                                return
                            },
                            _ => {println!("Unknown opcode: {}", opcode); self.PC += 2; return}
                        }
                    },
                    0xF000 => {
                        match opcode & 0x00FF {
                            0x0007 => {
                                // LD VX, DT: Set regX = delay timer value
                                self.gp_registers[x_val] = self.DT;
                                self.PC += 2;
                                return
                            },
                            0x000A => {
                                // LD Vx, K: Wait for a key press then store that key val in regX
                                for (i, v) in self.mmio.input_memory.iter().enumerate() {
                                    if *v {
                                        self.PC += 2;
                                        self.gp_registers[x_val] = i as u8;
                                        return
                                    }
                                }
                                return 
                            },
                            0x0015 => {
                                // LD DT, Vx: Set delay time = regX
                                self.DT = self.gp_registers[x_val];
                                self.PC += 2;
                                return
                            },
                            0x0018 => {
                                // LD ST, VX: Set sound timer = regX
                                self.ST = self.gp_registers[x_val];
                                self.PC += 2;
                                return
                            },
                            0x001E => {
                                // ADD I, VX: Set I = I + regX
                                self.I += self.gp_registers[x_val] as u16;
                                self.PC += 2;
                                return
                            },
                            0x0029 => {
                                // LD F, Vx: Set I = location in memory for the hex font sprite for digit regX
                                let font_digit: u8 = self.gp_registers[x_val];
                                // All font sprites start at location (their decimal value times 5)
                                self.I = (font_digit * 5) as u16;
                                self.PC += 2;
                                return
                            },
                            0x0033 => {
                                // LD B, Vx: Store BCD representation of regX in mem locations I, I+1, I+2
                                let mut num: u8 = self.gp_registers[x_val];
                                self.memory.write_byte(self.I, num / 100);
                                num %= 100;
                                self.memory.write_byte(self.I + 1, num / 10);
                                num %= 10;
                                self.memory.write_byte(self.I + 2, num);
                                self.PC += 2;
                                return
                            },
                            0x0055 => {
                                // LD [I], Vx: Store registers reg0 through regX in mem starting at I
                                for i in 0..x_val {
                                    self.memory.write_byte(i as u16, self.gp_registers[i]);
                                }
                                self.PC += 2;
                                return
                            },
                            0x0065 => {
                                // LD Vx, [I]: Read registers reg0 through regX from mem starting at I
                                for i in 0..x_val {
                                    self.gp_registers[i] = self.memory.read_byte(i as u16);
                                }
                                self.PC += 2;
                                return
                            },
                            _ => {println!("Unknown opcode: {}", opcode); self.PC += 2; return}
                        }
                    },
                    _ => {println!("Unknown opcode: {}", opcode); self.PC += 2; return}
                }
            }
        }
    }

    pub fn update_timers(&mut self) {
        if self.DT > 0 {
            self.DT -= 1;
        }
        if self.SP > 0 {
            println!("Beep");
            self.ST -= 1;
        }
    }

    pub fn get_draw_flag(&self) -> bool {
        self.d_flag
    }
}