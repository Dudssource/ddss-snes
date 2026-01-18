use crate::cpu::alu::Cpu;
use crate::cpu::alu::AddressMode;
use crate::cpu::alu::*;

impl Cpu {
    pub fn op_sbc(&mut self, opcode: u8) {
        let mut value = match opcode {
            0xE9 => self.fetch(AddressMode::Immediate),
            0xE5 => self.fetch(AddressMode::ZeroPage),
            0xF5 => self.fetch(AddressMode::ZeroPageX),
            0xED => self.fetch(AddressMode::Absolute),
            0xFD => self.fetch(AddressMode::AbsoluteIndexedX),
            0xF9 => self.fetch(AddressMode::AbsoluteIndexedY),
            0xE1 => self.fetch(AddressMode::ZeroPageDirectIndexedIndirectX),
            0xF1 => self.fetch(AddressMode::ZeroPageDirectIndirectIndexedY),
            0xEF => self.fetch(AddressMode::AbsoluteLong),
            0xFF => self.fetch(AddressMode::AbsoluteLongIndexedX),
            0xF2 => self.fetch(AddressMode::DirectIndirect),
            0xE7 => self.fetch(AddressMode::DirectIndirectLong),
            0xF7 => self.fetch(AddressMode::ZeroPageDirectIndirectIndexedLong),
            0xE3 => self.fetch(AddressMode::StackRelative),
            0xF3 => self.fetch(AddressMode::StackRelativeIndirectIndexedY),
            _ => panic!("invalid opcode {}", opcode),
        } as i32;

        // add carry flag (0 means borrow, so we negate it before...)
        value += ((!self.reg_p) & S_CARRY) as i32;

        if self.reg_p & S_DECIMAL_MODE > 0 {
            let mut result1 = (self.reg_a.data as i32 & 0xF) - (value & 0xF);
            if (self.reg_a.data as i32 & 0xF) < (value & 0xF) {
                result1 += 0x6;
            }

            let mut result2 = ((self.reg_a.data as i32 & 0xF0) >> 4)
                - (((value & 0xF0) >> 4) - ((result1 & 0xF0) >> 4));
            if ((self.reg_a.data as i32 & 0xF0) >> 4)
                < (((value & 0xF0) >> 4) - ((result1 & 0xF0) >> 4))
            {
                result2 += 0x6;
            }

            if self.emulation {
                // 8-bit
                let result = (result2 & 0xF) << 4 | result1 & 0xF;

                // carry flag
                self.flag_c(result >= 0x100);

                // overflow flag
                self.flag_v(result < 0x80 || result > 0x7F);

                // mask 8-bit
                self.reg_a.data = (result & 0xFF) as u16;
            } else {
                // 16-bit
                let mut result3 = ((self.reg_a.data as i32 & 0xF00) >> 8)
                    - (((value & 0xF00) >> 8) - ((result2 & 0xF00) >> 8));
                if ((self.reg_a.data as i32 & 0xF00) >> 8)
                    < (((value & 0xF00) >> 8) - ((result2 & 0xF00) >> 8))
                {
                    result3 += 0x6;
                }

                let mut result4 = ((self.reg_a.data as i32 & 0xF000) >> 12)
                    - (((value & 0xF000) >> 12) - ((result3 & 0xF000) >> 12));

                if ((self.reg_a.data as i32 & 0xF000) >> 12)
                    < (((value & 0xF000) >> 12) - ((result3 & 0xF000) >> 12))
                {
                    result4 += 0x6;
                }

                let result: i32 = ((result4 as i32 & 0xF) << 12)
                    | (result3 as i32 & 0xF) << 8
                    | (result2 as i32 & 0xF) << 4
                    | result1 as i32 & 0xF;

                // carry flag
                self.flag_c(result >= 0x10000);

                // overflow flag
                self.check_overflow(result);

                // mask 16-bit
                self.reg_a.data = (result & 0xFFFF) as u16;
            }
        } else {
            // hexadecimal mode
            let mut result = self.reg_a.data as i32 - value as i32;
            self.check_overflow(result);
            // borrow
            if result < 0 {
                // wrap around
                result = 0x10000 + result;
            }
            self.reg_a.data = (result & 0xFFFF) as u16;
        }

        // N and Z flags
        self.flag_nz(self.reg_a.data);
    }
}
