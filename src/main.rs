mod registers;
mod memory;
mod cpu;
use registers::Registers;


fn main() {
    let registers = registers::Registers::new();
    registers.print_registers();
    println!("Hello, world!");
}
