use ddss_snes::cpu::alu::Cpu;
use ddss_snes::cpu::bus::Bus;
use ddss_snes::rom::open;
use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {

    env_logger::init();
    
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        panic!("usage: ddss-snes rom-file.sfc")
    }

    let rom = open(&args[1])?;
    let bus = rom.map_to(Box::new(Bus::new()))?;

    let cpu = &mut Cpu::new(bus);
    cpu.start();

    Ok(())
}
