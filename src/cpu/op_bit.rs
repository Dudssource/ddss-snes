use log::debug;

use crate::cpu::alu::{AddressMode, Cpu, S_ACCUMULATOR_MEMORY, S_NEGATIVE, S_RESULT_ZERO};

impl Cpu {
    pub fn op_bit(&mut self, opcode: u8) {
        let sixteen_bits_mode = (self.reg_p & S_ACCUMULATOR_MEMORY) == 0;
        let oldpc = self.pc;
        let value = match opcode {
            0x24 => self.fetch(AddressMode::ZeroPage, sixteen_bits_mode),
            0x2C => self.fetch(AddressMode::Absolute, sixteen_bits_mode),
            0x89 => self.fetch(AddressMode::Immediate, sixteen_bits_mode),
            0x34 => self.fetch(
                AddressMode::ZeroPageDirectIndexedIndirectX,
                sixteen_bits_mode,
            ),
            0x3C => self.fetch(AddressMode::AbsoluteIndexedX, sixteen_bits_mode),
            _ => panic!("invalid opcode {}", opcode),
        };

        // sets or clears the n flag to reflect the value of the high bit of the
        // data located at the effective address specified by the operand
        self.flag(
            S_NEGATIVE,
            ((self.emulation || !sixteen_bits_mode) && (value & 0x80) > 0) || (value & 0x8000) > 0,
        );

        // sets or clears the v flag to reflect the contents of the
        // next-to-highest bit of the data addressed
        self.flag_v(
            ((self.emulation || !sixteen_bits_mode) && (value & 0x40) > 0) || (value & 0x4000) > 0,
        );

        let mut reg_a = self.reg_a.data;
        if self.emulation || !sixteen_bits_mode {
            reg_a &= 0xFF;
        }

        // it logically ANDs the data located at the effective address with the contents of the accumulator;
        // it changes neither value, but sets the z flag if the result is zero,
        // or clears it if the result is non-zero
        let result = reg_a & value;
        self.flag(S_RESULT_ZERO, result == 0);

        debug!(
            "[0x{:X}:0x{:X}] BIT : VALUE=0x{:X} RESULT=0x{:X} FLAGS={:b}",
            oldpc, opcode, value, result, self.reg_p
        );
    }
}
