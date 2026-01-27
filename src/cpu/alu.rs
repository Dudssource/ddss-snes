use crate::cpu::{bits::Word, bus::Bus};

pub const S_CARRY: u8 = 0x1;
pub const S_RESULT_ZERO: u8 = 0x1 << 1;
pub const S_IRQ_DISABLE: u8 = 0x1 << 2;
pub const S_DECIMAL_MODE: u8 = 0x1 << 3;
pub const S_BREAK_INSTRUCTION: u8 = 0x1 << 4;
pub const S_OVERFLOW: u8 = 0x1 << 6;
pub const S_NEGATIVE: u8 = 0x1 << 7;

pub struct Cpu {
    pub bus: Box<Bus>,
    pub reg_a: Word,
    pub reg_x: u16,
    pub reg_y: u16,
    pub reg_p: u8,
    pub reg_d: u16,
    pub reg_pb: u8,
    pub reg_db: u8,
    pub sp: u32,
    pub pc: u16,
    pub emulation: bool,
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

    pub fn pbr_pc(&self) -> u32 {
        ((self.reg_pb as u32) << 16) | self.pc as u32
    }

    pub fn make_word(lo: u8, hi: u8) -> u16 {
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
            0x10 | 0x30 | 0x50 | 0x70 | 0x90 | 0xB0 | 0xD0 | 0xF0 | 0x80 => self.op_branch(opcode),

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

            // ADC Add memory to accumulator with carry
            0x69 | 0x65 | 0x75 | 0x6D | 0x7D | 0x79 | 0x61 | 0x71 | 0x6F | 0x7F | 0x72 | 0x67
            | 0x77 | 0x63 | 0x73 => self.op_adc(opcode),

            // SBC Subtract memory from accumulator with borrow
            0xE9 | 0xE5 | 0xF5 | 0xED | 0xFD | 0xF9 | 0xE1 | 0xF1 | 0xEF | 0xFF | 0xF2 | 0xE7
            | 0xF7 | 0xE3 | 0xF3 => self.op_sbc(opcode),

            // CLC Clear carry flag
            0x18 => self.op_clc(),

            // SEC Set carry flag
            0x38 => self.op_sec(),

            // CLD Clear decimal mode
            0xD8 => self.op_cld(),

            // SED Set decimal mode
            0xF8 => self.op_sed(),

            // CLI Clear interrupt disable status
            0x58 => self.op_cli(),

            // SEI Set interrupt disable status
            0x78 => self.op_sei(),

            // CLV Clear overflow flag
            0xB8 => self.op_clv(),

            // ASL Shift Left One Bit (Memory or Accumulator)
            0x0A | 0x06 | 0x16 | 0x0E | 0x1E => self.op_asl(opcode),

            // AND memory with accumulator
            0x29 | 0x25 | 0x35 | 0x2D | 0x3D | 0x39 | 0x21 | 0x31 | 0x2F | 0x3F | 0x32 | 0x27
            | 0x37 | 0x23 | 0x33 => self.op_and(opcode),

            // BIT Test bits in memory with accumulator
            0x24 | 0x2C | 0x89 | 0x34 | 0x3C => self.op_bit(opcode),

            // DEC Decrement memory by one
            0xC6 | 0xD6 | 0xCE | 0xDE | 0x3A => self.op_dec(opcode),

            // INC Increment memory by one
            0xE6 | 0xF6 | 0xEE | 0xFE | 0x1A => self.op_inc(opcode),

            // ERROR
            _ => panic!("invalid opcode {}", opcode),
        }
    }

    pub fn fetch(&mut self, mode: AddressMode) -> u16 {
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

    pub fn store(&mut self, mode: AddressMode, value: &Word) {
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

    pub fn flag_nz(&mut self, value: u16) {
        // zero
        self.flag(S_RESULT_ZERO, value == 0);

        // 8 or 16 bit (6502 emulation on/off)
        self.flag(S_NEGATIVE, Word::is_signed(&self.emulation, &value));
    }

    pub fn check_overflow(&mut self, value: i32) {
        let mut final_carry = value > 0xFFFF;
        let penultimate_carry = value > 0x7FFF;
        if value < 0 {
            // borrow clears the flag
            self.flag_c(false);
            final_carry = false;
        } else {
            self.flag_c(final_carry);
        }
        self.flag_v(final_carry ^ penultimate_carry);
    }

    pub fn flag_v(&mut self, set: bool) {
        self.flag(S_OVERFLOW, set);
    }

    pub fn flag_c(&mut self, set: bool) {
        self.flag(S_CARRY, set);
    }

    pub fn flag(&mut self, flag: u8, set: bool) {
        if set {
            self.reg_p |= flag;
        } else {
            self.reg_p &= !(flag);
        }
    }
}
