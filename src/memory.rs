use registers;

/*
    0000-3FFF   16KB ROM Bank 00     (in cartridge, fixed at bank 00)
    4000-7FFF   16KB ROM Bank 01..NN (in cartridge, switchable bank number)
    8000-9FFF   8KB Video RAM (VRAM) (switchable bank 0-1 in CGB Mode)
    A000-BFFF   8KB External RAM     (in cartridge, switchable bank, if any)
    C000-CFFF   4KB Work RAM Bank 0 (WRAM)
    D000-DFFF   4KB Work RAM Bank 1 (WRAM)  (switchable bank 1-7 in CGB Mode)
    E000-FDFF   Same as C000-DDFF (ECHO)    (typically not used)
    FE00-FE9F   Sprite Attribute Table (OAM)
    FEA0-FEFF   Not Usable
    FF00-FF7F   I/O Ports
    FF80-FFFE   High RAM (HRAM)
    FFFF        Interrupt Enable Register
*/
struct Memory {
    mem: [u8; 0xFFFF]
}

impl Memory {

    /// Create a new Memory struct
    pub fn new() -> Memory {
        Memory {
            mem: [0; 0xFFFF], // TODO set to random
        }
    }

    /// Read a byte from memory at the given address
    pub fn read(&self, addr: u16) -> u8 {
        self.mem[addr]
    }

    /// Write a byte to memory at the given address
    pub fn write(&mut self, addr: u16, data: u8) {
        self.mem[addr] = u8;
    }
}