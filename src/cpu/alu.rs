use core::panic;

use crate::cpu::bus::Bus;

const S_CARRY: u8 = 0x1;
const S_RESULT_ZERO: u32 = 0x1 << 1;
const S_IRQ_DISABLE: u8 = 0x1 << 2;
const S_DECIMAL_MODE: u8 = 0x1 << 3;
const S_BREAK_INSTRUCTION: u8 = 0x1 << 4;
const S_OVERFLOW: u8 = 0x1 << 6;
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
            0xA1 | 0xA3 | 0xA5 | 0xA9 | 0xAD | 0xB1 | 0xB5 | 0xBD | 0xB9 => self.lda(opcode),
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
        }
    }

    fn nz(&mut self, value: u32) {
        if value == 0 {
            self.reg_p |= S_RESULT_ZERO;
        }
        if value & 0x80 > 0 {
            self.reg_p |= S_NEGATIVE;
        } else {
            self.reg_p &= !S_NEGATIVE;
        }
    }

    fn lda(&mut self, opcode: u8) {
        let result = match opcode {
            0xA9 => self.fetch(AddressMode::Immediate),
            0xA5 => self.fetch(AddressMode::ZeroPage),
            0xAD => self.fetch(AddressMode::Absolute),
            0xB5 => self.fetch(AddressMode::ZeroPageX),
            0xA1 => self.fetch(AddressMode::ZeroPageDirectIndexedIndirectX),
            0xB1 => self.fetch(AddressMode::ZeroPageDirectIndirectIndexedY),
            0xBD => self.fetch(AddressMode::AbsoluteIndexedX),
            0xB9 => self.fetch(AddressMode::AbsoluteIndexedY),
            _ => Err(format!("invalid opcode {}", opcode)),
        };

        match result {
            Ok(value) => self.reg_a = value,
            Err(msg) => panic!("LDA {} : {}", opcode, msg),
        }

        self.nz(self.reg_a);
    }
}
