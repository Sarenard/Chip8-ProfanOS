use core::panic;

use crate::chip8;

use crate::chip8::insts::Instruction;

use rust_profanos::println;

use alloc::{vec, vec::Vec, format};

pub trait PixelHandler {
    fn set_pixel(&mut self, x: usize, y: usize, on: bool);
}

pub trait KeyboardHandler {
    fn is_pressed(&mut self, key: u8) -> bool;
}

pub trait RandomHandler {
    fn random(&mut self) -> u8;
}

static FONT: [u8; 80] = [
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

static KEYBOARDMAP: [usize; 16] = [
    13, 0, 1, 2, 4, 5, 6, 8, 9, 10, 12, 14, 3, 7, 11, 15
];

pub struct VM<T: PixelHandler, T2: KeyboardHandler, T3: RandomHandler> {
    pub memory: [u8; 4096], // 4096 bytes
    registers: [u8; 16], // 8-bit data registers
    stack: Vec<u16>,
    i: u16,
    programcounter: usize,
    delaytimer: u8,
    soundtimer: u8,
    pub pixelhandler: T,
    pub keyboardhandler: T2,
    pub rng: T3,
    framebuffer: [[bool; 32]; 64],
}

impl<T: PixelHandler, T2: KeyboardHandler, T3: RandomHandler> VM<T, T2, T3> {
    pub fn new(pixelhandler: T, keyboardhandler: T2, randomhandler: T3) -> Self {
        let mut memory: [u8; 4096] = [0; 4096];
        memory[..FONT.len()].copy_from_slice(&FONT);
        VM {
            memory: memory,
            registers: [0; 16],
            stack: vec![],
            i: 0,
            programcounter: 0x200, // start of programs
            delaytimer: 0,
            soundtimer: 0,
            pixelhandler,
            keyboardhandler,
            rng: randomhandler,
            framebuffer: [[false; 32]; 64],
        }
    }

    pub fn update_pixel(&mut self, x: usize, y: usize, forceblack: bool) {
        if x >= 64 || y >= 32 {
            //panic!("Out of bounds write");
            return;
        }
        if forceblack {
            self.pixelhandler.set_pixel(x, y, false);
            self.framebuffer[x][y] = false;
            return;
        }
        self.pixelhandler.set_pixel(x, y, !self.framebuffer[x][y]);
        self.framebuffer[x][y] = !self.framebuffer[x][y];
    }

    pub fn check_key(&mut self, key: u8) -> bool {
        self.keyboardhandler.is_pressed(key)
    }

    pub fn decrease_timer(&mut self) {
        if self.delaytimer > 0 {
            self.delaytimer -= 1;
        }
        if self.soundtimer > 0 {
            self.soundtimer -= 1;
        }
    }

    pub fn setmemory(&mut self, content: Vec<u8>) {
        let max_memory_size = self.memory.len() - 0x200; // Reserve the first 512 bytes for system area and font set
    
        let memory_size = core::cmp::min(content.len(), max_memory_size);
    
        self.memory[0x200..0x200 + memory_size].copy_from_slice(&content[..memory_size]);
    }

    fn random(&mut self) -> u8 {
        self.rng.random()
    }

    pub fn process(&mut self) {
        let instruction1: u8 = self.memory[self.programcounter];
        let instruction2: u8 = self.memory[self.programcounter + 1];
        let instruction = ((instruction1 as u16) << 8) | instruction2 as u16;

        let instruction = chip8::insts::Instruction::new(instruction);

        let s = format!("instruction : {:?}", instruction);
        println!("{}", s);

        match instruction {
            Instruction::ClearScreen => {
                for i in 0..64 {
                    for j in 0..32 {
                        self.update_pixel(i, j, true);
                    }
                }
            },

            Instruction::Jump(val) => {
                self.programcounter = val as usize - 2;
            }

            Instruction::SetRegister(reg, val) => {
                self.registers[reg as usize] = val;
            }

            Instruction::AddRegister(reg, val) => {
                self.registers[reg as usize] = (val as usize + self.registers[reg as usize] as usize) as u8;
            }

            Instruction::SetI(val) => {
                self.i = val;
            }

            Instruction::Draw(reg1, reg2, size) => {
                let x = self.registers[reg1 as usize] % 64; // wrap
                let y = self.registers[reg2 as usize] % 32; // wrap
                self.registers[15] = 0;
                for i in 0..size { // pour chaque ligne
                    if y + i as u8 > 32 { // if we are outside of the screen
                        break;
                    }
                    let byte = self.memory[self.i as usize + i as usize];
                    for off in 0..8 {
                        let bit = ((byte & (0x1 << off)) >> off) == 1;
                        if bit {
                            self.update_pixel((x+7-off) as usize, (y+i as u8) as usize, false);
                        }
                    }
                }
            }

            Instruction::Call(addr) => {
                self.stack.push(self.programcounter as u16);
                self.programcounter = addr as usize - 2;
            }

            Instruction::Ret => {
                let val = self.stack.pop();
                match val {
                    Some(value) => {
                        self.programcounter = value as usize;
                    }
                    None => {
                        panic!("Ret sans Call !");
                    }
                }
            }

            Instruction::SkipNextInstruction(reg, val) => {
                let val_reg = self.registers[reg as usize] as u16;
                if val_reg == val {
                    self.programcounter += 2;
                }
            }

            Instruction::NSkipNextInstruction(reg, val) => {
                let val_reg = self.registers[reg as usize] as u16;
                if val_reg != val {
                    self.programcounter += 2;
                }
            }

            Instruction::R2SkipNextInstruction(reg1, reg2) => {
                let val_reg1 = self.registers[reg1 as usize] as u16;
                let val_reg2 = self.registers[reg2 as usize] as u16;
                if val_reg1 == val_reg2 {
                    self.programcounter += 2;
                }
            }

            Instruction::NR2SkipNextInstruction(reg1, reg2) => {
                let val_reg1 = self.registers[reg1 as usize] as u16;
                let val_reg2 = self.registers[reg2 as usize] as u16;
                if val_reg1 != val_reg2 {
                    self.programcounter += 2;
                }
            }

            Instruction::ERROR(nb) => {
                panic!("Bytecode not understood : {}", nb);
            }

            Instruction::STORE(a, b) => {
                self.registers[a as usize] = self.registers[b as usize];
            }

            Instruction::OR(a, b) => {
                self.registers[a as usize] = self.registers[a as usize] | self.registers[b as usize];
            }

            Instruction::AND(a, b) => {
                self.registers[a as usize] = self.registers[a as usize] & self.registers[b as usize];
            }

            Instruction::XOR(a, b) => {
                self.registers[a as usize] = self.registers[a as usize] ^ self.registers[b as usize];
            }

            Instruction::ADD(a, b) => {
                let tot = self.registers[a as usize] as usize + self.registers[b as usize] as usize;
                self.registers[a as usize] = (tot % 256) as u8;
                self.registers[15] = (tot % 256) as u8;
            }

            Instruction::SUB(a, b) => {
                let reg1 = self.registers[a as usize];
                let reg2 = self.registers[b as usize];
                self.registers[15] = (reg1 > reg2) as u8;
                self.registers[a as usize] = reg1.wrapping_sub(reg2);
            }

            Instruction::SHR(a) => {
                let reg1 = self.registers[a as usize];
                self.registers[15] = reg1%2;
                self.registers[a as usize] = self.registers[a as usize] / 2;
            }

            Instruction::SUBN(a, b) => {
                let reg1 = self.registers[a as usize];
                let reg2 = self.registers[b as usize];
                self.registers[15] = (reg2 > reg1) as u8;
                self.registers[a as usize] = reg2.wrapping_sub(reg1);
            }

            Instruction::SHL(a) => {
                let reg1 = self.registers[a as usize];
                self.registers[15] = (reg1 & 0x80) as u8;
                self.registers[a as usize] = reg1.wrapping_mul(2);
            }

            Instruction::AddI(reg) => {
                self.i += self.registers[reg as usize] as u16;
            }

            Instruction::ReadDelay(a) => {
                self.registers[a as usize] = self.delaytimer;
            }

            Instruction::SetDelay(a) => {
                self.delaytimer = self.registers[a as usize];
            }
            
            Instruction::SkipIfPressed(x) => {
                let val = self.registers[x as usize];
                let key_pressed = self.check_key(KEYBOARDMAP[val as usize] as u8);
                if key_pressed {
                    self.programcounter += 2;
                }
            }

            Instruction::SkipIfNotPressed(x) => {
                let val = self.registers[x as usize];
                let key_pressed = self.check_key(KEYBOARDMAP[val as usize] as u8);
                if !key_pressed {
                    self.programcounter += 2;
                }
            }

            Instruction::WaitKey(x) => {
                if !self.check_key(self.registers[KEYBOARDMAP[x as usize]]) {
                    self.programcounter -= 2
                }
            }

            Instruction::SetSound(x) => {
                self.soundtimer = self.registers[x as usize];
            }

            Instruction::SpriteDigit(x) => {
                let val = self.registers[x as usize];
                match val {
                    val if val > 9 => {
                        panic!("Too big !");
                    }
                    val =>  {
                        self.i = 5*val as u16;
                    }
                }
            }

            Instruction::StoreRegisters(nb) => {
                for i in 0..nb+1 {
                    self.memory[self.i as usize + i as usize] = self.registers[i as usize];
                }
            }
            
            Instruction::ReadRegisters(nb) => {
                for i in 0..nb+1 {
                    self.registers[i as usize] = self.memory[self.i as usize + i as usize];
                }
            }

            Instruction::StoreBCD(reg)  => {
                let val = self.registers[reg as usize];
                self.memory[self.i as usize + 0] = val / 100;
                self.memory[self.i as usize + 1] = (val / 10) % 10;
                self.memory[self.i as usize + 2] = val % 10;
            }

            Instruction::Random(x, kk) => {
                self.registers[x as usize] = self.random() & (kk as u8);
            }

            Instruction::Jump2(val) => {
                let reg = self.registers[0];
                self.programcounter = val as usize + reg as usize;
            }
        }

        self.programcounter += 2;
    }
}
