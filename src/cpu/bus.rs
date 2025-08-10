const STACK_POINTER_START: u16 = 0x1FF;
const STACK_POINTER_END: u16 = 0x100;

enum AddressMode {
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
    ZeroPageIndirectIndexedX,
    ZeroPageIndirectIndexedY,
}

pub struct Bus {
    work_ram: Box<[u8; 0xFFFF]>,
}

impl Bus {
    pub fn new() -> Self {
        Self {
            work_ram: Box::new([0; 0xFFFF]),
        }
    }

    pub fn write_byte(&mut self, val: u8, addr: u16) {
        match addr {}
    }

    pub fn read_byte(&mut self, addr: u16) -> u8{
        match addr {}
    }

    pub fn offset(self, addr: u16, mode: AddressMode) -> u16 {
        let value = match addr {
            
        };
    }
}
