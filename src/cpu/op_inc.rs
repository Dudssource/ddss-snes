use crate::cpu::{
    alu::{AddressMode, Cpu},
    bits::Word,
};

impl Cpu {
    pub fn op_inc(&mut self, opcode: u8) {
        let mut value = match opcode {
            0xE6 => self.fetch(AddressMode::ZeroPage),
            0xF6 => self.fetch(AddressMode::ZeroPageX),
            0xEE => self.fetch(AddressMode::Absolute),
            0xFE => self.fetch(AddressMode::AbsoluteIndexedX),
            0x1A => self.reg_a.data,
            _ => panic!("invalid opcode {}", opcode),
        };

        // ubc
        if self.emulation {
            value = (value & 0xFF) + 1
        } else {
            value += 1
        };

        // NZ
        self.flag_nz(value);

        // store
        match opcode {
            0xE6 => self.store(AddressMode::ZeroPage, &Word { data: value }),
            0xF6 => self.store(AddressMode::ZeroPageX, &Word { data: value }),
            0xEE => self.store(AddressMode::Absolute, &Word { data: value }),
            0xFE => self.store(AddressMode::AbsoluteIndexedX, &Word { data: value }),
            0x1A => self.reg_a.data = value,
            _ => panic!("invalid opcode {}", opcode),
        };
    }
}
