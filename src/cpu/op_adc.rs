use crate::cpu::alu::AddressMode;
use crate::cpu::alu::Cpu;
use crate::cpu::alu::*;

impl Cpu {
    pub fn op_adc(&mut self, opcode: u8) {
        let mut value = match opcode {
            0x69 => self.fetch(AddressMode::Immediate),
            0x65 => self.fetch(AddressMode::ZeroPage),
            0x75 => self.fetch(AddressMode::ZeroPageX),
            0x6D => self.fetch(AddressMode::Absolute),
            0x7D => self.fetch(AddressMode::AbsoluteIndexedX),
            0x79 => self.fetch(AddressMode::AbsoluteIndexedY),
            0x61 => self.fetch(AddressMode::ZeroPageDirectIndexedIndirectX),
            0x71 => self.fetch(AddressMode::ZeroPageDirectIndirectIndexedY),
            0x6F => self.fetch(AddressMode::AbsoluteLong),
            0x7F => self.fetch(AddressMode::AbsoluteLongIndexedX),
            0x72 => self.fetch(AddressMode::DirectIndirect),
            0x67 => self.fetch(AddressMode::DirectIndirectLong),
            0x77 => self.fetch(AddressMode::ZeroPageDirectIndirectIndexedLong),
            0x63 => self.fetch(AddressMode::StackRelative),
            0x73 => self.fetch(AddressMode::StackRelativeIndirectIndexedY),
            _ => panic!("invalid opcode {}", opcode),
        };

        // add carry flag
        value += (self.reg_p & S_CARRY) as u16;

        if self.reg_p & S_DECIMAL_MODE > 0 {
            let mut result1 = (self.reg_a.data & 0xF) + (value & 0xF);
            if result1 > 0x9 {
                // adjust
                result1 += 0x6;
            }

            let mut result2 =
                ((self.reg_a.data & 0xF0) >> 4) + ((value & 0xF0) >> 4) + ((result1 & 0xF0) >> 4);
            if result2 > 0x9 {
                // carry flag
                self.flag_c(self.emulation);

                // adjust
                result2 += 0x6;
            }

            if self.emulation {
                // 8-bit
                let result = (result2 & 0xF) << 4 | result1 & 0xF;

                // overflow flag
                self.flag_v(result < 0x80 || result > 0x7F);

                // mask 8-bit
                self.reg_a.data = result & 0xFF;
            } else {
                // 16-bit
                let mut result3 = ((self.reg_a.data & 0xF00) >> 8)
                    + ((value & 0xF00) >> 8)
                    + ((result2 & 0xF00) >> 8);
                if result3 > 0x9 {
                    // adjust
                    result3 += 0x6;
                }

                let mut result4 = ((self.reg_a.data & 0xF000) >> 12)
                    + ((value & 0xF000) >> 12)
                    + ((result3 & 0xF000) >> 12);

                if result4 > 0x9 {
                    // adjust
                    result4 += 0x6;
                    // carry flag
                    self.flag_c(true);
                }

                let result: u32 = ((result4 as u32 & 0xF) << 12)
                    | (result3 as u32 & 0xF) << 8
                    | (result2 as u32 & 0xF) << 4
                    | result1 as u32 & 0xF;

                // overflow flag
                self.flag_v(result < 0x8000 || result > 0x7F00);

                // mask 16-bit
                self.reg_a.data = (result & 0xFFFF) as u16;
            }
        } else {
            // hexadecimal mode
            self.check_overflow(self.reg_a.data as i32 + value as i32);
            self.reg_a.data += value;
        }

        // N and Z flags
        self.flag_nz(self.reg_a.data);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::{bits::Word, bus::Bus};

    #[test]
    fn op_adc_binary() {
        let mut b = Bus::new();
        b.write_byte(0x100, 0x46);
        let mut c = Cpu::new(Box::new(b));
        // PC
        c.pc = 0xFF;
        // CARRY
        c.reg_p = S_CARRY;
        // LDA
        c.reg_a = Word::new(0x0, 0x58);
        // immediate
        c.op_adc(0x69);
        assert_eq!(c.reg_a.data, 0x9F);
        assert_eq!(c.reg_p, 0x0);
    }

    #[test]
    fn op_adc_decimal() {

        /* Test 1 */
        let mut b = Bus::new();
        b.write_byte(0x100, 0x46);
        let mut c = Cpu::new(Box::new(b));
        // Emulation
        c.emulation = true;

        // PC
        c.pc = 0xFF;
        // CARRY + DECIMAL
        c.reg_p = S_CARRY | S_DECIMAL_MODE;
        // LDA
        c.reg_a = Word::new(0x0, 0x58);
        // immediate
        c.op_adc(0x69);
        assert_eq!(c.reg_a.data, 0x05);
        assert_eq!(c.reg_p & S_CARRY, S_CARRY);

        /* Test 2 */
        let mut b = Bus::new();
        b.write_byte(0x100, 0x34);
        let mut c = Cpu::new(Box::new(b));
        // Emulation
        c.emulation = true;
        // PC
        c.pc = 0xFF;
        // DECIMAL
        c.reg_p = S_DECIMAL_MODE;
        // LDA
        c.reg_a = Word::new(0x0, 0x12);
        // immediate
        c.op_adc(0x69);
        assert_eq!(c.reg_a.data, 0x46);
        assert_eq!(c.reg_p & S_CARRY, 0x0);

        /* Test 3 */
        let mut b = Bus::new();
        b.write_byte(0x100, 0x92);
        let mut c = Cpu::new(Box::new(b));
        // Emulation
        c.emulation = true;
        // PC
        c.pc = 0xFF;
        // DECIMAL
        c.reg_p = S_DECIMAL_MODE;
        // LDA
        c.reg_a = Word::new(0x0, 0x81);
        // immediate
        c.op_adc(0x69);
        assert_eq!(c.reg_a.data, 0x73);
        assert_eq!(c.reg_p & S_CARRY, S_CARRY);
    }
}
