mod registers;
mod memory;
mod cpu;
mod cartridge;

use registers::Registers;
use memory::Memory;
use cpu::CPU;
use cartridge::Cartridge;


fn main() {
    //let cpu = cpu::CPU::new();
    let cart = Cartridge::new("test_roms/tetris.gb");

    println!("{:X?}", cart.read(0x50));
}
