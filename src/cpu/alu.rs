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
            0xA9 => self.lda(),
        }
    }

    fn lda(&mut self) {
        self.pc = self.pc+1
        self.reg_a = bus.read_byte(self.pc);
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
