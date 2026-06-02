use std::{fmt, fs, io};

const LOGO: [u8; 48] = [
    0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D,
    0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99,
    0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,
];

pub struct Cartridge {
    // Raw ROM bytes read from the cartridge file.
    pub rom: Vec<u8>,
}

pub struct RomHeader {
    pub title: String,
    pub rom_size_bytes: usize,
    pub ram_size_bytes: usize,
}

#[derive(Debug)]
pub enum CartridgeError {
    Io(io::Error),
    RomTooSmall,
    InvalidHeaderChecksum,
    InvalidNintendoLogo,
    UnsupportedCartridgeType(u8),
    InvalidRomSize(u8),
    InvalidRamSize(u8),
}

pub type Result<T> = std::result::Result<T, CartridgeError>;

impl From<io::Error> for CartridgeError {
    fn from(err: io::Error) -> Self {
        CartridgeError::Io(err)
    }
}

impl fmt::Display for CartridgeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CartridgeError::Io(err) => {
                write!(f, "I/O error: {err}")
            }
            CartridgeError::RomTooSmall => {
                write!(f, "ROM is too small to contain a valid header")
            }
            CartridgeError::InvalidHeaderChecksum => {
                write!(f, "invalid ROM header checksum")
            }
            CartridgeError::InvalidNintendoLogo => {
                write!(f, "invalid Nintendo logo")
            }
            CartridgeError::UnsupportedCartridgeType(code) => {
                write!(f, "unsupported cartridge type: 0x{code:02X}")
            }
            CartridgeError::InvalidRomSize(code) => {
                write!(f, "invalid ROM size code: 0x{code:02X}")
            }
            CartridgeError::InvalidRamSize(code) => {
                write!(f, "invalid RAM size code: 0x{code:02X}")
            }
        }
    }
}

impl std::error::Error for CartridgeError {}

pub fn rom_size(code: u8) -> Option<usize> {
    match code {
        // 32 KiB ROM.
        0x00 => Some(32 * 1024),
        // 64 KiB ROM.
        0x01 => Some(64 * 1024),
        // 128 KiB ROM.
        0x02 => Some(128 * 1024),
        // 256 KiB ROM.
        0x03 => Some(256 * 1024),
        // 512 KiB ROM.
        0x04 => Some(512 * 1024),
        // 1 MiB ROM.
        0x05 => Some(1024 * 1024),
        // 2 MiB ROM.
        0x06 => Some(2 * 1024 * 1024),
        // 4 MiB ROM.
        0x07 => Some(4 * 1024 * 1024),
        // 8 MiB ROM.
        0x08 => Some(8 * 1024 * 1024),
        // Unknown or unsupported size code.
        _ => None,
    }
}

pub fn ram_size(code: u8) -> Option<usize> {
    match code {
        // No external RAM.
        0x00 => Some(0),
        // 2 KiB external RAM.
        0x01 => Some(2 * 1024),
        // 8 KiB external RAM.
        0x02 => Some(8 * 1024),
        // 32 KiB external RAM.
        0x03 => Some(32 * 1024),
        // 128 KiB external RAM.
        0x04 => Some(128 * 1024),
        // 64 KiB external RAM.
        0x05 => Some(64 * 1024),
        // Unknown or unsupported size code.
        _ => None,
    }
}

impl Cartridge {
    pub fn new() -> Self {
        Self { rom: Vec::new() }
    }

    pub fn load_rom(&mut self, path: &str) -> Result<()> {
        self.rom = fs::read(path)?;
        self.validate_logo()?;
        self.validate_cartridge_type()?;
        self.validate_checksum()?;
        Ok(())
    }

    pub fn header(&self) -> Result<RomHeader> {
        // The Game Boy header spans at least up to 0x014F.
        if self.rom.len() < 0x150 {
            return Err(CartridgeError::RomTooSmall);
        }

        let title = self.rom[0x0134..=0x0143]
            .iter()
            .copied()
            // Stop at the first zero byte because the title is zero-terminated.
            .take_while(|&b| b != 0)
            // Keep only printable ASCII characters and spaces.
            .filter(|&b| b.is_ascii_graphic() || b == b' ')
            // Convert bytes into a Rust string.
            .map(char::from)
            .collect();

        // Look up the ROM size from the header code at 0x0148.
        let rom_size_bytes =
            rom_size(self.rom[0x0148]).ok_or(CartridgeError::InvalidRomSize(self.rom[0x0148]))?;

        // Look up the RAM size from the header code at 0x0149.
        let ram_size_bytes =
            ram_size(self.rom[0x0149]).ok_or(CartridgeError::InvalidRamSize(self.rom[0x0149]))?;

        Ok(RomHeader {
            title,
            rom_size_bytes,
            ram_size_bytes,
        })
    }

    pub fn validate_checksum(&self) -> Result<()> {
        // The Game Boy header spans at least up to 0x014F.
        if self.rom.len() < 0x150 {
            return Err(CartridgeError::RomTooSmall);
        }

        // 0x0134...0x014C checksum bytes
        // 0x014D 8-bit checksum computed

        let checksum = (0x0134..=0x014C).fold(0u8, |acc, addr| {
            acc.wrapping_sub(self.rom[addr]).wrapping_sub(1)
        });

        if checksum != self.rom[0x014D] {
            return Err(CartridgeError::InvalidHeaderChecksum);
        }

        Ok(())
    }
    fn validate_logo(&self) -> Result<()> {
        let logo_exists = self.rom.get(0x0104..0x0134) == Some(&LOGO[..]);
        if !logo_exists {
            return Err(CartridgeError::InvalidNintendoLogo);
        }
        Ok(())
    }
    fn validate_cartridge_type(&self) -> Result<()> {
        // suported types:
        // 0x00       -> ROM ONLY
        // 0x01..=0x03 -> MBC1
        // 0x05..=0x06 -> MBC2
        // 0x08..=0x09 -> ROM + RAM
        // 0x0B..=0x0D -> MMM01
        // 0x0F..=0x13 -> MBC3
        // 0x19..=0x1E -> MBC5
        // 0x20       -> MBC6
        // 0x22       -> MBC7
        // 0xFC       -> Pocket Camera
        // 0xFD       -> Bandai Tama5
        // 0xFE       -> HuC3
        // 0xFF       -> HuC1 + RAM + Battery

        let code = self.rom[0x0147];

        if matches!(
            code,
            0x00
                | 0x01..=0x03
                | 0x05..=0x06
                | 0x08..=0x09
                | 0x0B..=0x0D
                | 0x0F..=0x13
                | 0x19..=0x1E
                | 0x20
                | 0x22
                | 0xFC..=0xFF
        ) {
            return Ok(());
        }

        Err(CartridgeError::UnsupportedCartridgeType(code))
    }
}
