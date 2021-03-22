use crate::drivers::FileDriver;

use rand::random;

pub struct CPU {
    // General purpose registers
    gp_registers: [u8; 16],
    // Special registers
    I: u16,
    DT: u8,
    ST: u8,
    PC: u16,
    SP: usize,
    stack: [u16; 16],
    memory: FileDriver

}
impl CPU {
    pub fn new(file_name: &str) -> CPU {
        CPU {
            gp_registers: [0; 16],
            I: 0,
            DT: 0,
            ST: 0,
            PC: 0x200,
            SP: 0,
            stack: [0; 16],
            memory: FileDriver::new(file_name)
        }
    }

    pub fn execute_next_opcode(&mut self) {
        let opcode = self.memory.get_opcode(self.PC);
        println!("{:?}", opcode);
        let draw_flag = false;

        // Parts of the opcode that are used by various instructions
        let addr = opcode & 0x0FFF;
        let nibble = opcode & 0x000F;
        let x_val = ((opcode & 0x0F00) >> 8) as usize;
        let y_val = ((opcode & 0x00F0) >> 4) as usize;
        let byte = (opcode & 0x00FF) as u8;

        match opcode {
            0x00E0 => {
                // TODO Display stuff
                // CLS: Clear screen
                print!("clear");
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
                        // TODO Display stuff
                        unimplemented!()
                    },
                    0xE000 => {
                        match opcode & 0x00FF {
                            0x009E => {
                                // SKP Vx: Skip next instruction if key with value regX is pressed
                                // TODO I/O Stuff
                                unimplemented!()
                            },
                            0x00A1 => {
                                // SKNP Vx: Skip next instruction if key with value regX is not pressed
                                // TODO I/O Stuff
                                unimplemented!()
                            },
                            _ => panic!("Unknown opcode: {}", opcode)
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
                                // TODO I/O stuff
                                unimplemented!()
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
                                // TODO Add fonts
                                unimplemented!()
                            },
                            _ => panic!("Unknown opcode: {}", opcode)
                        }
                    },
                    _ => {println!("Unknown opcode: {}", opcode); self.PC += 2; return}
                }
            }
        }
    }

}