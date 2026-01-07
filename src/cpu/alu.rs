use core::panic;

use crate::cpu::bus::Bus;

const S_CARRY: u32 = 0x1;
const S_RESULT_ZERO: u32 = 0x1 << 1;
const S_IRQ_DISABLE: u8 = 0x1 << 2;
const S_DECIMAL_MODE: u8 = 0x1 << 3;
const S_BREAK_INSTRUCTION: u8 = 0x1 << 4;
const S_OVERFLOW: u32 = 0x1 << 6;
const S_NEGATIVE: u32 = 0x1 << 7;

pub struct Cpu {
    bus: Box<Bus>,
    reg_a: u32,
    reg_x: u32,
    reg_y: u32,
    reg_p: u32,
    reg_d: u32,
    reg_pb: u32,
    reg_db: u32,
    sp: u32,
    pc: u32,
    emulation: bool,
}

#[derive(Debug)]
pub enum AddressMode {
    Immediate,
    ZeroPage,
    Absolute,
    AbsoluteIndexedX,
    AbsoluteIndexedY,
    ZeroPageX,
    ZeroPageDirectIndirectIndexedY,
    ZeroPageDirectIndexedIndirectX,
    AbsoluteLong,
    AbsoluteLongIndexedX,
    DirectIndirect,
    DirectIndirectLong,
    ZeroPageDirectIndirectIndexedLong,
    StackRelative,
    StackRelativeIndirectIndexedY,
}

impl Cpu {
    pub fn new(bus: Box<Bus>) -> Self {
        Self {
            bus: bus,
            sp: 0xFF,
            reg_a: 0x0,
            reg_x: 0x0,
            reg_y: 0x0,
            reg_p: 0x0,
            reg_d: 0x0,
            pc: 0x0,
            reg_pb: 0x0,
            reg_db: 0x0,
            emulation: false,
        }
    }

    pub fn start(&mut self) {
        loop {
            let opcode = self.bus.read_byte(self.pc);
            self.decode_and_execute(opcode);
            self.pc += 1;
        }
    }

    fn pbr_pc(&self) -> u32 {
        (self.reg_pb << 16) | self.pc
    }

    fn make_word(lo: u32, hi: u32) -> u32 {
        (hi << 8) | lo
    }

    fn decode_and_execute(&mut self, opcode: u8) {
        match opcode {
            // LDA
            0xA9 | 0xA5 | 0xB5 | 0xAD | 0xBD | 0xB9 | 0xA1 | 0xB1 | 0xAF | 0xBF | 0xB2 | 0xA7
            | 0xB7 | 0xA3 | 0xB3 => self.lda(opcode),

            // BRANCH
            0x10 | 0x30 | 0x50 | 0x70 | 0x90 | 0xB0 | 0xD0 | 0xF0 | 0x80 => self.branch(opcode),

            // ERROR
            _ => panic!("invalid opcode {}", opcode),
        }
    }

    fn fetch(&mut self, mode: AddressMode) -> Result<u32, String> {
        match mode {
            AddressMode::Absolute => {
                self.pc += 1;
                let addr_lo = self.bus.read_dword(self.pbr_pc());

                self.pc += 1;
                let addr_hi = self.bus.read_dword(self.pbr_pc());

                let addr = Self::make_word(addr_lo, addr_hi);

                let data_lo = self.bus.read_dword((self.reg_db << 16) | addr);
                let data_hi = self.bus.read_dword((self.reg_db << 16) | (addr + 1));

                Ok(Self::make_word(data_lo, data_hi))
            }

            AddressMode::AbsoluteIndexedX => {
                self.pc += 1;
                let addr_lo = self.bus.read_dword(self.pbr_pc());

                self.pc += 1;
                let addr_hi = self.bus.read_dword(self.pbr_pc());

                let addr = Self::make_word(addr_lo, addr_hi) + self.reg_x;

                let data_lo = self.bus.read_dword((self.reg_db << 16) | addr);
                let data_hi = self.bus.read_dword((self.reg_db << 16) | (addr + 1));

                Ok(Self::make_word(data_lo, data_hi))
            }

            AddressMode::AbsoluteIndexedY => {
                self.pc += 1;
                let addr_lo = self.bus.read_dword(self.pbr_pc());

                self.pc += 1;
                let addr_hi = self.bus.read_dword(self.pbr_pc());

                let addr = Self::make_word(addr_lo, addr_hi) + self.reg_y;

                let data_lo = self.bus.read_dword((self.reg_db << 16) | addr);
                let data_hi = self.bus.read_dword((self.reg_db << 16) | (addr + 1));

                Ok(Self::make_word(data_lo, data_hi))
            }

            AddressMode::Immediate => {
                self.pc += 1;
                let data_lo = self.bus.read_dword(self.pbr_pc());

                self.pc += 1;
                let data_hi = self.bus.read_dword(self.pbr_pc());

                Ok(Self::make_word(data_lo, data_hi))
            }

            AddressMode::ZeroPage => {
                self.pc += 1;
                let direct_offset = self.bus.read_byte(self.pbr_pc()) as u32;

                let data_lo = self.bus.read_dword(self.reg_d + direct_offset);
                let data_hi = self.bus.read_dword(self.reg_d + direct_offset + 1);

                Ok(Self::make_word(data_lo, data_hi))
            }

            AddressMode::ZeroPageX => {
                self.pc += 1;
                let direct_offset = self.bus.read_byte(self.pbr_pc()) as u32;

                let data_lo = self.bus.read_dword(self.reg_d + direct_offset + self.reg_x);
                let data_hi = self
                    .bus
                    .read_dword(self.reg_d + direct_offset + self.reg_x + 1);

                Ok(Self::make_word(data_lo, data_hi))
            }

            AddressMode::ZeroPageDirectIndirectIndexedY => {
                self.pc += 1;
                let direct_offset = self.bus.read_dword(self.pbr_pc());

                let addr_lo = self.bus.read_dword(self.reg_d + direct_offset);
                let addr_hi = self.bus.read_dword(self.reg_d + direct_offset + 1);
                let addr = Self::make_word(addr_lo, addr_hi) + self.reg_y;

                let data_lo = self.bus.read_dword((self.reg_db << 16) | addr);
                let data_hi = self.bus.read_dword((self.reg_db << 16) | (addr + 1));

                Ok(Self::make_word(data_lo, data_hi))
            }

            AddressMode::ZeroPageDirectIndexedIndirectX => {
                self.pc += 1;
                let direct_offset = self.bus.read_dword(self.pbr_pc());

                let addr_lo = self.bus.read_dword(self.reg_d + direct_offset + self.reg_x);
                let addr_hi = self
                    .bus
                    .read_dword(self.reg_d + direct_offset + self.reg_x + 1);
                let addr = Self::make_word(addr_lo, addr_hi);

                let data_lo = self.bus.read_dword((self.reg_db << 16) | addr);
                let data_hi = self.bus.read_dword((self.reg_db << 16) | (addr + 1));

                Ok(Self::make_word(data_lo, data_hi))
            }

            AddressMode::AbsoluteLong => {
                self.pc += 1;
                let addr_lo = self.bus.read_dword(self.pbr_pc());

                self.pc += 1;
                let addr_hi = self.bus.read_dword(self.pbr_pc());

                self.pc += 1;
                let addr_bank = self.bus.read_dword(self.pbr_pc());

                let addr = Self::make_word(addr_lo, addr_hi);

                let data_lo = self.bus.read_dword((addr_bank << 16) | addr);
                let data_hi = self.bus.read_dword((addr_bank << 16) | (addr + 1));

                Ok(Self::make_word(data_lo, data_hi))
            }

            AddressMode::AbsoluteLongIndexedX => {
                self.pc += 1;
                let addr_lo = self.bus.read_dword(self.pbr_pc());

                self.pc += 1;
                let addr_hi = self.bus.read_dword(self.pbr_pc());

                self.pc += 1;
                let addr_bank = self.bus.read_dword(self.pbr_pc());

                let addr = Self::make_word(addr_lo, addr_hi) + self.reg_x;

                let data_lo = self.bus.read_dword((addr_bank << 16) | addr);
                let data_hi = self.bus.read_dword((addr_bank << 16) | (addr + 1));

                Ok(Self::make_word(data_lo, data_hi))
            }

            AddressMode::DirectIndirect => {
                self.pc += 1;
                let direct_offset = self.bus.read_dword(self.pbr_pc());

                let addr_lo = self.bus.read_dword(self.reg_d + direct_offset);
                let addr_hi = self.bus.read_dword(self.reg_d + direct_offset + 1);
                let addr = Self::make_word(addr_lo, addr_hi);

                let data_lo = self.bus.read_dword((self.reg_db << 16) | addr);
                let data_hi = self.bus.read_dword((self.reg_db << 16) | (addr + 1));

                Ok(Self::make_word(data_lo, data_hi))
            }

            AddressMode::DirectIndirectLong => {
                self.pc += 1;
                let direct_offset = self.bus.read_dword(self.pbr_pc());

                let addr_lo = self.bus.read_dword(self.reg_d + direct_offset);
                let addr_hi = self.bus.read_dword(self.reg_d + direct_offset + 1);
                let addr_bank = self.bus.read_dword(self.reg_d + direct_offset + 2);

                let addr = Self::make_word(addr_lo, addr_hi);

                let data_lo = self.bus.read_dword((addr_bank << 16) | addr);
                let data_hi = self.bus.read_dword((addr_bank << 16) | (addr + 1));

                Ok(Self::make_word(data_lo, data_hi))
            }

            AddressMode::ZeroPageDirectIndirectIndexedLong => {
                self.pc += 1;
                let direct_offset = self.bus.read_dword(self.pbr_pc());

                let addr_lo = self.bus.read_dword(self.reg_d + direct_offset);
                let addr_hi = self.bus.read_dword(self.reg_d + direct_offset + 1);
                let addr_bank = self.bus.read_dword(self.reg_d + direct_offset + 2);

                let addr = Self::make_word(addr_lo, addr_hi) + self.reg_y;

                let data_lo = self.bus.read_dword((addr_bank << 16) | addr);
                let data_hi = self.bus.read_dword((addr_bank << 16) | (addr + 1));

                Ok(Self::make_word(data_lo, data_hi))
            }

            AddressMode::StackRelative => {
                self.pc += 1;
                let stack_offset = self.bus.read_dword(self.pbr_pc());

                let data_lo = self.bus.read_dword(self.sp + stack_offset);
                let data_hi = self.bus.read_dword(self.sp + stack_offset + 1);

                Ok(Self::make_word(data_lo, data_hi))
            }

            AddressMode::StackRelativeIndirectIndexedY => {
                self.pc += 1;
                let stack_offset = self.bus.read_dword(self.pbr_pc());

                let addr_lo = self.bus.read_dword(self.sp + stack_offset);
                let addr_hi = self.bus.read_dword(self.sp + stack_offset + 1);

                let addr = Self::make_word(addr_lo, addr_hi) + self.reg_y;

                let data_lo = self.bus.read_dword(addr);
                let data_hi = self.bus.read_dword(addr + 1);

                Ok(Self::make_word(data_lo, data_hi))
            }
        }
    }

    fn nz(&mut self, value: u32) {
        if value == 0 {
            self.reg_p |= S_RESULT_ZERO;
        } else {
            self.reg_p &= !S_RESULT_ZERO;
        }

        // 8 or 16 bit (6502 emulation on/off)
        if (self.emulation && value & 0x80 > 0) || (!self.emulation && value & 0x8000 > 0) {
            self.reg_p |= S_NEGATIVE;
        } else {
            self.reg_p &= !S_NEGATIVE;
        }
    }

    fn branch(&mut self, opcode: u8) {
        let taken: Result<bool, String> = match opcode {
            // BNE
            0xD0 => Ok((self.reg_p & S_RESULT_ZERO) == 0),

            // BEQ
            0xF0 => Ok((self.reg_p & S_RESULT_ZERO) > 0),

            // BMI
            0x30 => Ok((self.reg_p & S_NEGATIVE) > 0),

            // BPL
            0x10 => Ok((self.reg_p & S_NEGATIVE) == 0),

            // BCS
            0xB0 => Ok((self.reg_p & S_CARRY) > 0),

            // BCC
            0x90 => Ok((self.reg_p & S_CARRY) == 0),

            // BVC
            0x50 => Ok((self.reg_p & S_OVERFLOW) == 0),

            // BVS
            0x70 => Ok((self.reg_p & S_OVERFLOW) > 0),

            // BRA
            0x80 => Ok(true),

            _ => Err(format!("branch : unknown opcode {}", opcode)),
        };

        match taken {
            Ok(taken) => {
                self.pc += 1;
                let offset = self.bus.read_byte(self.pbr_pc());

                // TODO: Add 1 more cycle if branch is taken
                if taken {
                    // if signed, flip all bits and add 1 to get real value, then subtract from PC
                    // this is because offset is a one byte signed two's-complement
                    self.pc = match (offset & 0x80) > 0 {
                        true => self.pc - ((!offset) + 1) as u32,
                        false => self.pc + offset as u32,
                    };
                }
            }
            Err(msg) => panic!("error : {}", msg),
        }
    }

    fn lda(&mut self, opcode: u8) {
        let result = match opcode {
            0xA9 => self.fetch(AddressMode::Immediate),
            0xA5 => self.fetch(AddressMode::ZeroPage),
            0xB5 => self.fetch(AddressMode::ZeroPageX),
            0xAD => self.fetch(AddressMode::Absolute),
            0xBD => self.fetch(AddressMode::AbsoluteIndexedX),
            0xB9 => self.fetch(AddressMode::AbsoluteIndexedY),
            0xA1 => self.fetch(AddressMode::ZeroPageDirectIndexedIndirectX),
            0xB1 => self.fetch(AddressMode::ZeroPageDirectIndirectIndexedY),
            0xAF => self.fetch(AddressMode::AbsoluteLong),
            0xBF => self.fetch(AddressMode::AbsoluteLongIndexedX),
            0xB2 => self.fetch(AddressMode::DirectIndirect),
            0xA7 => self.fetch(AddressMode::DirectIndirectLong),
            0xB7 => self.fetch(AddressMode::ZeroPageDirectIndirectIndexedLong),
            0xA3 => self.fetch(AddressMode::StackRelative),
            0xB3 => self.fetch(AddressMode::StackRelativeIndirectIndexedY),

            _ => Err(format!("invalid opcode {}", opcode)),
        };

        match result {
            Ok(value) => self.reg_a = value,
            Err(msg) => panic!("LDA {} : {}", opcode, msg),
        }

        self.nz(self.reg_a);
    }
}
