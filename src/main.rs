use ddss_snes::cpu::alu::Cpu;
use ddss_snes::cpu::bus::Bus;

fn main() {
    let cpu = &mut Cpu::new(Box::new(Bus::new()));
    cpu.start();
}
