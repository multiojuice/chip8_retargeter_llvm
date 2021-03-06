# CHIP-8 Interpreter

Brief overview (approx. one paragraph)...

Author(s):

- Zachary Johnson
- Owen Sullivan

## Assignment Details
In this assignment you will be implementing an interpreter to run programs written in the Chip-8 programming language. Your job will be to implement the memory and CPU functionality for the interpreter, the display and I/O functions are done for you. Below is all of the information you will need to implement memory and CPU functionality.

### Memory
The Chip-8 language is able to access up to 4KB of ram or 4,096 bytes. In hex these are addresses 0x000 (0) to 0xFFF (4,096). Most Chip-8 programs start at location 0x200 (512) as the first 512 bytes are reserved for the interpreter (0x000 - 0x1FF) and are not used by any programs. 

Chip-8 also supports hex fonts which are sprites that represent the hex digits 0 through F. Each font sprite is 5 bytes long. These have been provided for you in file_driver.rs. All you will need to do is put the fonts into memory, in the block that is reserved for the interpreter (0x000 - 0x1FF). Make sure you place them somewhere that will be easy to get later since one instruction does require you to get the memory location of individual fonts.

### Registers
  Chip-8 has 16 general use registers, each capable of storing 8-bit values. In this document general purpose registers will be referred to as Vx or Vy where x and y are a hexadecimal digit. Another register, referred to as the I register, is usually used to hold memory addresses and can hold 16-bit values. Register VF shouldn't be used by any program as it is used as a flag by some of the opcodes.

  There are also 2 special purpose 8-bit registers referred to as DT, for delay timer, and ST for sound timer. While these registers are not zero, their value are decreased by 1 at a rate of 60Hz. The sound timer, when not zero, is active and means the Chip-8 buzzer would sound until it reached zero. For this assignment you are not required to make the program actually make sound, a simple print statement whenever a sound would be played is sufficient. The delay timer is used by some instructions.

Other registers that your interpreters will need are the 16-bit program counter (PC) and the 8-bit stack pointer (SP). The program counter is used to keep track of where in the program the interpreter is at, and holds address of the current instruction. The stack pointer is used to point to the top level of the stack. The stack is an array of 16 16-bit values, which stores addresses that the program can return to when it finishes a subroutine. Because of this the Chip-8 is built to allow up to 16 levels of nested subroutines.

### Opcode Information
Chip-8 opcodes are 2 bytes long, stored with the most significant byte first. Since each memory location can only hold a single byte, to get a full opcode you need to read an even memory address and the address next to it. Because of this remember to increment the program counter by 2, after each instruction that requires it, to stay at an even address. Any sprite data will be properly padded so that all instructions after are still in the right place.

The following variables are used in the descriptions below:
- *nnn*: A 12-bit value in the lowest 12 bits of the instruction. Used mostly to give memory addresses.
- *n*: A 4-bit value in the lowest 4 bits of the instruction. Used by the draw instruction to indicate the size of a sprite in bytes.
- *x*: A 4-bit value in the lower 4 bits of the first half (high-byte) of the instruction. This is one of the values used to indicate a specific register.
- *y*: A 4-bit value in the upper 4 bits of the second half (low-byte) of the instruction. This is one of the values used to indicate a specific register.
- *kk*: An 8-bit value in the lowest 8 bits of the instruction. Used in instructions where some constant 8-bit value is needed.

| Opcode   | Name       | Description |
|----------|------------|-------------|
| 0x0*nnn* | SYS *nnn* | This instruction is ignored by modern interpreters |
| 0x00E0   | CLS      | Clear display |
| 0x00EE   | RET      | Return from a subroutine by setting the Program Counter to the address at the top of the stack. Remember to decrement the stack pointer |
| 0x1*nnn* | JP *nnn*  | Jump to address *nnn* |
| 0x2*nnn* | CALL *nnn* | Call subroutine at address *nnn*. Put the current Program Counter on the stack and increment the stack pointer by 1. Then jump to address *nnn* |
| 0x3*xkk* | SE Vx, *kk* | Skip the next instruction if register *x* equals *kk* |
| 0x4*xkk* | SNE Vx, *kk* | Skip the next instruction if register *x* equals *kk* |
| 0x5*xy*0 | SE Vx, Vy    | Skip the next instruction if register *x* equals register *y* |
| 0x6*xkk* | LD Vx, *kk* | Load the value *kk* into register *x* |
| 0x7*xkk* | ADD Vx, *kk* | Add *kk* to the value of register *x* and store it in register *x* |
| 0x8*xy*0 | LD Vx, Vy    | Load the value of register *y* into register *x* |
| 0x8*xy*1 | OR Vx, Vy    | Perform bitwise OR on the values in registers *x* and *y*, then store the result in register *x* |
| 0x8*xy*2 | AND Vx, Vy   | Perform bitwise AND on the values in registers *x* and *y*, then store the result in register *x* |
| 0x8*xy*3 | XOR Vx, Vy   | Perform bitwise XOR on the values in registers *x* and *y*, then store the result in register *x* |
| 0x8*xy*4 | ADD Vx, Vy   | Add register *y* to register *x*. If the result is greater than 8bits (>255) set register 0xF to 1 for the carry, otherwise 0. Only lowest 8 bits of the result is kept and stored in register *x* | 
| 0x8*xy*5 | SUB Vx, Vy   | Subtract register *y* from register *x*. If register *x* is greater than register *y* set register 0xF to 1 other wise 0. This represents no borrow. Store the result in register *x*. |
| 0x8*xy*6 | SHR Vx       |  Set register 0xF to the least significant bit of register *x*.Then bitwise shift right on register *x*.|
| 0x8*xy*7 | SUBN Vx, Vy  | Subtract register *x* from register *y*. If register *y* is greater than register *x* set register 0xF to 1 other wise 0. This represents no borrow. Store result in register *x*. |
| 0x8*xy*E | SHL Vx       | Set register 0xF to the most significant bit of register *x*. Then bitwise shift life on register *x*. |
| 0x9*xy*0 | SNE Vx, Vy   | Skip next instruction if register *x* does not equal register *y*. |
| 0xA*nnn* | LD I, *nnn*  | Set I register equal to *nnn*. |
| 0xB*nnn* | JP V0, *nnn* | Jump to address *nnn* plus register 0. |
| 0xC*xkk* | RND Vx, *kk* | Set register *x* to the value of *kk* ANDed with a random 8bit (0 to 255) number. |
| 0xD*xyn* | DRW Vx, Vy, *n* | Read in an *n*-byte sized sprite starting at the address stored in register I. Display the sprite at location (register *x*, register *y*). Sprites wrap around the dge of the screen and if any pixels are erased, due to the XOR operation of displaying the sprite, set register 0xF to 1. |
| 0xE*x*9E | SKP Vx | Skip next instruction if the key with the value of register *x* is pressed. |
| 0xE*x*A1 | SKNP Vx | Skip next instruction if the key with the value of register *x* is not pressed. |
| 0xF*x*07 | LD Vx, DT | Set register *x* to equal the delay timer. |
| 0xF*x*0A | LD Vx, K  | Wait for a key press and then store that key value in register *x*. This opcode should stop all execution until any key is pressed. |
| 0xF*x*15 | LD DT, Vx | Set the delay timer equal to register *x* |
| 0xF*x*18 | LD ST, Vx | Set the sound timer equal to register *x* |
| 0xF*x*1E | ADD I, Vx | Add the value of register *x* to the I register and store it back in the I register |
| 0xF*x*29 | LD F, Vx  | Set the I register equal to the location of the hex font sprite that has the value corresponding to register *x*'s value. Check below for more information on the fonts |
| 0xF*x*33 | LD B, Vx  | Store the binary-coded decimal representation of the value in register *x* in memory locations I, I+1, and I+2. The hundres place should be stored in memory location represented by the value in the I register, tens place in I+1, and ones place in I+2. |
| 0xF*x*55 | LD [I], Vx | Store registers 0 through *x* in memory starting at address stored in the I register |
| 0xF*x*65 | LD Vx, [I] | Read registers 0 through *x* from memory starting at address stored in the I register |
---
---

# [Assignment Design](https://www.cs.rit.edu/~mtf/teaching/20205/psr/assignments.html#assignment_design) Activity

Design a novel programming assignment suitable for the first 1/3 of the course
(Rust Basics).  The assignment should be a bit larger than
[Programming#01](https://www.cs.rit.edu/~mtf/teaching/20205/psr/assignments.html#prog01),
but need not be quite as large as
[Programming#02](https://www.cs.rit.edu/~mtf/teaching/20205/psr/assignments.html#prog02),
[Programming#03](https://www.cs.rit.edu/~mtf/teaching/20205/psr/assignments.html#prog03),
or
[Programming#04](https://www.cs.rit.edu/~mtf/teaching/20205/psr/assignments.html#prog04),
although larger assignments are often more interesting.

Individual or Pair assignment.  Each individual student and one member of each
pair should complete the [myCourses
survey](https://mycourses.rit.edu/d2l/lms/survey/user/surveys_list.d2l?ou=888966)
by Tue. 03/09 to indicate how the assignment will be undertaken. A Git
repository on [`https://git.cs.rit.edu/psr2205`](https://git.cs.rit.edu/psr2205)
will be created for each individual/pair for the assignment.

The assignment should have the following structure:

- `README.md` or `README.adoc`: assignment writeup, in either
  [Markdown](https://git.cs.rit.edu/help/user/markdown.html) or
  [AsciiDoc](https://git.cs.rit.edu/help/user/asciidoc.html) format as supported
  by [`git.cs.rit.edu`](https://git.cs.rit.edu) (GitLab)
- `soln`: reference solution
  * `Crate.toml` and `Crate.lock`: `[workspace]` definition, if multi-crate
    assignment; _optional_
  * `assets`: directory of shared assets, if multi-crate assignment; _optional_
  * _`crate1`_: first (and possibly only) assignment crate
    + `Crate.toml` and `Crate.lock`: crate manifest
    + `src`: directory of source files, must include `lib.rs` or `main.rs` and
      may include additional files
    + `assets`: directory of crate-level assets; _optional_
  * _`crate2`_, ..., _`craten`_: second and additional crates, with directory
    structure similar to _`crate1`_; _optional
- `starter`: student starter code; should be a copy of the `soln` directory
  except that any code to be written for the assignment has been removed and
  replaced by `unimplemented!()` or equivalent
