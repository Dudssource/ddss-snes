use crate::cpu::{bits::Word, bus::Bus};

const S_CARRY: u8 = 0x1;
const S_RESULT_ZERO: u8 = 0x1 << 1;
const S_IRQ_DISABLE: u8 = 0x1 << 2;
const S_DECIMAL_MODE: u8 = 0x1 << 3;
const S_BREAK_INSTRUCTION: u8 = 0x1 << 4;
const S_OVERFLOW: u8 = 0x1 << 6;
const S_NEGATIVE: u8 = 0x1 << 7;

pub struct Cpu {
    bus: Box<Bus>,
    reg_a: Word,
    reg_x: u16,
    reg_y: u16,
    reg_p: u8,
    reg_d: u16,
    reg_pb: u8,
    reg_db: u8,
    sp: u32,
    pc: u16,
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
    ZeroPageY,
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
            reg_a: Word::new(0, 0),
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
            let opcode = self.bus.read_byte(self.pc as u32);
            self.decode_and_execute(opcode);
            self.pc += 1;
        }
    }

    fn pbr_pc(&self) -> u32 {
        ((self.reg_pb as u32) << 16) | self.pc as u32
    }

    fn make_word(lo: u8, hi: u8) -> u16 {
        ((hi as u16) << 8) | lo as u16
    }

    fn decode_and_execute(&mut self, opcode: u8) {
        match opcode {
            // STA Store accumulator in memory
            0x85 | 0x95 | 0x8D | 0x9D | 0x99 | 0x81 | 0x91 | 0x8F | 0x9F | 0x92 | 0x87 | 0x97
            | 0x83 | 0x93 => self.op_sta(opcode),

            // LDA Load Accumulator with Memory
            0xA9 | 0xA5 | 0xB5 | 0xAD | 0xBD | 0xB9 | 0xA1 | 0xB1 | 0xAF | 0xBF | 0xB2 | 0xA7
            | 0xB7 | 0xA3 | 0xB3 => self.op_lda(opcode),

            // LDY Load index Y with memory
            0xA0 | 0xA4 | 0xB4 | 0xAC | 0xBC => self.op_ldy(opcode),

            // LDX Load index X with memory
            0xA2 | 0xA6 | 0xB6 | 0xAE | 0xBE => self.op_ldx(opcode),

            // CPY Compare Memory and Index Y
            0xC0 | 0xC4 | 0xCC => self.op_cpy(opcode),

            // CPX Compare Memory and Index X
            0xE0 | 0xE4 | 0xEC => self.op_cpx(opcode),

            // CMP Compare memory and accumulator
            0xC9 | 0xC5 | 0xD5 | 0xCD | 0xDD | 0xD9 | 0xC1 | 0xD1 | 0xCF | 0xDF | 0xD2 | 0xC7
            | 0xD7 | 0xC3 | 0xD3 => self.op_cmp(opcode),

            // BRANCH on
            0x10 | 0x30 | 0x50 | 0x70 | 0x90 | 0xB0 | 0xD0 | 0xF0 | 0x80 => self.branch(opcode),

            // JSR Jump to new location saving return address
            0x20 => self.op_jsr(),

            // JSL Jump Subroutine Long
            0x22 => self.op_jsl(),

            // REP Reset Status Bits
            0xC2 => self.op_rep(),

            // JML Jump Long
            0xDC => self.op_jml(),

            // SEP Set Processor Status Bits
            0xE2 => self.op_sep(),

            // PHA Push accumulator on stack
            0x48 => self.op_pha(),

            // PHB Push Data Bank Register on Stack
            0x8B => self.op_phb(),

            // PHD Push Direct Register on Stack
            0x0B => self.op_phd(),

            // PHK Push Program Bank Register on Stack
            0x4B => self.op_phk(),

            // PHP Push processor status on stack
            0x08 => self.op_php(),

            // PHX Push Index X on Stack
            0xDA => self.op_phx(),

            // PHY Push Index Y on Stack
            0x5A => self.op_phy(),

            // ERROR
            _ => panic!("invalid opcode {}", opcode),
        }
    }

    fn fetch(&mut self, mode: AddressMode) -> u16 {
        match mode {
            AddressMode::Absolute => {
                self.pc += 1;
                let addr_lo = self.bus.read_byte(self.pbr_pc());

                self.pc += 1;
                let addr_hi = self.bus.read_byte(self.pbr_pc());

                let addr = Self::make_word(addr_lo, addr_hi) as u32;

                let data_lo = self.bus.read_byte(((self.reg_db as u32) << 16) | addr);
                let data_hi = self.bus.read_byte(((self.reg_db as u32) << 16) | addr + 1);

                Self::make_word(data_lo, data_hi)
            }

            AddressMode::AbsoluteIndexedX => {
                self.pc += 1;
                let addr_lo = self.bus.read_byte(self.pbr_pc());

                self.pc += 1;
                let addr_hi = self.bus.read_byte(self.pbr_pc());

                let addr = (Self::make_word(addr_lo, addr_hi) + self.reg_x) as u32;

                let data_lo = self.bus.read_byte(((self.reg_db as u32) << 16) | addr);
                let data_hi = self
                    .bus
                    .read_byte(((self.reg_db as u32) << 16) | (addr + 1));

                Self::make_word(data_lo, data_hi)
            }

            AddressMode::AbsoluteIndexedY => {
                self.pc += 1;
                let addr_lo = self.bus.read_byte(self.pbr_pc());

                self.pc += 1;
                let addr_hi = self.bus.read_byte(self.pbr_pc());

                let addr = (Self::make_word(addr_lo, addr_hi) + self.reg_y) as u32;

                let data_lo = self.bus.read_byte(((self.reg_db as u32) << 16) | addr);
                let data_hi = self
                    .bus
                    .read_byte(((self.reg_db as u32) << 16) | (addr + 1));

                Self::make_word(data_lo, data_hi)
            }

            AddressMode::Immediate => {
                self.pc += 1;
                let data_lo = self.bus.read_byte(self.pbr_pc());

                self.pc += 1;
                let data_hi = self.bus.read_byte(self.pbr_pc());

                Self::make_word(data_lo, data_hi)
            }

            AddressMode::ZeroPage => {
                self.pc += 1;
                let direct_offset = self.bus.read_byte(self.pbr_pc()) as u32;

                let data_lo = self.bus.read_byte(self.reg_d as u32 + direct_offset);
                let data_hi = self.bus.read_byte(self.reg_d as u32 + direct_offset + 1);

                Self::make_word(data_lo, data_hi)
            }

            AddressMode::ZeroPageX => {
                self.pc += 1;
                let direct_offset = self.bus.read_byte(self.pbr_pc()) as u32;

                let data_lo = self
                    .bus
                    .read_byte((self.reg_d + self.reg_x) as u32 + direct_offset);
                let data_hi = self
                    .bus
                    .read_byte((self.reg_d + self.reg_x) as u32 + direct_offset + 1);

                Self::make_word(data_lo, data_hi)
            }

            AddressMode::ZeroPageY => {
                self.pc += 1;
                let direct_offset = self.bus.read_byte(self.pbr_pc()) as u32;

                let data_lo = self
                    .bus
                    .read_byte((self.reg_d + self.reg_y) as u32 + direct_offset);
                let data_hi = self
                    .bus
                    .read_byte((self.reg_d + self.reg_y) as u32 + direct_offset + 1);

                Self::make_word(data_lo, data_hi)
            }

            AddressMode::ZeroPageDirectIndirectIndexedY => {
                self.pc += 1;
                let direct_offset = self.bus.read_dword(self.pbr_pc());

                let addr_lo = self.bus.read_byte(self.reg_d as u32 + direct_offset);
                let addr_hi = self.bus.read_byte(self.reg_d as u32 + direct_offset + 1);
                let addr = (Self::make_word(addr_lo, addr_hi) + self.reg_y) as u32;

                let data_lo = self.bus.read_byte(((self.reg_db as u32) << 16) | addr);
                let data_hi = self
                    .bus
                    .read_byte(((self.reg_db as u32) << 16) | (addr + 1));

                Self::make_word(data_lo, data_hi)
            }

            AddressMode::ZeroPageDirectIndexedIndirectX => {
                self.pc += 1;
                let direct_offset = self.bus.read_dword(self.pbr_pc());

                let addr_lo = self
                    .bus
                    .read_byte((self.reg_d + self.reg_x) as u32 + direct_offset);
                let addr_hi = self
                    .bus
                    .read_byte((self.reg_d + self.reg_x) as u32 + direct_offset + 1);

                let addr = Self::make_word(addr_lo, addr_hi) as u32;

                let data_lo = self.bus.read_byte(((self.reg_db as u32) << 16) | addr);
                let data_hi = self
                    .bus
                    .read_byte(((self.reg_db as u32) << 16) | (addr + 1));

                Self::make_word(data_lo, data_hi)
            }

            AddressMode::AbsoluteLong => {
                self.pc += 1;
                let addr_lo = self.bus.read_byte(self.pbr_pc());

                self.pc += 1;
                let addr_hi = self.bus.read_byte(self.pbr_pc());

                self.pc += 1;
                let addr_bank = self.bus.read_dword(self.pbr_pc());

                let addr = Self::make_word(addr_lo, addr_hi) as u32;

                let data_lo = self.bus.read_byte((addr_bank << 16) | addr);
                let data_hi = self.bus.read_byte((addr_bank << 16) | (addr + 1));

                Self::make_word(data_lo, data_hi)
            }

            AddressMode::AbsoluteLongIndexedX => {
                self.pc += 1;
                let addr_lo = self.bus.read_byte(self.pbr_pc());

                self.pc += 1;
                let addr_hi = self.bus.read_byte(self.pbr_pc());

                self.pc += 1;
                let addr_bank = self.bus.read_dword(self.pbr_pc());

                let addr = (Self::make_word(addr_lo, addr_hi) + self.reg_x) as u32;

                let data_lo = self.bus.read_byte((addr_bank << 16) | addr);
                let data_hi = self.bus.read_byte((addr_bank << 16) | (addr + 1));

                Self::make_word(data_lo, data_hi)
            }

            AddressMode::DirectIndirect => {
                self.pc += 1;
                let direct_offset = self.bus.read_dword(self.pbr_pc());

                let addr_lo = self.bus.read_byte(self.reg_d as u32 + direct_offset);
                let addr_hi = self.bus.read_byte(self.reg_d as u32 + direct_offset + 1);
                let addr = Self::make_word(addr_lo, addr_hi) as u32;

                let data_lo = self.bus.read_byte(((self.reg_db as u32) << 16) | addr);
                let data_hi = self
                    .bus
                    .read_byte(((self.reg_db as u32) << 16) | (addr + 1));

                Self::make_word(data_lo, data_hi)
            }

            AddressMode::DirectIndirectLong => {
                self.pc += 1;
                let direct_offset = self.bus.read_dword(self.pbr_pc());

                let addr_lo = self.bus.read_byte(self.reg_d as u32 + direct_offset);
                let addr_hi = self.bus.read_byte(self.reg_d as u32 + direct_offset + 1);
                let addr_bank = self.bus.read_dword(self.reg_d as u32 + direct_offset + 2);

                let addr = Self::make_word(addr_lo, addr_hi) as u32;

                let data_lo = self.bus.read_byte((addr_bank << 16) | addr);
                let data_hi = self.bus.read_byte((addr_bank << 16) | (addr + 1));

                Self::make_word(data_lo, data_hi)
            }

            AddressMode::ZeroPageDirectIndirectIndexedLong => {
                self.pc += 1;
                let direct_offset = self.bus.read_dword(self.pbr_pc());

                let addr_lo = self.bus.read_byte(self.reg_d as u32 + direct_offset);
                let addr_hi = self.bus.read_byte(self.reg_d as u32 + direct_offset + 1);
                let addr_bank = self.bus.read_dword(self.reg_d as u32 + direct_offset + 2);

                let addr = (Self::make_word(addr_lo, addr_hi) + self.reg_y) as u32;

                let data_lo = self.bus.read_byte((addr_bank << 16) | addr);
                let data_hi = self.bus.read_byte((addr_bank << 16) | (addr + 1));

                Self::make_word(data_lo, data_hi)
            }

            AddressMode::StackRelative => {
                self.pc += 1;
                let stack_offset = self.bus.read_dword(self.pbr_pc());

                let data_lo = self.bus.read_byte(self.sp + stack_offset);
                let data_hi = self.bus.read_byte(self.sp + stack_offset + 1);

                Self::make_word(data_lo, data_hi)
            }

            AddressMode::StackRelativeIndirectIndexedY => {
                self.pc += 1;
                let stack_offset = self.bus.read_dword(self.pbr_pc());

                let addr_lo = self.bus.read_byte(self.sp + stack_offset);
                let addr_hi = self.bus.read_byte(self.sp + stack_offset + 1);

                let addr = (Self::make_word(addr_lo, addr_hi) + self.reg_y) as u32;

                let data_lo = self.bus.read_byte(addr);
                let data_hi = self.bus.read_byte(addr + 1);

                Self::make_word(data_lo, data_hi)
            }
        }
    }

    fn store(&mut self, mode: AddressMode, value: &Word) {
        match mode {
            AddressMode::Absolute => {
                self.pc += 1;
                let addr_lo = self.bus.read_byte(self.pbr_pc());

                self.pc += 1;
                let addr_hi = self.bus.read_byte(self.pbr_pc());

                let addr = Self::make_word(addr_lo, addr_hi) as u32;

                // data_lo
                self.bus
                    .write_byte(((self.reg_db as u32) << 16) | addr, value.lo());

                // data_hi
                self.bus
                    .write_byte(((self.reg_db as u32) << 16) | addr + 1, value.hi());
            }

            AddressMode::AbsoluteIndexedX => {
                self.pc += 1;
                let addr_lo = self.bus.read_byte(self.pbr_pc());

                self.pc += 1;
                let addr_hi = self.bus.read_byte(self.pbr_pc());

                let addr = (Self::make_word(addr_lo, addr_hi) + self.reg_x) as u32;

                // data_lo
                self.bus
                    .write_byte(((self.reg_db as u32) << 16) | addr, value.lo());

                // data_hi
                self.bus
                    .write_byte(((self.reg_db as u32) << 16) | (addr + 1), value.hi());
            }

            AddressMode::AbsoluteIndexedY => {
                self.pc += 1;
                let addr_lo = self.bus.read_byte(self.pbr_pc());

                self.pc += 1;
                let addr_hi = self.bus.read_byte(self.pbr_pc());

                let addr = (Self::make_word(addr_lo, addr_hi) + self.reg_y) as u32;

                // data_lo
                self.bus
                    .write_byte(((self.reg_db as u32) << 16) | addr, value.lo());

                // data_hi
                self.bus
                    .write_byte(((self.reg_db as u32) << 16) | (addr + 1), value.hi());
            }

            AddressMode::Immediate => {
                // data_lo
                self.pc += 1;
                self.bus.write_byte(self.pbr_pc(), value.lo());

                // data_hi
                self.pc += 1;
                self.bus.write_byte(self.pbr_pc(), value.hi());
            }

            AddressMode::ZeroPage => {
                self.pc += 1;
                let direct_offset = self.bus.read_byte(self.pbr_pc()) as u32;

                // data_lo
                self.bus
                    .write_byte(self.reg_d as u32 + direct_offset, value.lo());

                // data_hi
                self.bus
                    .write_byte(self.reg_d as u32 + direct_offset + 1, value.hi());
            }

            AddressMode::ZeroPageX => {
                self.pc += 1;
                let direct_offset = self.bus.read_byte(self.pbr_pc()) as u32;

                // data_lo
                self.bus
                    .write_byte((self.reg_d + self.reg_x) as u32 + direct_offset, value.lo());

                // data_hi
                self.bus.write_byte(
                    (self.reg_d + self.reg_x) as u32 + direct_offset + 1,
                    value.hi(),
                );
            }

            AddressMode::ZeroPageY => {
                self.pc += 1;
                let direct_offset = self.bus.read_byte(self.pbr_pc()) as u32;

                // data_lo
                self.bus
                    .write_byte((self.reg_d + self.reg_y) as u32 + direct_offset, value.lo());

                // data_hi
                self.bus.write_byte(
                    (self.reg_d + self.reg_y) as u32 + direct_offset + 1,
                    value.hi(),
                );
            }

            AddressMode::ZeroPageDirectIndirectIndexedY => {
                self.pc += 1;
                let direct_offset = self.bus.read_dword(self.pbr_pc());

                let addr_lo = self.bus.read_byte(self.reg_d as u32 + direct_offset);
                let addr_hi = self.bus.read_byte(self.reg_d as u32 + direct_offset + 1);
                let addr = (Self::make_word(addr_lo, addr_hi) + self.reg_y) as u32;

                // data_lo
                self.bus
                    .write_byte(((self.reg_db as u32) << 16) | addr, value.lo());

                // data_hi
                self.bus
                    .write_byte(((self.reg_db as u32) << 16) | (addr + 1), value.hi());
            }

            AddressMode::ZeroPageDirectIndexedIndirectX => {
                self.pc += 1;
                let direct_offset = self.bus.read_dword(self.pbr_pc());

                let addr_lo = self
                    .bus
                    .read_byte((self.reg_d + self.reg_x) as u32 + direct_offset);
                let addr_hi = self
                    .bus
                    .read_byte((self.reg_d + self.reg_x) as u32 + direct_offset + 1);

                let addr = Self::make_word(addr_lo, addr_hi) as u32;

                // data_lo
                self.bus
                    .write_byte(((self.reg_db as u32) << 16) | addr, value.lo());

                // data_hi
                self.bus
                    .write_byte(((self.reg_db as u32) << 16) | (addr + 1), value.hi());
            }

            AddressMode::AbsoluteLong => {
                self.pc += 1;
                let addr_lo = self.bus.read_byte(self.pbr_pc());

                self.pc += 1;
                let addr_hi = self.bus.read_byte(self.pbr_pc());

                self.pc += 1;
                let addr_bank = self.bus.read_dword(self.pbr_pc());

                let addr = Self::make_word(addr_lo, addr_hi) as u32;

                // data_lo
                self.bus.write_byte((addr_bank << 16) | addr, value.lo());

                // data_hi
                self.bus
                    .write_byte((addr_bank << 16) | (addr + 1), value.hi());
            }

            AddressMode::AbsoluteLongIndexedX => {
                self.pc += 1;
                let addr_lo = self.bus.read_byte(self.pbr_pc());

                self.pc += 1;
                let addr_hi = self.bus.read_byte(self.pbr_pc());

                self.pc += 1;
                let addr_bank = self.bus.read_dword(self.pbr_pc());

                let addr = (Self::make_word(addr_lo, addr_hi) + self.reg_x) as u32;

                // data_lo
                self.bus.write_byte((addr_bank << 16) | addr, value.lo());

                // data_hi
                self.bus
                    .write_byte((addr_bank << 16) | (addr + 1), value.hi());
            }

            AddressMode::DirectIndirect => {
                self.pc += 1;
                let direct_offset = self.bus.read_dword(self.pbr_pc());

                let addr_lo = self.bus.read_byte(self.reg_d as u32 + direct_offset);
                let addr_hi = self.bus.read_byte(self.reg_d as u32 + direct_offset + 1);
                let addr = Self::make_word(addr_lo, addr_hi) as u32;

                // data_lo
                self.bus
                    .write_byte(((self.reg_db as u32) << 16) | addr, value.lo());

                // data_hi
                self.bus
                    .write_byte(((self.reg_db as u32) << 16) | (addr + 1), value.hi());
            }

            AddressMode::DirectIndirectLong => {
                self.pc += 1;
                let direct_offset = self.bus.read_dword(self.pbr_pc());

                let addr_lo = self.bus.read_byte(self.reg_d as u32 + direct_offset);
                let addr_hi = self.bus.read_byte(self.reg_d as u32 + direct_offset + 1);
                let addr_bank = self.bus.read_dword(self.reg_d as u32 + direct_offset + 2);

                let addr = Self::make_word(addr_lo, addr_hi) as u32;

                // data_lo
                self.bus.write_byte((addr_bank << 16) | addr, value.lo());

                // data_hi
                self.bus
                    .write_byte((addr_bank << 16) | (addr + 1), value.hi());
            }

            AddressMode::ZeroPageDirectIndirectIndexedLong => {
                self.pc += 1;
                let direct_offset = self.bus.read_dword(self.pbr_pc());

                let addr_lo = self.bus.read_byte(self.reg_d as u32 + direct_offset);
                let addr_hi = self.bus.read_byte(self.reg_d as u32 + direct_offset + 1);
                let addr_bank = self.bus.read_dword(self.reg_d as u32 + direct_offset + 2);

                let addr = (Self::make_word(addr_lo, addr_hi) + self.reg_y) as u32;

                // data_lo
                self.bus.write_byte((addr_bank << 16) | addr, value.lo());

                // data_hi
                self.bus
                    .write_byte((addr_bank << 16) | (addr + 1), value.hi());
            }

            AddressMode::StackRelative => {
                self.pc += 1;
                let stack_offset = self.bus.read_dword(self.pbr_pc());

                // data_lo
                self.bus.write_byte(self.sp + stack_offset, value.lo());

                // data_hi
                self.bus.write_byte(self.sp + stack_offset + 1, value.hi());
            }

            AddressMode::StackRelativeIndirectIndexedY => {
                self.pc += 1;
                let stack_offset = self.bus.read_dword(self.pbr_pc());

                let addr_lo = self.bus.read_byte(self.sp + stack_offset);
                let addr_hi = self.bus.read_byte(self.sp + stack_offset + 1);

                let addr = (Self::make_word(addr_lo, addr_hi) + self.reg_y) as u32;

                // data_lo
                self.bus.write_byte(addr, value.lo());

                // data_hi
                self.bus.write_byte(addr + 1, value.hi());
            }
        }
    }

    fn flag_nz(&mut self, value: u16) {
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

    fn flag_c(&mut self, set: bool) {
        if set {
            self.reg_p |= S_CARRY;
        } else {
            self.reg_p &= !(S_CARRY);
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
                        true => self.pc - ((!offset) + 1) as u16,
                        false => self.pc + offset as u16,
                    };
                }
            }
            Err(msg) => panic!("error : {}", msg),
        }
    }

    fn op_lda(&mut self, opcode: u8) {
        self.reg_a.data = match opcode {
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
            _ => panic!("invalid opcode {}", opcode),
        };

        self.flag_nz(self.reg_a.data);
    }

    fn op_sta(&mut self, opcode: u8) {
        let value = &self.reg_a.clone();
        match opcode {
            0x85 => self.store(AddressMode::ZeroPage, value),
            0x95 => self.store(AddressMode::ZeroPageX, value),
            0x8D => self.store(AddressMode::Absolute, value),
            0x9D => self.store(AddressMode::AbsoluteIndexedX, value),
            0x99 => self.store(AddressMode::AbsoluteIndexedY, value),
            0x81 => self.store(AddressMode::ZeroPageDirectIndexedIndirectX, value),
            0x91 => self.store(AddressMode::ZeroPageDirectIndirectIndexedY, value),
            0x8F => self.store(AddressMode::AbsoluteLong, value),
            0x9F => self.store(AddressMode::AbsoluteLongIndexedX, value),
            0x92 => self.store(AddressMode::DirectIndirect, value),
            0x87 => self.store(AddressMode::DirectIndirectLong, value),
            0x97 => self.store(AddressMode::ZeroPageDirectIndirectIndexedLong, value),
            0x83 => self.store(AddressMode::StackRelative, value),
            0x93 => self.store(AddressMode::StackRelativeIndirectIndexedY, value),
            _ => panic!("invalid opcode {}", opcode),
        };
    }

    fn op_ldy(&mut self, opcode: u8) {
        self.reg_y = match opcode {
            0xA0 => self.fetch(AddressMode::Immediate),
            0xA4 => self.fetch(AddressMode::ZeroPage),
            0xB4 => self.fetch(AddressMode::ZeroPageX),
            0xAC => self.fetch(AddressMode::Absolute),
            0xBC => self.fetch(AddressMode::AbsoluteIndexedX),
            _ => panic!("invalid opcode {}", opcode),
        };

        self.flag_nz(self.reg_y);
    }

    fn op_ldx(&mut self, opcode: u8) {
        self.reg_x = match opcode {
            0xA2 => self.fetch(AddressMode::Immediate),
            0xA6 => self.fetch(AddressMode::ZeroPage),
            0xB6 => self.fetch(AddressMode::ZeroPageY),
            0xAE => self.fetch(AddressMode::Absolute),
            0xBE => self.fetch(AddressMode::AbsoluteIndexedY),
            _ => panic!("invalid opcode {}", opcode),
        };

        self.flag_nz(self.reg_x);
    }

    fn op_jsl(&mut self) {
        // New PC Low
        self.pc += 1;
        let pcl = self.bus.read_byte(self.pbr_pc()) as u16;

        // New PC High
        self.pc += 1;
        let pch = self.bus.read_byte(self.pbr_pc()) as u16;

        // push PBR
        self.bus.write_byte(self.sp, self.reg_pb);
        self.sp -= 1;

        // New PBR
        self.pc += 1;
        let pbr = self.bus.read_byte(self.pbr_pc());

        // push PC High
        self.bus
            .write_byte(self.sp, ((self.pc & 0xFF00) >> 8) as u8);
        self.sp -= 1;

        // push PC Low
        self.bus.write_byte(self.sp, (self.pc & 0xFF) as u8);
        self.sp -= 1;

        // Save new PBR (bank)
        self.reg_pb = pbr;

        // Save new PC
        self.pc = (pch << 8) | pcl;
    }

    fn op_jsr(&mut self) {
        // New PC Low
        self.pc += 1;
        let pcl = self.bus.read_byte(self.pbr_pc());

        // New PC High
        self.pc += 1;
        let pch = self.bus.read_byte(self.pbr_pc());

        // push PC High
        self.bus
            .write_byte(self.sp, ((self.pc & 0xFF00) >> 8) as u8);
        self.sp -= 1;

        // push PC Low
        self.bus.write_byte(self.sp, (self.pc & 0xFF) as u8);
        self.sp -= 1;

        // Save new PC
        self.pc = Self::make_word(pcl, pch);
    }

    fn op_rep(&mut self) {
        let mut mask = (self.fetch(AddressMode::Immediate) & 0xFF) as u8;
        if self.emulation {
            mask &= 0xCF;
        }
        self.reg_p = self.reg_p & (!mask);
    }

    fn op_sep(&mut self) {
        let mut mask = (self.fetch(AddressMode::Immediate) & 0xFF) as u8;
        if self.emulation {
            mask &= 0xCF;
        }
        self.reg_p = self.reg_p | mask;
    }

    fn op_cpx(&mut self, opcode: u8) {
        let operand = match opcode {
            0xE0 => self.fetch(AddressMode::Immediate),
            0xE4 => self.fetch(AddressMode::ZeroPage),
            0xEC => self.fetch(AddressMode::Absolute),
            _ => panic!("invalid opcode {}", opcode),
        };

        // negative and zero
        self.flag_nz(self.reg_x - operand);

        // carry is clear when borrow is required; that is, if the register is less than the operand
        self.flag_c(self.reg_x >= operand);
    }

    fn op_cpy(&mut self, opcode: u8) {
        let operand = match opcode {
            0xC0 => self.fetch(AddressMode::Immediate),
            0xC4 => self.fetch(AddressMode::ZeroPage),
            0xCC => self.fetch(AddressMode::Absolute),
            _ => panic!("invalid opcode {}", opcode),
        };

        // negative and zero
        self.flag_nz(self.reg_y - operand);

        // carry is clear when borrow is required; that is, if the register is less than the operand
        self.flag_c(self.reg_y >= operand);
    }

    fn op_cmp(&mut self, opcode: u8) {
        let operand = match opcode {
            0xC9 => self.fetch(AddressMode::Immediate),
            0xC5 => self.fetch(AddressMode::ZeroPage),
            0xD5 => self.fetch(AddressMode::ZeroPageX),
            0xCD => self.fetch(AddressMode::Absolute),
            0xDD => self.fetch(AddressMode::AbsoluteIndexedX),
            0xD9 => self.fetch(AddressMode::AbsoluteIndexedY),
            0xC1 => self.fetch(AddressMode::ZeroPageDirectIndirectIndexedY),
            0xD1 => self.fetch(AddressMode::ZeroPageDirectIndirectIndexedY),
            0xCF => self.fetch(AddressMode::AbsoluteLong),
            0xDF => self.fetch(AddressMode::AbsoluteLongIndexedX),
            0xD2 => self.fetch(AddressMode::DirectIndirect),
            0xC7 => self.fetch(AddressMode::DirectIndirectLong),
            0xD7 => self.fetch(AddressMode::ZeroPageDirectIndirectIndexedLong),
            0xC3 => self.fetch(AddressMode::StackRelative),
            0xD3 => self.fetch(AddressMode::StackRelativeIndirectIndexedY),
            _ => panic!("invalid opcode {}", opcode),
        };

        // negative and zero
        self.flag_nz(self.reg_a.data - operand);

        // carry is clear when borrow is required; that is, if the register is less than the operand
        self.flag_c(self.reg_a.data >= operand);
    }

    fn op_jml(&mut self) {
        self.pc += 1;
        let addr_lo = self.bus.read_byte(self.pbr_pc());

        self.pc += 1;
        let addr_hi = self.bus.read_byte(self.pbr_pc());

        let addr = Self::make_word(addr_lo, addr_hi) as u32;

        let pcl = self.bus.read_byte(addr) as u16;
        let pch = self.bus.read_byte(addr + 1) as u16;
        let pbr = self.bus.read_byte(addr + 2);

        // Save new PBR (bank)
        self.reg_pb = pbr;

        // Save new PC
        self.pc = (pch << 8) | pcl;
    }

    fn op_pha(&mut self) {
        self.pc += 1;
        self.bus.write_byte(self.sp, self.reg_a.hi());
        self.sp -= 1;
        self.bus.write_byte(self.sp, self.reg_a.lo());
        self.sp -= 1;
    }

    fn op_phb(&mut self) {
        self.pc += 1;
        self.bus.write_byte(self.sp, self.reg_db);
        self.sp -= 1;
    }

    fn op_phd(&mut self) {
        let value = Word { data: self.reg_d };
        self.pc += 1;
        self.bus.write_byte(self.sp, value.hi());
        self.sp -= 1;
        self.bus.write_byte(self.sp, value.lo());
        self.sp -= 1;
    }

    fn op_phk(&mut self) {
        self.pc += 1;
        self.bus.write_byte(self.sp, self.reg_pb);
        self.sp -= 1;
    }

    fn op_php(&mut self) {
        self.pc += 1;
        self.bus.write_byte(self.sp, self.reg_p);
        self.sp -= 1;
    }

    fn op_phx(&mut self) {
        let value = Word { data: self.reg_x };
        self.pc += 1;
        self.bus.write_byte(self.sp, value.hi());
        self.sp -= 1;
        self.bus.write_byte(self.sp, value.lo());
        self.sp -= 1;
    }

    fn op_phy(&mut self) {
        let value = Word { data: self.reg_y };
        self.pc += 1;
        self.bus.write_byte(self.sp, value.hi());
        self.sp -= 1;
        self.bus.write_byte(self.sp, value.lo());
        self.sp -= 1;
    }
}
