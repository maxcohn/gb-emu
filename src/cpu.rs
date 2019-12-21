use crate::registers::Registers;

/// CPU and it's components: registers
struct CPU {
    registers: Registers,
}

impl CPU {
    /// Create a new CPU struct
    pub fn new() -> CPU {
        CPU {
            registers: Registers::new()
        }
    }

    pub fn next_instruction(&mut self) {
        self.registers.get_pc
    }
}
