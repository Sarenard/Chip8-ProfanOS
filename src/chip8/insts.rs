use rust_profanos::println;

use core::ffi::c_uint;

#[derive(Debug, Clone)]
pub enum Instruction {
    ClearScreen,
    Jump(u16), // addr
    SetRegister(u8, u8),
    AddRegister(u8, u8),
    SetI(u16),
    Draw(u16, u16, u16),
    Call(u16), // addr
    Ret,
    SkipNextInstruction(u16, u16),
    NSkipNextInstruction(u16, u16),
    R2SkipNextInstruction(u16, u16),
    NR2SkipNextInstruction(u16, u16),
    STORE(u16, u16),
    OR(u16, u16),
    AND(u16, u16),
    XOR(u16, u16),
    ADD(u16, u16),
    SUB(u16, u16),
    SHR(u16),
    SUBN(u16, u16),
    SHL(u16),
    ReadDelay(u16),
    WaitKey(u16),
    SetDelay(u16),
    SetSound(u16),
    AddI(u16),
    SpriteDigit(u16),
    #[allow(dead_code)]
    StoreBCD(u16),
    #[allow(dead_code)]
    StoreRegisters(u16),
    #[allow(dead_code)]
    ReadRegisters(u16),
    SkipIfPressed(u16),
    SkipIfNotPressed(u16),
    Random(u16, u16),
    Jump2(u16),

    ERROR(u16), // unknown opcode
}

impl Instruction {
    pub fn new(value: u16) -> Instruction {
        let lower1 = (value & 0x000F) >> 0;
        let upper1 = (value & 0x00F0) >> 4;
        let lower2 = (value & 0x0F00) >> 8;
        let upper2 = (value & 0xF000) >> 12;

        // println!("%d %d %d %d", upper2 as c_uint, lower2 as c_uint, upper1 as c_uint, lower1 as c_uint);
        match (upper2, lower2, upper1, lower1) {
            (0x0, 0x0, 0xe, 0x0) => {
                return Instruction::ClearScreen;
            }

            (0x0, 0x0, 0xe, 0xe) => {
                return Instruction::Ret;
            }

            (0x1, n1, n2, n3) => {
                let val: u16 = n1 << 8 | n2 << 4 | n3;
                return Instruction::Jump(val);
            }

            (0x2, n1, n2, n3) => {
                let val: u16 = n1 << 8 | n2 << 4 | n3;
                return Instruction::Call(val);
            }

            (0x3, x, n1, n2) => {
                let val: u16 = (n1 << 4 | n2) % 256;
                return Instruction::SkipNextInstruction(x, val);
            }

            (0x4, x, n1, n2) => {
                let val: u16 = (n1 << 4 | n2) % 256;
                return Instruction::NSkipNextInstruction(x, val);
            }

            (0x5, x, y, 0x0) => {
                return Instruction::R2SkipNextInstruction(x, y);
            }

            (0x6, x, n1, n2) => {
                let val: u8 = (n1 << 4 | n2).try_into().unwrap();
                return Instruction::SetRegister(x.try_into().unwrap(), val);
            }

            (0x7, x, n1, n2) => {
                let val: u8 = (n1 << 4 | n2).try_into().unwrap();
                return Instruction::AddRegister(x.try_into().unwrap(), val);
            }

            (0x8, x, y, 0x0) => {
                return Instruction::STORE(x, y);
            }

            (0x8, x, y, 0x1) => {
                return Instruction::OR(x, y);
            }

            (0x8, x, y, 0x2) => {
                return Instruction::AND(x, y);
            }

            (0x8, x, y, 0x3) => {
                return Instruction::XOR(x, y);
            }

            (0x8, x, y, 0x4) => {
                return Instruction::ADD(x, y);
            }

            (0x8, x, y, 0x5) => {
                return Instruction::SUB(x, y);
            }

            (0x8, x, _y, 0x6) => {
                return Instruction::SHR(x);
            }

            (0x8, x, y, 0x7) => {
                return Instruction::SUBN(x, y);
            }

            (0x8, x, _y, 0xE) => {
                return Instruction::SHL(x);
            }

            (0x9, x, y, 0x0) => {
                return Instruction::NR2SkipNextInstruction(x, y);
            }

            (0xa, n1, n2, n3) => {
                let val: u16 = n1 << 8 | n2 << 4 | n3;
                return Instruction::SetI(val);
            }

            (0xb, n1, n2, n3) => {
                let val: u16 = n1 << 8 | n2 << 4 | n3;
                return Instruction::Jump2(val);
            }

            (0xc, x, n1, n2) => {
                let val: u16 = n1 << 4 | n2;
                return Instruction::Random(x, val);
            }

            (0xd, x, y, n) => {
                return Instruction::Draw(x, y, n);
            }

            (0xe, x, 0x9, 0xe) => {
                return Instruction::SkipIfPressed(x);
            }

            (0xe, x, 0xa, 0x1) => {
                return Instruction::SkipIfNotPressed(x);
            }
            
            (0xf, x, 0x0, 0x7) => {
                return Instruction::ReadDelay(x);
            }

            (0xf, x, 0x0, 0xa) => {
                return Instruction::WaitKey(x);
            }

            (0xf, x, 0x1, 0x5) => {
                return Instruction::SetDelay(x);
            }

            (0xf, x, 0x1, 0x8) => {
                return Instruction::SetSound(x);
            }

            (0xf, x, 0x1, 0xe) => {
                return Instruction::AddI(x);
            }

            (0xf, x, 0x2, 0x9) => {
                return Instruction::SpriteDigit(x);
            }

            (0xf, x, 0x3, 0x3) => {
                return Instruction::StoreBCD(x);
            }

            (0xf, x, 0x5, 0x5) => {
                return Instruction::StoreRegisters(x);
            }

            (0xf, x, 0x6, 0x5) => {
                return Instruction::ReadRegisters(x);
            }

            _ => {
                return Instruction::ERROR(value);
            }
        }
    }
}