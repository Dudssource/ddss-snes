use std::error::Error;
use std::fs::read;
use std::fmt;

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

pub struct Cartdrige {
    pub headered: bool,
    pub game_title: String,
    pub fast_rom: bool,
    pub rom_mode: MapMode,
    pub chipset : ChipsetType,
    pub rom_size: u32,
    pub ram_size: u32,
    pub region: Region,
}

impl fmt::Display for Cartdrige {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(game_title: {}, rom_mode: {:?}, chipset: {:?}, fast_rom: {}, rom_size: {}, ram_size: {}, country: {:?})", self.game_title, self.rom_mode, self.chipset, self.fast_rom, self.rom_size, self.ram_size, self.region)
    }
}

pub fn open_rom(path: &str) -> Result<(), Box<dyn Error>> {

    let rom_file = read(path)?;
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

    let rom_size = 1 << rom_file[0x7FD7];
    let ram_size = 1 << rom_file[0x7FD8];

    let region = match rom_file[0x7FD9] {
        0x0 => Ok(Region::Japan),
        0x1 => Ok(Region::USA),
        0x2 => Ok(Region::Europe),
        _ => Err("invalid region"),
    }?;

    let rom = Cartdrige {
        region: region,
        fast_rom: rom_file[0x7FD5] & 0x10 > 0,
        game_title: String::from(game_title),
        ram_size: ram_size,
        rom_size: rom_size,
        chipset: cartridge_type,
        rom_mode: rom_mode,
        headered: headered,
    };

    println!("{}", rom);

    Ok(())
}
