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
    ZeroPageX,
    ZeroPageDirectIndirectIndexedY,
    ZeroPageDirectIndexedIndirectX,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
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

    pub fn decode_and_execute(&mut self, bus: &mut Bus, opcode: u8) {
        match opcode {
            0xA1 | 0xA3 | 0xA5 | 0xA9 | 0xB1 | 0xB5 => self.lda(bus, opcode),
            _ => panic!("invalid opcode {}", opcode),
        }
    }

    pub fn start(&mut self, bus: &mut Bus) {
        loop {
            let opcode = bus.read_byte(self.pc);
            self.decode_and_execute(bus, opcode);
            self.pc += 1;
        }
    }

    pub fn addr_pbr(&mut self) -> u32 {
        (self.reg_pb << 16) | self.pc
    }

    pub fn fetch(&mut self, bus: &mut Bus, mode: AddressMode) -> Result<u32, String> {
        match mode {
            AddressMode::Immediate => {
                self.pc += 1;
                let data_lo = bus.read_dword(self.addr_pbr());
                self.pc += 1;
                let data_hi = bus.read_dword(self.addr_pbr());

                Ok((data_hi << 8) | data_lo)
            }
            AddressMode::ZeroPage => {
                self.pc += 1;
                let direct_offset = bus.read_byte(self.addr_pbr()) as u32;
                let data_lo = bus.read_dword(self.reg_d + direct_offset);
                let data_hi = bus.read_dword(self.reg_d + direct_offset + 1);

                Ok((data_hi << 8) | data_lo)
            }
            AddressMode::ZeroPageX => {
                self.pc += 1;
                let direct_offset = bus.read_byte(self.addr_pbr()) as u32;
                let data_lo = bus.read_dword(self.reg_d + direct_offset + self.reg_x);
                let data_hi = bus.read_dword(self.reg_d + direct_offset + self.reg_x + 1);

                Ok((data_hi << 8) | data_lo)
            }
            AddressMode::ZeroPageDirectIndirectIndexedY => {
                self.pc += 1;
                let direct_offset = bus.read_byte(self.addr_pbr()) as u32;
                let addr_lo = bus.read_byte(self.reg_d + direct_offset) as u32;
                let addr_hi = bus.read_byte(self.reg_d + direct_offset + 1) as u32;
                let data_lo = bus.read_dword(
                    (self.reg_db << 16) | (((addr_hi << 16) | addr_lo) + self.reg_y) as u32,
                );
                let data_hi = bus.read_dword(
                    (self.reg_db << 16) | (((addr_hi << 16) | addr_lo) + self.reg_y + 1) as u32,
                );

                Ok((data_hi << 8) | data_lo)
            }
            AddressMode::ZeroPageDirectIndexedIndirectX => {
                self.pc += 1;
                let direct_offset = bus.read_byte(self.addr_pbr()) as u32;
                let addr_lo = bus.read_byte(self.reg_d + direct_offset + self.reg_x) as u32;
                let addr_hi = bus.read_byte(self.reg_d + direct_offset + self.reg_x + 1) as u32;
                let data_lo =
                    bus.read_dword((self.reg_db << 16) | ((addr_hi << 8) | addr_lo) as u32);
                let data_hi =
                    bus.read_dword((self.reg_db << 16) | (((addr_hi << 8) | addr_lo) + 1) as u32);

                Ok((data_hi << 8) | data_lo)
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

    fn lda(&mut self, bus: &mut Bus, opcode: u8) {
        let result = match opcode {
            0xA9 => self.fetch(bus, AddressMode::Immediate),
            0xA5 => self.fetch(bus, AddressMode::ZeroPage),
            0xB5 => self.fetch(bus, AddressMode::ZeroPageX),
            0xA1 => self.fetch(bus, AddressMode::ZeroPageDirectIndexedIndirectX),
            0xB1 => self.fetch(bus, AddressMode::ZeroPageDirectIndirectIndexedY),
            _ => Err(format!("invalid opcode {}", opcode)),
        };

        match result {
            Ok(value) => self.reg_a = value,
            Err(msg) => panic!("LDA {} : {}", opcode, msg),
        }

        self.nz(self.reg_a);
    }
}
