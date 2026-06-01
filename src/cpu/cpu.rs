use crate::memory::bus::Bus;

pub struct Cpu {
    // Program counter points to the next byte to fetch.
    pc: u16,
}

impl Cpu {
    pub fn new() -> Self {
        Self { pc: 0 }
    }

    // Read the byte at the current program counter and advance it.
    pub fn fetch_byte(&mut self, bus: &Bus) -> u8 {
        // Read the current instruction byte from memory.
        let value = bus.read(self.pc);

        // Advance to the next byte, wrapping on overflow.
        self.pc = self.pc.wrapping_add(1);

        // Return the fetched byte to the caller.
        return value;
    }
}
