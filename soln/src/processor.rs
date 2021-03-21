use crate::drivers::FileDriver;

struct CPU {
    // General purpose registers
    gp_registers: Vec<u8>,
    // Special registers
    I: u16,
    DT: u8,
    ST: u8,
    PC: u16,
    SP: u8,

    stack: Vec<u16>,
    memory: FileDriver

}
impl CPU {
    fn new() -> CPU {
        CPU {
            gp_registers: vec![0: 16],
            I: 0,
            DT: 0,
            ST: 0,
            PC: 0x200,
            SP: 0,
            stack: vec![0: 16],
            memory
        }
    }

    fn read_file(&mut self, file_name: &str) {
        self.memory = FileDriver::new(file_name)
    }

    fn execute_next_opcode(&mut self) {
        let opcode = self.memory.get_opcode(self.PC);
        let draw_flag = false;

        // Parts of the opcode that are used by various instructions
        let addr = (opcode & 0x0FFF);
        let nibble = (opcode & 0x000F);
        let x_val = (opcode & 0x0F00) >> 8;
        let y_val = (opcode & 0x00F0) >> 4;
        let byte = (opcode & 0x00FF);

        match opcode {
            0x00E0 => {
                // TODO Display stuff
                // CLS: Clear screen
                unimplemented!()
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
                        self.PC += 1;
                        return
                    },
                    0x1000 => {
                        // JP: Jump to addr
                        self.PC = addr;
                        return
                    },
                    0x2000 => {
                        // CALL: Call subroutine
                        self.SP += 1;
                        self.stack[self.SP] = self.PC;
                        self.PC = addr;
                        return
                    }

                }
            }
        }
    }

}