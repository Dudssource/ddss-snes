const STACK_POINTER_START: u16 = 0x1FF;
const STACK_POINTER_END: u16 = 0x100;

pub struct Bus {
    work_ram: Box<[u8]>,
}

impl Bus {
    pub fn new() -> Self {
        Self {
            work_ram: vec![0u8; i32::MAX as usize].into_boxed_slice(),
        }
    }

    pub fn write_byte(&mut self, addr: u32, val: u8) {
        match addr {
            _ => self.work_ram[addr as usize] = val,
        }
    }

    pub fn read_dword(&self, addr: u32) -> u32 {
        self.read_byte(addr) as u32
    }

    pub fn read_byte(&self, addr: u32) -> u8 {
        match addr {
            _ => self.work_ram[addr as usize],
        }
    }
}
