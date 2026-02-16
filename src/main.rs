use ddss_snes::cpu::alu::Cpu;
use ddss_snes::cpu::bus::Bus;
use ddss_snes::cartridge::open_rom;
use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        panic!("usage: ddss-snes rom-file.sfc")
    }

    open_rom(&args[1])?;

    let cpu = &mut Cpu::new(Box::new(Bus::new()));
    cpu.start();

    Ok(())
}
