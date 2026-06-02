mod cartridge;
mod cpu;
mod memory;

use cartridge::cartridge::Cartridge;
use cpu::cpu::Cpu;
use memory::bus::Bus;
use std::{env, io};

fn main() -> cartridge::cartridge::Result<()> {
    let rom_path = env::args()
        .nth(1)
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Usage: gbc-emu <rom.gb>"))?;

    let mut cartridge = Cartridge::new();
    cartridge.load_rom(&rom_path)?;

    let header = cartridge.header()?;

    println!("Title: {}", header.title);
    println!("ROM: {} KiB", header.rom_size_bytes / 1024);
    println!("RAM: {} KiB", header.ram_size_bytes / 1024);

    let mut bus = Bus::new(cartridge.rom);
    let mut cpu = Cpu::new();

    bus.write(0x0000, 0x42);

    let opcode = cpu.fetch_byte(&bus);

    println!("First cartridge opcode: {:#X}", opcode);

    Ok(())
}
