mod registers;
mod memory;
mod cpu;
mod cartridge;

use registers::Registers;
use memory::Memory;
use cpu::CPU;
use cartridge::Cartridge;

use std::fs::File;
use std::io::Read;

fn main() {

    // read rom into vec<u8>
    let mut file = File::open("test_roms/tetris.gb").expect("Failed to open ROM file");
    let mut data = Vec::new();
    file.read_to_end(&mut data).expect("Failed to read data ROM file.");

    // move rom data to memory unit
    let mut memory = Memory::new();
    for (i, b) in data.iter().enumerate() {
        memory.write(i as u16, *b);
    }

    let mut cpu = CPU::new(memory);

    println!("{:X?}", cpu.next_instruction());
}
