use crate::cpu::alu::{AddressMode, Cpu, S_ACCUMULATOR_MEMORY, S_CARRY};
use crate::cpu::bits::Word;
use log::debug;

impl Cpu {
    pub fn op_ror(&mut self, opcode: u8) {
        let sixteen_bits_mode = (self.reg_p & S_ACCUMULATOR_MEMORY) == 0;
        let oldpc = self.pc;
        let val = match opcode {
            0x6A => self.reg_a.data,
            0x66 => self.fetch(AddressMode::ZeroPage, sixteen_bits_mode),
            0x76 => self.fetch(AddressMode::ZeroPageX, sixteen_bits_mode),
            0x6E => self.fetch(AddressMode::Absolute, sixteen_bits_mode),
            0x7E => self.fetch(AddressMode::AbsoluteIndexedX, sixteen_bits_mode),
            _ => panic!("invalid opcode {}", opcode),
        };

        let old_carry = self.reg_p & S_CARRY;
        let mut result = Word { data: 0x0 };
        let new_carry = (val & 0x1) > 0;

        if sixteen_bits_mode {
            result.data = ((val >> 1) & 0xFFFF) | (old_carry << 15) as u16;
        } else {
            result.data = ((val >> 1) & 0xFF) | (old_carry << 7) as u16;
        }

        self.flag_c(new_carry);
        self.flag_nz(result.data);

        match opcode {
            0x6A => self.reg_a = result.clone(),
            0x66 => self.store(AddressMode::ZeroPage, &result, sixteen_bits_mode),
            0x76 => self.store(AddressMode::ZeroPageX, &result, sixteen_bits_mode),
            0x6E => self.store(AddressMode::Absolute, &result, sixteen_bits_mode),
            0x7E => self.store(AddressMode::AbsoluteIndexedX, &result, sixteen_bits_mode),
            _ => panic!("invalid opcode {}", opcode),
        };

        debug!(
            "[0x{:X}:0x{:X}] ROR : RESULT=0x{:X} FLAGS={:b}",
            oldpc, opcode, result.data, self.reg_p
        );
    }
}
