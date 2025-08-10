const S_CARRY: u8 = 0x1;
const S_RESULT_ZERO: u8 = (0x1 << 1);
const S_IRQ_DISABLE: u8 = (0x1 << 2);
const S_DECIMAL_MODE: u8 = (0x1 << 3);
const S_BREAK_INSTRUCTION: u8 = (0x1 << 4);
const S_OVERFLOW: u8 = (0x1 << 6);
const S_NEGATIVE: u8 = (0x1 << 7);

pub struct Cpu {
    reg_a: u8,
    reg_x: u8,
    reg_y: u8,
    reg_p: u8,
    sp: u16,
    pc: u16,
}

impl Cpu {
    pub fn decode_and_execute(&mut self, bus: &mut Bus, opcode: u8) {
        match opcode {
            0xA1 => self.lda(opcode),
            0xA9 => self.lda(opcode),
        }
    }

    pub fn start(&mut self, bus: &mut Bus) {
        loop {
            let opcode = bus.read_byte(c.pc);
            self.decode_and_execute(&mut bus, opcode);
            self.pc += 1;
        }
    }

    pub fn offset(self, bus: Bus, addr: u16, mode: AddressMode) -> u16 {
        let value = match addr {
            ZeroPageIndirectIndexedY => {
                self.read_byte(self.read_byte(0x100 + addr) | (self.reg_y << 4))
            }
        };
    }

    fn lda(&mut self, opcode: u8) {
        self.reg_a = match opcode {
            0xA1 => self.offset(bus.read_byte(self.pc + 1), bus.ZeroPageIndirectIndexedY),
            0xA9 => self.read_byte(self.pc + 1),
            () => 0,
        };

        if self.reg_a == 0 {
            self.reg_p |= S_RESULT_ZERO;
        }
        if self.reg_a & 0x80 > 0 {
            self.reg_p |= S_NEGATIVE;
        } else {
            self.reg_p &= !S_NEGATIVE;
        }
    }
}
