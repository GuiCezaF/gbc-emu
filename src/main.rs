use std::{env, fs, io};

struct Cartridge {
    rom: Vec<u8>,
}

struct RomHeader {
    title: String,
    rom_size_bytes: usize,
    ram_size_bytes: usize,
}

fn rom_size(code: u8) -> Option<usize> {
    match code {
        0x00 => Some(32 * 1024),
        0x01 => Some(64 * 1024),
        0x02 => Some(128 * 1024),
        0x03 => Some(256 * 1024),
        0x04 => Some(512 * 1024),
        0x05 => Some(1024 * 1024),
        0x06 => Some(2 * 1024 * 1024),
        0x07 => Some(4 * 1024 * 1024),
        0x08 => Some(8 * 1024 * 1024),
        _ => None,
    }
}

fn ram_size(code: u8) -> Option<usize> {
    match code {
        0x00 => Some(0),
        0x01 => Some(2 * 1024),
        0x02 => Some(8 * 1024),
        0x03 => Some(32 * 1024),
        0x04 => Some(128 * 1024),
        0x05 => Some(64 * 1024),
        _ => None,
    }
}

impl Cartridge {
    fn new() -> Self {
        Self { rom: Vec::new() }
    }

    fn load_rom(&mut self, path: &str) -> io::Result<()> {
        self.rom = fs::read(path)?;
        Ok(())
    }

    fn header(&self) -> io::Result<RomHeader> {
        if self.rom.len() < 0x150 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "ROM is too small to contain a valid header",
            ));
        }

        let title = self.rom[0x0134..=0x0143]
            .iter()
            .copied()
            .take_while(|&b| b != 0)
            .filter(|&b| b.is_ascii_graphic() || b == b' ')
            .map(char::from)
            .collect();

        let rom_size_bytes = rom_size(self.rom[0x0148])
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Invalid ROM size code"))?;

        let ram_size_bytes = ram_size(self.rom[0x0149])
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Invalid RAM size code"))?;

        Ok(RomHeader {
            title,
            rom_size_bytes,
            ram_size_bytes,
        })
    }
}

fn main() -> io::Result<()> {
    let rom_path = env::args()
        .nth(1)
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Usage: gbc-emu <rom.gb>"))?;

    let mut cartridge = Cartridge::new();
    cartridge.load_rom(&rom_path)?;

    let header = cartridge.header()?;

    println!("Title: {}", header.title);
    println!("ROM: {} KiB", header.rom_size_bytes / 1024);
    println!("RAM: {} KiB", header.ram_size_bytes / 1024);

    Ok(())
}
