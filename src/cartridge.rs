use std::fs::File;
use std::io::Read;

pub struct Cartridge {
    rom: Vec<u8>,
}

impl Cartridge {
    pub fn new(filename: &str) -> Cartridge{
        let mut file = File::open(filename).expect("Failed to open ROM file");
        let mut data = Vec::new();
        file.read_to_end(&mut data).expect("Failed to read data ROM file.");

        Cartridge {
            rom: data,
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        self.rom[addr as usize]
    }


}