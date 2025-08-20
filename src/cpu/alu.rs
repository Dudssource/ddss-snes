use crate::cpu::bus::Bus;

const S_CARRY: u8 = 0x1;
const S_RESULT_ZERO: u8 = 0x1 << 1;
const S_IRQ_DISABLE: u8 = 0x1 << 2;
const S_DECIMAL_MODE: u8 = 0x1 << 3;
const S_BREAK_INSTRUCTION: u8 = 0x1 << 4;
const S_OVERFLOW: u8 = 0x1 << 6;
const S_NEGATIVE: u8 = 0x1 << 7;

pub struct Cpu<'a> {
    reg_a: u8,
    reg_x: u8,
    reg_y: u8,
    reg_p: u8,
    reg_dp: u16,
    sp: u16,
    pc: u16,
    bus: &'a mut Bus,
}

pub enum AddressMode {
    Implied,
    Accumulator,
    Immediate,
    Absolute,
    ProgramCounterRelative,
    Stack,
    ZeroStack,
    AbsoluteIndexedX,
    AbsoluteIndexedY,
    ZeroPageIndexedX,
    ZeroPageIndexedY,
    AbsoluteIndirect,
    ZeroPage,
    ZeroPageIndirectIndexedX,
    ZeroPageIndirectIndexedY,
    DirectPage,
}

impl<'a> Cpu<'a> {
    pub fn new(bus: &'a mut Bus) -> Self {
        Self {
            bus: bus,
            sp: 0xFF,
            reg_a: 0x0,
            reg_x: 0x0,
            reg_y: 0x0,
            reg_p: 0x0,
            reg_dp: 0x0,
            pc: 0x0,
        }
    }

    pub fn decode_and_execute(&mut self, opcode: u8) {
        match opcode {
            0xA1 | 0xA3 | 0xA5 | 0xA9 => self.lda(opcode),
            _ => (),
        }
    }

    pub fn start(&mut self) {
        loop {
            let opcode = self.bus.read_byte(self.pc);
            self.decode_and_execute(opcode);
            self.pc += 1;
        }
    }

    pub fn fetch(&self, offset: u8, mode: AddressMode) -> u8 {
        let offset = self.pc + (offset as u16);
        match mode {
            AddressMode::ZeroPageIndirectIndexedY => self.bus.read_byte(
                ((self.bus.read_byte(offset) as u16 & 0x100) | (self.reg_y << 4) as u16) as u16,
            ),
            AddressMode::AbsoluteIndexedX => self
                .bus
                .read_byte((self.bus.read_byte(offset) + self.reg_x) as u16 & 0x100),
            AddressMode::Stack => self
                .bus
                .read_byte((self.sp + 0x100) - (self.fetch(1, AddressMode::Immediate)) as u16),
            AddressMode::ZeroPage => self.fetch(1, AddressMode::Immediate),
            AddressMode::DirectPage => self
                .bus
                .read_byte((self.fetch(1u8, AddressMode::Immediate)) as u16 + self.reg_dp),
            AddressMode::Immediate => self.bus.read_byte(offset),
            _ => self.bus.read_byte(offset),
        }
    }

    fn nz(&mut self, value: u8) {
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
        let reg_a = match opcode {
            0xA1 => self.fetch(1u8, AddressMode::ZeroPageIndirectIndexedX),
            0xA3 => self.fetch(1u8, AddressMode::Stack),
            0xA5 => self.fetch(1u8, AddressMode::ZeroPage),
            0xA9 => self.fetch(1u8, AddressMode::Immediate),
            _ => self.reg_a,
        };
        self.reg_a = reg_a;
        self.nz(self.reg_a);
    }
}
