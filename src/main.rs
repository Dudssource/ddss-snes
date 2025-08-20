use ddss_snes::cpu::alu::Cpu;
use ddss_snes::cpu::bus::Bus;

fn main() {
    let bus = &mut Bus::new();
    let cpu = Cpu::new(bus);
}
