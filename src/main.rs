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

    let rom_file = "test_roms/09-op r,r.gb";
    // read rom into vec<u8>
    let mut file = File::open(rom_file).expect("Failed to open ROM file");
    let mut data = Vec::new();
    file.read_to_end(&mut data).expect("Failed to read data ROM file.");

    // move rom data to memory unit
    let mut memory = Memory::new();
    for (i, b) in data.iter().enumerate() {
        memory.write(i as u16, *b);
    }

    let mut registers = Registers::new();

    registers.set_pc(0x100);

    let mut cpu = CPU::new_custom(registers, memory);

    
    for i in 100..500 {
        cpu.next_instruction();
        let mut s = String::new();
        std::io::stdin().read_line(&mut s);
    }
    
    //cpu.memory.write(0x100 as u16, 0x01);
    //cpu.memory.write(0x101 as u16, 0x44);
    //cpu.memory.write(0x102 as u16, 0x55);
    //cpu.exec();

    //println!("{:X?}", cpu.registers.get_bc());
}
