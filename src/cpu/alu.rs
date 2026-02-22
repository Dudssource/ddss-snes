use crate::cpu::{bits::Word, bus::Bus};
use log::debug;

pub const S_CARRY: u8 = 0x1;
pub const S_RESULT_ZERO: u8 = 0x1 << 1;
pub const S_IRQ_DISABLE: u8 = 0x1 << 2;
pub const S_DECIMAL_MODE: u8 = 0x1 << 3;
pub const S_BREAK_INSTRUCTION: u8 = 0x1 << 4; // accumulator 8 or 16 bits
pub const S_INDEX_REGISTERS: u8 = S_BREAK_INSTRUCTION; // x-y index registers 8 or 16 bits
pub const S_ACCUMULATOR_MEMORY: u8 = 0x1 << 5;
pub const S_OVERFLOW: u8 = 0x1 << 6;
pub const S_NEGATIVE: u8 = 0x1 << 7;

pub const STACK_POINTER_START: u32 = 0x1FF;

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
            sp: STACK_POINTER_START,
            reg_a: Word::new(0, 0),
            reg_x: 0x0,
            reg_y: 0x0,
            reg_p: 0x0,
            reg_d: 0x0,
            pc: 0x8000,
            reg_pb: 0x0,
            reg_db: 0x0,
            emulation: true,
        }
    }

    pub fn start(&mut self) {
        loop {
            let opcode = self.bus.read_byte(self.pc as u32);
            self.decode_and_execute(opcode);
            self.incr_pc();
        }
    }

    pub fn incr_pc(&mut self) {
        // if the program counter increments past $FFFF, it rolls over to $0000
        // without incrementing the program counter bank
        if self.pc == 0xFFFF {
            self.pc = 0x0;
        } else {
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
            0x20 => self.op_jsr(opcode),

            // JSL Jump Subroutine Long
            0x22 => self.op_jsl(opcode),

            // REP Reset Status Bits
            0xC2 => self.op_rep(opcode),

            // JML Jump Long
            0xDC => self.op_jml(opcode),

            // JMP Jump to new location
            0x4C | 0x6C | 0x7C | 0x5C => self.op_jmp(opcode),

            // SEP Set Processor Status Bits
            0xE2 => self.op_sep(opcode),

            // PHA Push accumulator on stack
            0x48 => self.op_pha(opcode),

            // PHB Push Data Bank Register on Stack
            0x8B => self.op_phb(opcode),

            // PHD Push Direct Register on Stack
            0x0B => self.op_phd(opcode),

            // PHK Push Program Bank Register on Stack
            0x4B => self.op_phk(opcode),

            // PHP Push processor status on stack
            0x08 => self.op_php(opcode),

            // PHX Push Index X on Stack
            0xDA => self.op_phx(opcode),

            // PHY Push Index Y on Stack
            0x5A => self.op_phy(opcode),

            // ADC Add memory to accumulator with carry
            0x69 | 0x65 | 0x75 | 0x6D | 0x7D | 0x79 | 0x61 | 0x71 | 0x6F | 0x7F | 0x72 | 0x67
            | 0x77 | 0x63 | 0x73 => self.op_adc(opcode),

            // SBC Subtract memory from accumulator with borrow
            0xE9 | 0xE5 | 0xF5 | 0xED | 0xFD | 0xF9 | 0xE1 | 0xF1 | 0xEF | 0xFF | 0xF2 | 0xE7
            | 0xF7 | 0xE3 | 0xF3 => self.op_sbc(opcode),

            // CLC Clear carry flag
            0x18 => self.op_clc(opcode),

            // SEC Set carry flag
            0x38 => self.op_sec(opcode),

            // CLD Clear decimal mode
            0xD8 => self.op_cld(opcode),

            // SED Set decimal mode
            0xF8 => self.op_sed(opcode),

            // CLI Clear interrupt disable status
            0x58 => self.op_cli(opcode),

            // SEI Set interrupt disable status
            0x78 => self.op_sei(opcode),

            // CLV Clear overflow flag
            0xB8 => self.op_clv(opcode),

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

            // DEX Decrement index X by one
            0xCA => self.op_dex(opcode),

            // INX Increment Index X by one
            0xE8 => self.op_inx(opcode),

            // DEY Decrement index Y by one
            0x88 => self.op_dey(opcode),

            // INY Increment Index Y by one
            0xC8 => self.op_iny(opcode),

            // EOR "Exclusive-Or" memory with accumulator
            0x49 | 0x45 | 0x55 | 0x4D | 0x5D | 0x59 | 0x41 | 0x51 | 0x4F | 0x5F | 0x52 | 0x47
            | 0x57 | 0x43 | 0x53 => self.op_eor(opcode),

            // LSR Shift right one bit (memory or accumulator)
            0x4A | 0x46 | 0x56 | 0x4E | 0x5E => self.op_lsr(opcode),

            //  XCE Exchange Carry and Emulation Bits
            0xFB => self.op_xce(opcode),

            // TXS Transfer index X to stack pointer
            0x9A => self.op_txs(opcode),

            // STZ Store zero in memory
            0x9C | 0x9E | 0x64 | 0x74 => self.op_stz(opcode),

            // RTS Return from subroutine
            0x60 => self.op_rts(opcode),

            // STY Store index Y in memory
            0x84 | 0x94 | 0x8C => self.op_sty(opcode),

            // STX Store index X in memory
            0x86 | 0x96 | 0x8E => self.op_stx(opcode),

            // ERROR
            _ => panic!("invalid opcode 0x{:X} at 0x{:X}", opcode, self.pc),
        }
    }

    pub fn fetch(&mut self, mode: AddressMode, sixteen_bits_mode: bool) -> u16 {
        debug!("fetch {:?}", mode);

        match mode {
            AddressMode::Absolute => {
                self.incr_pc();
                let addr_lo = self.bus.read_byte(self.pbr_pc());

                self.incr_pc();
                let addr_hi = self.bus.read_byte(self.pbr_pc());

                let addr = Self::make_word(addr_lo, addr_hi) as u32;

                let data_lo = self.bus.read_byte(((self.reg_db as u32) << 16) | addr);
                let mut data_hi = 0u8;

                if sixteen_bits_mode {
                    data_hi = self.bus.read_byte(((self.reg_db as u32) << 16) | addr + 1);
                }

                Self::make_word(data_lo, data_hi)
            }

            AddressMode::AbsoluteIndexedX => {
                self.incr_pc();
                let addr_lo = self.bus.read_byte(self.pbr_pc());

                self.incr_pc();
                let addr_hi = self.bus.read_byte(self.pbr_pc());

                let addr = (Self::make_word(addr_lo, addr_hi) + self.reg_x) as u32;

                let data_lo = self.bus.read_byte(((self.reg_db as u32) << 16) | addr);
                let mut data_hi = 0u8;

                if sixteen_bits_mode {
                    data_hi = self
                        .bus
                        .read_byte(((self.reg_db as u32) << 16) | (addr + 1));
                }

                Self::make_word(data_lo, data_hi)
            }

            AddressMode::AbsoluteIndexedY => {
                self.incr_pc();
                let addr_lo = self.bus.read_byte(self.pbr_pc());

                self.incr_pc();
                let addr_hi = self.bus.read_byte(self.pbr_pc());

                let addr = (Self::make_word(addr_lo, addr_hi) + self.reg_y) as u32;

                let data_lo = self.bus.read_byte(((self.reg_db as u32) << 16) | addr);
                let mut data_hi = 0u8;

                if sixteen_bits_mode {
                    data_hi = self
                        .bus
                        .read_byte(((self.reg_db as u32) << 16) | (addr + 1));
                }

                Self::make_word(data_lo, data_hi)
            }

            AddressMode::Immediate => {
                self.incr_pc();
                let data_lo = self.bus.read_byte(self.pbr_pc());
                let mut data_hi = 0u8;

                if sixteen_bits_mode {
                    self.incr_pc();
                    data_hi = self.bus.read_byte(self.pbr_pc());
                }

                debug!("[0x{:X}] fetch {:?}", self.pc, mode);

                Self::make_word(data_lo, data_hi)
            }

            AddressMode::ZeroPage => {
                self.incr_pc();
                let direct_offset = self.bus.read_byte(self.pbr_pc()) as u32;

                let data_lo = self.bus.read_byte(self.reg_d as u32 + direct_offset);
                let mut data_hi = 0u8;

                if sixteen_bits_mode {
                    data_hi = self.bus.read_byte(self.reg_d as u32 + direct_offset + 1);
                }

                Self::make_word(data_lo, data_hi)
            }

            AddressMode::ZeroPageX => {
                self.incr_pc();
                let direct_offset = self.bus.read_byte(self.pbr_pc()) as u32;

                let data_lo = self
                    .bus
                    .read_byte((self.reg_d + self.reg_x) as u32 + direct_offset);
                let mut data_hi = 0u8;

                if sixteen_bits_mode {
                    data_hi = self
                        .bus
                        .read_byte((self.reg_d + self.reg_x) as u32 + direct_offset + 1);
                }

                Self::make_word(data_lo, data_hi)
            }

            AddressMode::ZeroPageY => {
                self.incr_pc();
                let direct_offset = self.bus.read_byte(self.pbr_pc()) as u32;

                let data_lo = self
                    .bus
                    .read_byte((self.reg_d + self.reg_y) as u32 + direct_offset);
                let mut data_hi = 0u8;

                if sixteen_bits_mode {
                    data_hi = self
                        .bus
                        .read_byte((self.reg_d + self.reg_y) as u32 + direct_offset + 1);
                }

                Self::make_word(data_lo, data_hi)
            }

            AddressMode::ZeroPageDirectIndirectIndexedY => {
                self.incr_pc();
                let direct_offset = self.bus.read_dword(self.pbr_pc());

                let addr_lo = self.bus.read_byte(self.reg_d as u32 + direct_offset);
                let addr_hi = self.bus.read_byte(self.reg_d as u32 + direct_offset + 1);
                let addr = (Self::make_word(addr_lo, addr_hi) + self.reg_y) as u32;

                let data_lo = self.bus.read_byte(((self.reg_db as u32) << 16) | addr);
                let mut data_hi = 0u8;

                if sixteen_bits_mode {
                    data_hi = self
                        .bus
                        .read_byte(((self.reg_db as u32) << 16) | (addr + 1));
                }

                Self::make_word(data_lo, data_hi)
            }

            AddressMode::ZeroPageDirectIndexedIndirectX => {
                self.incr_pc();
                let direct_offset = self.bus.read_dword(self.pbr_pc());

                let addr_lo = self
                    .bus
                    .read_byte((self.reg_d + self.reg_x) as u32 + direct_offset);
                let addr_hi = self
                    .bus
                    .read_byte((self.reg_d + self.reg_x) as u32 + direct_offset + 1);

                let addr = Self::make_word(addr_lo, addr_hi) as u32;

                let data_lo = self.bus.read_byte(((self.reg_db as u32) << 16) | addr);
                let mut data_hi = 0u8;

                if sixteen_bits_mode {
                    data_hi = self
                        .bus
                        .read_byte(((self.reg_db as u32) << 16) | (addr + 1));
                }

                Self::make_word(data_lo, data_hi)
            }

            AddressMode::AbsoluteLong => {
                self.incr_pc();
                let addr_lo = self.bus.read_byte(self.pbr_pc());

                self.incr_pc();
                let addr_hi = self.bus.read_byte(self.pbr_pc());

                self.incr_pc();
                let addr_bank = self.bus.read_dword(self.pbr_pc());

                let addr = Self::make_word(addr_lo, addr_hi) as u32;

                let data_lo = self.bus.read_byte((addr_bank << 16) | addr);
                let mut data_hi = 0u8;

                if sixteen_bits_mode {
                    data_hi = self.bus.read_byte((addr_bank << 16) | (addr + 1));
                }

                Self::make_word(data_lo, data_hi)
            }

            AddressMode::AbsoluteLongIndexedX => {
                self.incr_pc();
                let addr_lo = self.bus.read_byte(self.pbr_pc());

                self.incr_pc();
                let addr_hi = self.bus.read_byte(self.pbr_pc());

                self.incr_pc();
                let addr_bank = self.bus.read_dword(self.pbr_pc());

                let addr = (Self::make_word(addr_lo, addr_hi) + self.reg_x) as u32;

                let data_lo = self.bus.read_byte((addr_bank << 16) | addr);
                let mut data_hi = 0u8;

                if sixteen_bits_mode {
                    data_hi = self.bus.read_byte((addr_bank << 16) | (addr + 1));
                }

                Self::make_word(data_lo, data_hi)
            }

            AddressMode::DirectIndirect => {
                self.incr_pc();
                let direct_offset = self.bus.read_dword(self.pbr_pc());

                let addr_lo = self.bus.read_byte(self.reg_d as u32 + direct_offset);
                let addr_hi = self.bus.read_byte(self.reg_d as u32 + direct_offset + 1);
                let addr = Self::make_word(addr_lo, addr_hi) as u32;

                let data_lo = self.bus.read_byte(((self.reg_db as u32) << 16) | addr);
                let mut data_hi = 0u8;

                if sixteen_bits_mode {
                    data_hi = self
                        .bus
                        .read_byte(((self.reg_db as u32) << 16) | (addr + 1));
                }

                Self::make_word(data_lo, data_hi)
            }

            AddressMode::DirectIndirectLong => {
                self.incr_pc();
                let direct_offset = self.bus.read_dword(self.pbr_pc());

                let addr_lo = self.bus.read_byte(self.reg_d as u32 + direct_offset);
                let addr_hi = self.bus.read_byte(self.reg_d as u32 + direct_offset + 1);
                let addr_bank = self.bus.read_dword(self.reg_d as u32 + direct_offset + 2);

                let addr = Self::make_word(addr_lo, addr_hi) as u32;

                let data_lo = self.bus.read_byte((addr_bank << 16) | addr);
                let mut data_hi = 0u8;

                if sixteen_bits_mode {
                    data_hi = self.bus.read_byte((addr_bank << 16) | (addr + 1));
                }

                Self::make_word(data_lo, data_hi)
            }

            AddressMode::ZeroPageDirectIndirectIndexedLong => {
                self.incr_pc();
                let direct_offset = self.bus.read_dword(self.pbr_pc());

                let addr_lo = self.bus.read_byte(self.reg_d as u32 + direct_offset);
                let addr_hi = self.bus.read_byte(self.reg_d as u32 + direct_offset + 1);
                let addr_bank = self.bus.read_dword(self.reg_d as u32 + direct_offset + 2);

                let addr = (Self::make_word(addr_lo, addr_hi) + self.reg_y) as u32;

                let data_lo = self.bus.read_byte((addr_bank << 16) | addr);
                let mut data_hi = 0u8;

                if sixteen_bits_mode {
                    data_hi = self.bus.read_byte((addr_bank << 16) | (addr + 1));
                }

                Self::make_word(data_lo, data_hi)
            }

            AddressMode::StackRelative => {
                self.incr_pc();
                let stack_offset = self.bus.read_dword(self.pbr_pc());

                let data_lo = self.bus.read_byte(self.sp + stack_offset);
                let mut data_hi = 0u8;

                if sixteen_bits_mode {
                    data_hi = self.bus.read_byte(self.sp + stack_offset + 1);
                }

                Self::make_word(data_lo, data_hi)
            }

            AddressMode::StackRelativeIndirectIndexedY => {
                self.incr_pc();
                let stack_offset = self.bus.read_dword(self.pbr_pc());

                let addr_lo = self.bus.read_byte(self.sp + stack_offset);
                let addr_hi = self.bus.read_byte(self.sp + stack_offset + 1);

                let addr = (Self::make_word(addr_lo, addr_hi) + self.reg_y) as u32;

                let data_lo = self.bus.read_byte(addr);
                let mut data_hi = 0u8;

                if sixteen_bits_mode {
                    data_hi = self.bus.read_byte(addr + 1);
                }

                Self::make_word(data_lo, data_hi)
            }
        }
    }

    pub fn store(&mut self, mode: AddressMode, value: &Word, sixteen_bits_mode: bool) {
        match mode {
            AddressMode::Absolute => {
                self.incr_pc();
                let addr_lo = self.bus.read_byte(self.pbr_pc());

                self.incr_pc();
                let addr_hi = self.bus.read_byte(self.pbr_pc());

                let addr = Self::make_word(addr_lo, addr_hi) as u32;

                // data_lo
                self.bus
                    .write_byte(((self.reg_db as u32) << 16) | addr, value.lo());

                if sixteen_bits_mode {
                    // data_hi
                    self.bus
                        .write_byte(((self.reg_db as u32) << 16) | addr + 1, value.hi());
                }
            }

            AddressMode::AbsoluteIndexedX => {
                self.incr_pc();
                let addr_lo = self.bus.read_byte(self.pbr_pc());

                self.incr_pc();
                let addr_hi = self.bus.read_byte(self.pbr_pc());

                let addr = (Self::make_word(addr_lo, addr_hi) + self.reg_x) as u32;

                // data_lo
                self.bus
                    .write_byte(((self.reg_db as u32) << 16) | addr, value.lo());

                if sixteen_bits_mode {
                    // data_hi
                    self.bus
                        .write_byte(((self.reg_db as u32) << 16) | (addr + 1), value.hi());
                }
            }

            AddressMode::AbsoluteIndexedY => {
                self.incr_pc();
                let addr_lo = self.bus.read_byte(self.pbr_pc());

                self.incr_pc();
                let addr_hi = self.bus.read_byte(self.pbr_pc());

                let addr = (Self::make_word(addr_lo, addr_hi) + self.reg_y) as u32;

                // data_lo
                self.bus
                    .write_byte(((self.reg_db as u32) << 16) | addr, value.lo());

                if sixteen_bits_mode {
                    // data_hi
                    self.bus
                        .write_byte(((self.reg_db as u32) << 16) | (addr + 1), value.hi());
                }
            }

            AddressMode::Immediate => {
                // data_lo
                self.incr_pc();
                self.bus.write_byte(self.pbr_pc(), value.lo());

                if sixteen_bits_mode {
                    // data_hi
                    self.incr_pc();
                    self.bus.write_byte(self.pbr_pc(), value.hi());
                }
            }

            AddressMode::ZeroPage => {
                self.incr_pc();
                let direct_offset = self.bus.read_byte(self.pbr_pc()) as u32;

                // data_lo
                self.bus
                    .write_byte(self.reg_d as u32 + direct_offset, value.lo());

                if sixteen_bits_mode {
                    // data_hi
                    self.bus
                        .write_byte(self.reg_d as u32 + direct_offset + 1, value.hi());
                }
            }

            AddressMode::ZeroPageX => {
                self.incr_pc();
                let direct_offset = self.bus.read_byte(self.pbr_pc()) as u32;

                // data_lo
                self.bus
                    .write_byte((self.reg_d + self.reg_x) as u32 + direct_offset, value.lo());

                if sixteen_bits_mode {
                    // data_hi
                    self.bus.write_byte(
                        (self.reg_d + self.reg_x) as u32 + direct_offset + 1,
                        value.hi(),
                    );
                }
            }

            AddressMode::ZeroPageY => {
                self.incr_pc();
                let direct_offset = self.bus.read_byte(self.pbr_pc()) as u32;

                // data_lo
                self.bus
                    .write_byte((self.reg_d + self.reg_y) as u32 + direct_offset, value.lo());

                if sixteen_bits_mode {
                    // data_hi
                    self.bus.write_byte(
                        (self.reg_d + self.reg_y) as u32 + direct_offset + 1,
                        value.hi(),
                    );
                }
            }

            AddressMode::ZeroPageDirectIndirectIndexedY => {
                self.incr_pc();
                let direct_offset = self.bus.read_dword(self.pbr_pc());

                let addr_lo = self.bus.read_byte(self.reg_d as u32 + direct_offset);
                let addr_hi = self.bus.read_byte(self.reg_d as u32 + direct_offset + 1);
                let addr = (Self::make_word(addr_lo, addr_hi) + self.reg_y) as u32;

                // data_lo
                self.bus
                    .write_byte(((self.reg_db as u32) << 16) | addr, value.lo());

                if sixteen_bits_mode {
                    // data_hi
                    self.bus
                        .write_byte(((self.reg_db as u32) << 16) | (addr + 1), value.hi());
                }
            }

            AddressMode::ZeroPageDirectIndexedIndirectX => {
                self.incr_pc();
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

                if sixteen_bits_mode {
                    // data_hi
                    self.bus
                        .write_byte(((self.reg_db as u32) << 16) | (addr + 1), value.hi());
                }
            }

            AddressMode::AbsoluteLong => {
                self.incr_pc();
                let addr_lo = self.bus.read_byte(self.pbr_pc());

                self.incr_pc();
                let addr_hi = self.bus.read_byte(self.pbr_pc());

                self.incr_pc();
                let addr_bank = self.bus.read_dword(self.pbr_pc());

                let addr = Self::make_word(addr_lo, addr_hi) as u32;

                // data_lo
                self.bus.write_byte((addr_bank << 16) | addr, value.lo());

                if sixteen_bits_mode {
                    // data_hi
                    self.bus
                        .write_byte((addr_bank << 16) | (addr + 1), value.hi());
                }
            }

            AddressMode::AbsoluteLongIndexedX => {
                self.incr_pc();
                let addr_lo = self.bus.read_byte(self.pbr_pc());

                self.incr_pc();
                let addr_hi = self.bus.read_byte(self.pbr_pc());

                self.incr_pc();
                let addr_bank = self.bus.read_dword(self.pbr_pc());

                let addr = (Self::make_word(addr_lo, addr_hi) + self.reg_x) as u32;

                // data_lo
                self.bus.write_byte((addr_bank << 16) | addr, value.lo());

                if sixteen_bits_mode {
                    // data_hi
                    self.bus
                        .write_byte((addr_bank << 16) | (addr + 1), value.hi());
                }
            }

            AddressMode::DirectIndirect => {
                self.incr_pc();
                let direct_offset = self.bus.read_dword(self.pbr_pc());

                let addr_lo = self.bus.read_byte(self.reg_d as u32 + direct_offset);
                let addr_hi = self.bus.read_byte(self.reg_d as u32 + direct_offset + 1);
                let addr = Self::make_word(addr_lo, addr_hi) as u32;

                // data_lo
                self.bus
                    .write_byte(((self.reg_db as u32) << 16) | addr, value.lo());

                if sixteen_bits_mode {
                    // data_hi
                    self.bus
                        .write_byte(((self.reg_db as u32) << 16) | (addr + 1), value.hi());
                }
            }

            AddressMode::DirectIndirectLong => {
                self.incr_pc();
                let direct_offset = self.bus.read_dword(self.pbr_pc());

                let addr_lo = self.bus.read_byte(self.reg_d as u32 + direct_offset);
                let addr_hi = self.bus.read_byte(self.reg_d as u32 + direct_offset + 1);
                let addr_bank = self.bus.read_dword(self.reg_d as u32 + direct_offset + 2);

                let addr = Self::make_word(addr_lo, addr_hi) as u32;

                // data_lo
                self.bus.write_byte((addr_bank << 16) | addr, value.lo());

                if sixteen_bits_mode {
                    // data_hi
                    self.bus
                        .write_byte((addr_bank << 16) | (addr + 1), value.hi());
                }
            }

            AddressMode::ZeroPageDirectIndirectIndexedLong => {
                self.incr_pc();
                let direct_offset = self.bus.read_dword(self.pbr_pc());

                let addr_lo = self.bus.read_byte(self.reg_d as u32 + direct_offset);
                let addr_hi = self.bus.read_byte(self.reg_d as u32 + direct_offset + 1);
                let addr_bank = self.bus.read_dword(self.reg_d as u32 + direct_offset + 2);

                let addr = (Self::make_word(addr_lo, addr_hi) + self.reg_y) as u32;

                // data_lo
                self.bus.write_byte((addr_bank << 16) | addr, value.lo());

                if sixteen_bits_mode {
                    // data_hi
                    self.bus
                        .write_byte((addr_bank << 16) | (addr + 1), value.hi());
                }
            }

            AddressMode::StackRelative => {
                self.incr_pc();
                let stack_offset = self.bus.read_dword(self.pbr_pc());

                // data_lo
                self.bus.write_byte(self.sp + stack_offset, value.lo());

                if sixteen_bits_mode {
                    // data_hi
                    self.bus.write_byte(self.sp + stack_offset + 1, value.hi());
                }
            }

            AddressMode::StackRelativeIndirectIndexedY => {
                self.incr_pc();
                let stack_offset = self.bus.read_dword(self.pbr_pc());

                let addr_lo = self.bus.read_byte(self.sp + stack_offset);
                let addr_hi = self.bus.read_byte(self.sp + stack_offset + 1);

                let addr = (Self::make_word(addr_lo, addr_hi) + self.reg_y) as u32;

                // data_lo
                self.bus.write_byte(addr, value.lo());

                if sixteen_bits_mode {
                    // data_hi
                    self.bus.write_byte(addr + 1, value.hi());
                }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fetch_address_absolute() {
        let mut b = Bus::new();
        b.write_byte(0x100, 0xFF);
        b.write_byte(0x101, 0xFF);
        b.write_byte(0x12FFFF, 0x1);
        b.write_byte(0x130000, 0x2);
        let mut c = Cpu::new(Box::new(b));
        c.reg_db = 0x12;
        c.pc = 0xFF;
        let result = c.fetch(AddressMode::Absolute, true);
        assert_eq!(result, 0x201);
    }
}
