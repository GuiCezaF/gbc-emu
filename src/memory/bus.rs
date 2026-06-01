const ROM_START: u16 = 0x0000;
const ROM_END: u16 = 0x7FFF;

const VRAM_START: u16 = 0x8000;
const VRAM_END: u16 = 0x9FFF;

const ERAM_START: u16 = 0xA000;
const ERAM_END: u16 = 0xBFFF;

const WRAM_START: u16 = 0xC000;
const WRAM_END: u16 = 0xDFFF;

const OAM_START: u16 = 0xFE00;
const OAM_END: u16 = 0xFE9F;

const IO_START: u16 = 0xFF00;
const IO_END: u16 = 0xFF7F;

const HRAM_START: u16 = 0xFF80;
const HRAM_END: u16 = 0xFFFE;

const IE_REGISTER: u16 = 0xFFFF;

pub struct Bus {
    /// Cartridge ROM data (includes header and program code).
    /// NOTE: This will later be replaced/extended by MBC (bank switching logic).
    rom: Vec<u8>,

    /// Video RAM (tile data, background maps, sprites data).
    vram: [u8; 0x2000],

    /// External RAM (cartridge RAM).
    /// NOTE: This is NOT always available. Depends on cartridge type (MBC).
    eram: [u8; 0x2000],

    /// Work RAM (internal system RAM).
    wram: [u8; 0x2000],

    /// Object Attribute Memory (sprite attribute table).
    oam: [u8; 0xA0],

    /// I/O registers (timer, joypad, PPU registers, etc.).
    /// NOTE: This is NOT plain memory. Each address has behavior.
    io: [u8; 0x80],

    /// High RAM (fast internal CPU memory).
    hram: [u8; 0x7F],

    /// Interrupt Enable register (IE at 0xFFFF).
    ie: u8,
}

impl Bus {
    /// Creates a new memory bus instance with a loaded ROM.
    /// All internal memory regions start cleared.
    pub fn new(rom: Vec<u8>) -> Self {
        Self {
            rom,

            vram: [0; 0x2000],
            eram: [0; 0x2000],
            wram: [0; 0x2000],
            oam: [0; 0xA0],
            io: [0; 0x80],
            hram: [0; 0x7F],

            ie: 0,
        }
    }

    /// Reads a byte from the memory map.
    /// This function routes the address to the correct hardware region.
    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            // Cartridge ROM (fixed region)
            ROM_START..=ROM_END => {
                // TODO: Replace with MBC (Memory Bank Controller) logic.
                // Right now this assumes linear ROM access only.
                self.rom[addr as usize]
            }

            // Video RAM (tile data, background maps)
            VRAM_START..=VRAM_END => self.vram[(addr - VRAM_START) as usize],

            // External cartridge RAM (battery-backed RAM, save data)
            ERAM_START..=ERAM_END => {
                // TODO: This must be controlled by MBC.
                self.eram[(addr - ERAM_START) as usize]
            }

            // Work RAM (system RAM)
            WRAM_START..=WRAM_END => self.wram[(addr - WRAM_START) as usize],

            // Sprite attribute table (OAM)
            OAM_START..=OAM_END => self.oam[(addr - OAM_START) as usize],

            // Hardware I/O registers
            IO_START..=IO_END => {
                // TODO: This must NOT be a direct memory read.
                // Each register has specific behavior:
                // - Timer registers increment automatically
                // - Joypad reflects input state
                // - PPU registers reflect GPU state
                self.io[(addr - IO_START) as usize]
            }

            // High RAM (fast internal CPU memory)
            HRAM_START..=HRAM_END => self.hram[(addr - HRAM_START) as usize],

            // Interrupt Enable register
            IE_REGISTER => self.ie,

            // Open bus behavior (undefined memory region)
            _ => 0xFF,
        }
    }

    /// Writes a byte into the memory map.
    /// Some regions are read-only or require special hardware behavior.
    pub fn write(&mut self, addr: u16, value: u8) {
        match addr {
            // ROM region is normally read-only.
            ROM_START..=ROM_END => {
                // TODO: This is NOT writable memory.
                // Writes here must be forwarded to MBC:
                // - bank switching
                // - enabling external RAM
                // - ROM banking control
            }

            // VRAM (video memory)
            VRAM_START..=VRAM_END => {
                self.vram[(addr - VRAM_START) as usize] = value;
            }

            // External cartridge RAM
            ERAM_START..=ERAM_END => {
                // TODO: Controlled by MBC (may be disabled or banked)
                self.eram[(addr - ERAM_START) as usize] = value;
            }

            // Work RAM
            WRAM_START..=WRAM_END => {
                self.wram[(addr - WRAM_START) as usize] = value;
            }

            // Sprite memory
            OAM_START..=OAM_END => {
                self.oam[(addr - OAM_START) as usize] = value;
            }

            // I/O registers
            IO_START..=IO_END => {
                // TODO: Must be replaced with register-specific behavior:
                // Example:
                // - writing 0xFF04 resets DIV timer
                // - writing 0xFF46 triggers DMA transfer
                // - writing LCDC affects PPU state
                self.io[(addr - IO_START) as usize] = value;
            }

            // High RAM
            HRAM_START..=HRAM_END => {
                self.hram[(addr - HRAM_START) as usize] = value;
            }

            // Interrupt Enable register
            IE_REGISTER => {
                self.ie = value;
            }

            // Unused or restricted memory area
            _ => {}
        }
    }
}
