use crate::cpu::bus::Bus;
use log::{debug, info};
use std::error::Error;
use std::fmt;
use std::fs::read;

#[derive(Debug)]
pub enum MapMode {
    LoRom2_68MHz,   // 0x20
    HiRom2_68MHz,   // 0x21
    SA1,            // 0x23
    ExHiRom2_68MHz, // 0x25
    LoRom3_58MHz,   // 0x30
    HiRom3_58MHz,   // 0x31
    ExHiRom3_58MHz, // 0x35
}

#[derive(Debug)]
pub enum ChipsetType {
    ROM,              // 0x0
    ROMRAM,           // 0x1
    ROMRAMBATTERY,    // 0x2,
    ROMSA1,           // 0x33
    ROMSA1RAM,        // 0x34
    ROMSA1RAMBATTERY, // 0x35
    ROMSA1BATTERY,    // 0x36
}

#[derive(Debug)]
pub enum Region {
    Japan,
    USA,
    Europe,
}

pub struct ROM {
    pub headered: bool,
    pub game_title: String,
    pub fast_rom: bool,
    pub rom_mode: MapMode,
    pub chipset: ChipsetType,
    pub rom_size: u32,
    pub real_rom_size: u8,
    pub ram_size: u32,
    pub real_ram_size: u8,
    pub region: Region,
    pub data: Vec<u8>,
}

impl fmt::Display for ROM {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "(game_title: {}, rom_mode: {:?}, chipset: {:?}, fast_rom: {}, rom_size: {}, ram_size: {}, country: {:?})",
            self.game_title,
            self.rom_mode,
            self.chipset,
            self.fast_rom,
            self.real_rom_size,
            self.ram_size,
            self.region
        )
    }
}

impl ROM {
    pub fn map_to(&self, mut bus: Box<Bus>) -> Result<Box<Bus>, Box<dyn Error>> {
        const BASE_ADDRESS: u32 = 0x8000;
        let mut counter: u32 = 0;
        let mut addr_counter: u32 = 0;
        let mut bank = 0x0;
        while counter < self.data.len() as u32 {
            let mut chunk_size: u32 = 0;
            debug!(
                "[start] bank {} addr 0x{:X} offset 0x{:X} ... ",
                bank,
                (BASE_ADDRESS + addr_counter),
                counter,
            );
            while chunk_size < 32768 {
                if bank < 0x7E {
                    let addr = (bank << 16) | (BASE_ADDRESS + addr_counter);
                    bus.write_byte(addr, self.data[counter as usize]);
                }
                let addr = ((bank + 0x80) << 16) | (BASE_ADDRESS + addr_counter);
                bus.write_byte(addr, self.data[counter as usize]);
                chunk_size += 1;
                counter += 1;
                addr_counter += 1;
            }
            debug!(
                "[end] bank {} addr 0x{:X} offset 0x{:X}",
                bank,
                (BASE_ADDRESS + addr_counter),
                counter,
            );
            bank += 1;
            addr_counter = 0;
        }

        Ok(bus)
    }
}

pub fn open(path: &str) -> Result<Box<ROM>, Box<dyn Error>> {
    let rom_file = read(path)?;
    let length = rom_file.len();
    let headered = rom_file.len() % 1024 == 512;

    let game_title = match std::str::from_utf8(&rom_file[0x7FC0..0x7FD4]) {
        Ok(decoded_string) => Ok(decoded_string.trim_end()),
        Err(e) => Err(e),
    }?;

    let rom_mode = match rom_file[0x7FD5] {
        0x20 => Ok(MapMode::LoRom2_68MHz),
        0x21 => Ok(MapMode::HiRom2_68MHz),
        0x23 => Ok(MapMode::SA1),
        0x25 => Ok(MapMode::ExHiRom2_68MHz),
        0x30 => Ok(MapMode::LoRom3_58MHz),
        0x31 => Ok(MapMode::HiRom3_58MHz),
        0x35 => Ok(MapMode::ExHiRom3_58MHz),
        _ => Err("invalid map mode"),
    }?;

    let cartridge_type = match rom_file[0x7FD6] {
        0x0 => Ok(ChipsetType::ROM),
        0x01 => Ok(ChipsetType::ROMRAM),
        0x02 => Ok(ChipsetType::ROMRAMBATTERY),
        0x33 => Ok(ChipsetType::ROMSA1),
        0x34 => Ok(ChipsetType::ROMSA1RAM),
        0x35 => Ok(ChipsetType::ROMSA1RAMBATTERY),
        0x36 => Ok(ChipsetType::ROMSA1BATTERY),
        _ => Err("invalid chipset type"),
    }?;

    let real_rom_size = rom_file[0x7FD7];
    let rom_size = 1 << real_rom_size;
    let real_ram_size = rom_file[0x7FD8];
    let ram_size = 1 << real_ram_size;

    let region = match rom_file[0x7FD9] {
        0x0 => Ok(Region::Japan),
        0x1 => Ok(Region::USA),
        0x2 => Ok(Region::Europe),
        _ => Err("invalid region"),
    }?;

    let rom = ROM {
        region: region,
        fast_rom: rom_file[0x7FD5] & 0x10 > 0,
        game_title: String::from(game_title),
        ram_size: ram_size,
        real_ram_size: real_ram_size,
        rom_size: rom_size,
        real_rom_size: real_rom_size,
        chipset: cartridge_type,
        rom_mode: rom_mode,
        headered: headered,
        data: rom_file,
    };

    info!("Loaded ROM: {} ({} bytes)", rom, length);

    Ok(Box::new(rom))
}
