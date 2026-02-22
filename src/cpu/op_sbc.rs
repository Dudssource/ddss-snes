use log::debug;

use crate::cpu::alu::AddressMode;
use crate::cpu::alu::Cpu;
use crate::cpu::alu::*;

impl Cpu {
    pub fn op_sbc(&mut self, opcode: u8) {
        let mut value = match opcode {
            0xE9 => self.fetch(AddressMode::Immediate, true),
            0xE5 => self.fetch(AddressMode::ZeroPage, true),
            0xF5 => self.fetch(AddressMode::ZeroPageX, true),
            0xED => self.fetch(AddressMode::Absolute, true),
            0xFD => self.fetch(AddressMode::AbsoluteIndexedX, true),
            0xF9 => self.fetch(AddressMode::AbsoluteIndexedY, true),
            0xE1 => self.fetch(AddressMode::ZeroPageDirectIndexedIndirectX, true),
            0xF1 => self.fetch(AddressMode::ZeroPageDirectIndirectIndexedY, true),
            0xEF => self.fetch(AddressMode::AbsoluteLong, true),
            0xFF => self.fetch(AddressMode::AbsoluteLongIndexedX, true),
            0xF2 => self.fetch(AddressMode::DirectIndirect, true),
            0xE7 => self.fetch(AddressMode::DirectIndirectLong, true),
            0xF7 => self.fetch(AddressMode::ZeroPageDirectIndirectIndexedLong, true),
            0xE3 => self.fetch(AddressMode::StackRelative, true),
            0xF3 => self.fetch(AddressMode::StackRelativeIndirectIndexedY, true),
            _ => panic!("invalid opcode {}", opcode),
        } as u32;

        // add carry flag (0 means borrow, so we negate it before...)
        value += ((!self.reg_p) & S_CARRY) as u32;

        // to help setting the carry flag properly
        let underflow = value > self.reg_a.data as u32;

        if self.reg_p & S_DECIMAL_MODE > 0 {
            if self.emulation {
                // convert subtrahend to base 10, so the number becomes a "signed" number
                // 8-bit
                value = ((9 - ((value & 0xF0) >> 4)) << 4) | ((9 - (value & 0xF)) + 1);
            } else {
                // the same as above, but 16-bit
                value = ((9 - ((value & 0xF000) >> 12)) << 12)
                    | ((9 - ((value & 0xF00) >> 8)) << 8)
                    | ((9 - ((value & 0xF0) >> 4)) << 4)
                    | ((9 - (value & 0xF)) + 1);
            }

            let mut result1 = (self.reg_a.data as u32 & 0xF) + (value & 0xF);
            if result1 > 0x9 {
                // adjust
                result1 += 0x6;
            }

            let mut result2 = ((self.reg_a.data as u32 & 0xF0) >> 4)
                + ((value & 0xF0) >> 4)
                + ((result1 & 0xF0) >> 4);

            if result2 > 0x9 {
                // carry flag
                self.flag_c(self.emulation);

                // adjust
                result2 += 0x6;
            }

            if self.emulation {
                // 8-bit
                let result = ((result2 & 0xF) << 4) | (result1 & 0xF);

                // overflow flag
                self.flag_v(result < 0x80 || result > 0x7F);

                // mask 8-bit
                self.reg_a.data = (result & 0xFF) as u16;
            } else {
                // 16-bit
                let mut result3 = ((self.reg_a.data as u32 & 0xF00) >> 8)
                    + ((value & 0xF00) >> 8)
                    + ((result2 & 0xF00) >> 8);
                if result3 > 0x9 {
                    // adjust
                    result3 += 0x6;
                }

                let mut result4 = ((self.reg_a.data as u32 & 0xF000) >> 12)
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
            self.reg_a.data = (self.reg_a.data as i32 - value as i32) as u16;
        }

        // handle underflow (wrap around)
        if underflow {
            self.flag_c(false);
        }

        // N and Z flags
        self.flag_nz(self.reg_a.data);

        debug!(
            "[0x{:X}] SBC : A=0x{:X} FLAGS={:b}",
            opcode, self.reg_a.data, self.reg_p
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::{bits::Word, bus::Bus};

    #[test]
    fn op_sbc_decimal() {
        /* Test 1 */
        let mut b = Bus::new();
        b.write_byte(0x100, 0x12);
        let mut c = Cpu::new(Box::new(b));
        // PC
        c.pc = 0xFF;
        // 8-bit
        c.emulation = true;
        // CARRY
        c.reg_p = S_CARRY | S_DECIMAL_MODE;
        // LDA
        c.reg_a = Word::new(0x0, 0x46);
        // immediate
        c.op_sbc(0xE9);
        assert_eq!(c.reg_a.data, 0x34);
        assert_eq!(c.reg_p & S_CARRY, 0x1);

        /* Test 2 */
        let mut b = Bus::new();
        b.write_byte(0x100, 0x13);
        let mut c = Cpu::new(Box::new(b));
        // PC
        c.pc = 0xFF;
        // 8-bit
        c.emulation = true;
        // CARRY
        c.reg_p = S_CARRY | S_DECIMAL_MODE;
        // LDA
        c.reg_a = Word::new(0x0, 0x40);
        // immediate
        c.op_sbc(0xE9);
        assert_eq!(c.reg_a.data, 0x27);
        assert_eq!(c.reg_p & S_CARRY, 0x1);

        /* Test 3 */
        let mut b = Bus::new();
        b.write_byte(0x100, 0x2);
        let mut c = Cpu::new(Box::new(b));
        // PC
        c.pc = 0xFF;
        // 8-bit
        c.emulation = true;
        // CARRY
        c.reg_p = S_DECIMAL_MODE;
        // LDA
        c.reg_a = Word::new(0x0, 0x32);
        // immediate
        c.op_sbc(0xE9);
        assert_eq!(c.reg_a.data, 0x29);
        assert_eq!(c.reg_p & S_CARRY, 0x1);

        /* Test 4 */
        let mut b = Bus::new();
        b.write_byte(0x100, 0x21);
        let mut c = Cpu::new(Box::new(b));
        // PC
        c.pc = 0xFF;
        // 8-bit
        c.emulation = true;
        // CARRY
        c.reg_p = S_CARRY | S_DECIMAL_MODE;
        // LDA
        c.reg_a = Word::new(0x0, 0x12);
        // immediate
        c.op_sbc(0xE9);
        assert_eq!(c.reg_a.data, 0x91);
        assert_eq!(c.reg_p & S_CARRY, 0x0);

        /* Test 5 */
        let mut b = Bus::new();
        b.write_byte(0x100, 0x03);
        b.write_byte(0x101, 0x20);
        let mut c = Cpu::new(Box::new(b));
        // PC
        c.pc = 0xFF;
        // 16-bit
        c.emulation = false;
        // CARRY
        c.reg_p = S_CARRY;
        // LDA
        c.reg_a = Word::new(0x0, 0x1);
        // immediate
        c.op_sbc(0xE9);
        assert_eq!(c.reg_a.data, 0xDFFE);
        assert_eq!(S_NEGATIVE, c.reg_p);

        /* Test 6 */
        let mut b = Bus::new();
        b.write_byte(0x100, 0x03);
        b.write_byte(0x101, 0x20);
        let mut c = Cpu::new(Box::new(b));
        // PC
        c.pc = 0xFF;
        // 16-bit
        c.emulation = false;
        // CARRY + DECIMAL
        c.reg_p = S_CARRY | S_DECIMAL_MODE;
        // LDA
        c.reg_a = Word::new(0x0, 0x1);
        // immediate
        c.op_sbc(0xE9);
        assert_eq!(c.reg_a.data, 0x7998);
        assert_eq!(S_DECIMAL_MODE | S_OVERFLOW, c.reg_p);
    }
}
