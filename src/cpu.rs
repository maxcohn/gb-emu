use crate::registers::Registers;
use crate::memory::Memory;
/// CPU and it's components: registers
pub struct CPU {
    registers: Registers,
    memory: Memory,
}

impl CPU {
    /// Create a new CPU struct
    pub fn new() -> CPU {
        CPU {
            registers: Registers::new(),
            memory: Memory::new(),
        }
    }

    pub fn next_instruction(&mut self) -> u8{
        let cur_pc = self.registers.get_pc();
        let cur_op = self.memory.read(cur_pc);

        cur_op
    }
}
