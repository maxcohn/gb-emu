use crate::registers::Registers;
use crate::memory::Memory;

//TODO refactor to put all variants of an op into a single match arm, as opposed to
//TODO multiple, how it is now

//TODO replace all standard addition and subtraction with wrapping versions


/// Length of corresponding opcodes in bytes. If 0, that means the opcode isn't used
const OP_LENGTHS: [u8;0x100] = [
    1, 3, 1, 1, 1, 1, 2, 1, 3, 1, 1, 1, 1, 1, 2, 1, // 0
    1, 3, 1, 1, 1, 1, 2, 1, 2, 1, 1, 1, 1, 1, 2, 1, // 1
    2, 3, 1, 1, 1, 1, 2, 1, 2, 1, 1, 1, 1, 1, 2, 1, // 2
    2, 3, 1, 1, 1, 1, 2, 1, 2, 1, 1, 1, 1, 1, 2, 1, // 3
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 4
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 5
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 6
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 7
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 8
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 9
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // A
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // B
    1, 1, 3, 3, 3, 1, 2, 1, 1, 1, 3, 1, 3, 3, 2, 1, // C
    1, 1, 3, 0, 3, 1, 2, 1, 1, 1, 3, 0, 3, 0, 2, 1, // D
    2, 1, 1, 0, 0, 1, 2, 1, 2, 1, 3, 0, 0, 0, 2, 1, // E
    2, 1, 1, 1, 0, 1, 2, 1, 2, 1, 3, 1, 0, 0, 2, 1, // F
];

/// Number of CPU cycles that are required for the corresponding opcode. If 0,
/// then the opcode isn't used.
const OP_CYCLES: [u8; 0x100] = [
    4, 12, 8, 8, 4, 4, 8, 4, 20, 8, 8, 8, 4, 4, 8, 4,
    4, 12, 8, 8, 4, 4, 8, 4, 12, 8, 8, 8, 4, 4, 8, 4,
    12, 12, 8, 8, 4, 4, 8, 4, 12, 8, 8, 8, 4, 4, 8, 4,
    12, 12, 8, 8, 12, 12, 12, 4, 12, 8, 8, 8, 4, 4, 8, 4,
    4, 4, 4, 4, 4, 4, 8, 4, 4, 4, 4, 4, 4, 4, 8, 4,
    4, 4, 4, 4, 4, 4, 8, 4, 4, 4, 4, 4, 4, 4, 8, 4,
    4, 4, 4, 4, 4, 4, 8, 4, 4, 4, 4, 4, 4, 4, 8, 4,
    8, 8, 8, 8, 8, 8, 4, 8, 4, 4, 4, 4, 4, 4, 8, 4,
    4, 4, 4, 4, 4, 4, 8, 4, 4, 4, 4, 4, 4, 4, 8, 4,
    4, 4, 4, 4, 4, 4, 8, 4, 4, 4, 4, 4, 4, 4, 8, 4,
    4, 4, 4, 4, 4, 4, 8, 4, 4, 4, 4, 4, 4, 4, 8, 4,
    4, 4, 4, 4, 4, 4, 8, 4, 4, 4, 4, 4, 4, 4, 8, 4,
    20, 12, 16, 16, 24, 16, 8, 16, 20, 16, 16, 4, 24, 24, 8, 16,
    20, 12, 16, 0, 24, 16, 8, 16, 20, 16, 16, 0, 24, 0, 8, 16,
    12, 12, 8, 0, 0, 16, 8, 16, 16, 4, 16, 0, 0, 0, 8, 16,
    12, 12, 8, 4, 0, 16, 8, 16, 12, 8, 16, 4, 0, 0, 8, 16,
];

/// Mnemonic for the corresponding opcode. If empty string, that means the opcode
/// isn't used
const OP_MNEMONICS: [&str; 0x100] = [
    "NOP", "LD BC,d16", "LD (BC),A", "INC BC", "INC B", "DEC B", "LD B,d8", "RLCA", "LD (a16),SP", "ADD HL,BC", "LD A,(BC)", "DEC BC", "INC C", "DEC C", "LD C,d8", "RRCA",
    "STOP 0", "LD DE,d16", "LD (DE),A", "INC DE", "INC D", "DEC D", "LD D,d8", "RLA", "JR r8", "ADD HL,DE", "LD A,(DE)", "DEC DE", "INC E", "DEC E", "LD E,d8", "RRA",
    "JR NZ,r8", "LD HL,d16", "LD (HL+),A", "INC HL", "INC H", "DEC H", "LD H,d8", "DAA", "JR Z,r8", "ADD HL,HL", "LD A,(HL+)", "DEC HL", "INC L", "DEC L", "LD L,d8", "CPL",
    "JR NC,r8", "LD SP,d16", "LD (HL-),A", "INC SP", "INC (HL)", "DEC (HL)", "LD (HL),d8", "SCF", "JR C,r8", "ADD HL,SP", "LD A,(HL-)", "DEC SP", "INC A", "DEC A", "LD A,d8", "CCF",
    "LD B,B", "LD B,C", "LD B,D", "LD B,E", "LD B,H", "LD B,L", "LD B,(HL)", "LD B,A", "LD C,B", "LD C,C", "LD C,D", "LD C,E", "LD C,H", "LD C,L", "LD C,(HL)", "LD C,A",
    "LD D,B", "LD D,C", "LD D,D", "LD D,E", "LD D,H", "LD D,L", "LD D,(HL)", "LD D,A", "LD E,B", "LD E,C", "LD E,D", "LD E,E", "LD E,H", "LD E,L", "LD E,(HL)", "LD E,A",
    "LD H,B", "LD H,C", "LD H,D", "LD H,E", "LD H,H", "LD H,L", "LD H,(HL)", "LD H,A", "LD L,B", "LD L,C", "LD L,D", "LD L,E", "LD L,H", "LD L,L", "LD L,(HL)", "LD L,A",
    "LD (HL),B", "LD (HL),C", "LD (HL),D", "LD (HL),E", "LD (HL),H", "LD (HL),L", "HALT", "LD (HL),A", "LD A,B", "LD A,C", "LD A,D", "LD A,E", "LD A,H", "LD A,L", "LD A,(HL)", "LD A,A",
    "ADD A,B", "ADD A,C", "ADD A,D", "ADD A,E", "ADD A,H", "ADD A,L", "ADD A,(HL)", "ADD A,A", "ADC A,B", "ADC A,C", "ADC A,D", "ADC A,E", "ADC A,H", "ADC A,L", "ADC A,(HL)", "ADC A,A",
    "SUB B", "SUB C", "SUB D", "SUB E", "SUB H", "SUB L", "SUB (HL)", "SUB A", "SBC A,B", "SBC A,C", "SBC A,D", "SBC A,E", "SBC A,H", "SBC A,L", "SBC A,(HL)", "SBC A,A",
    "AND B", "AND C", "AND D", "AND E", "AND H", "AND L", "AND (HL)", "AND A", "XOR B", "XOR C", "XOR D", "XOR E", "XOR H", "XOR L", "XOR (HL)", "XOR A",
    "OR B", "OR C", "OR D", "OR E", "OR H", "OR L", "OR (HL)", "OR A", "CP B", "CP C", "CP D", "CP E", "CP H", "CP L", "CP (HL)", "CP A",
    "RET NZ", "POP BC", "JP NZ,a16", "JP a16", "CALL NZ,a16", "PUSH BC", "ADD A,d8", "RST 00H", "RET Z", "RET", "JP Z,a16", "PREFIX CB", "CALL Z,a16", "CALL a16", "ADC A,d8", "RST 08H",
    "RET NC", "POP DE", "JP NC,a16", "", "CALL NC,a16", "PUSH DE", "SUB d8", "RST 10H", "RET C", "RETI", "JP C,a16", "", "CALL C,a16", "", "SBC A,d8", "RST 18H",
    "LDH (a8),A", "POP HL", "LD (C),A", "", "", "PUSH HL", "AND d8", "RST 20H", "ADD SP,r8", "JP (HL)", "LD (a16),A", "", "", "", "XOR d8", "RST 28H",
    "LDH A,(a8)", "POP AF", "LD A,(C)", "DI", "", "PUSH AF", "OR d8", "RST 30H", "LD HL,SP+r8", "LD SP,HL", "LD A,(a16)", "EI", "", "", "CP d8", "RST 38H",
];

/// Length of CB prefixed opcodes in bytes. If 0, that means the opcode isn't used
const CB_LENGTHS: [u8; 0x100] = [
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
];

/// Number of CPU cycles that are require for the CB prefixed opcode
const CB_CYCLES: [u8; 0x100] = [
    8, 8, 8, 8, 8, 8, 16, 8, 8, 8, 8, 8, 8, 8, 16, 8,
    8, 8, 8, 8, 8, 8, 16, 8, 8, 8, 8, 8, 8, 8, 16, 8,
    8, 8, 8, 8, 8, 8, 16, 8, 8, 8, 8, 8, 8, 8, 16, 8,
    8, 8, 8, 8, 8, 8, 16, 8, 8, 8, 8, 8, 8, 8, 16, 8,
    8, 8, 8, 8, 8, 8, 16, 8, 8, 8, 8, 8, 8, 8, 16, 8,
    8, 8, 8, 8, 8, 8, 16, 8, 8, 8, 8, 8, 8, 8, 16, 8,
    8, 8, 8, 8, 8, 8, 16, 8, 8, 8, 8, 8, 8, 8, 16, 8,
    8, 8, 8, 8, 8, 8, 16, 8, 8, 8, 8, 8, 8, 8, 16, 8,
    8, 8, 8, 8, 8, 8, 16, 8, 8, 8, 8, 8, 8, 8, 16, 8,
    8, 8, 8, 8, 8, 8, 16, 8, 8, 8, 8, 8, 8, 8, 16, 8,
    8, 8, 8, 8, 8, 8, 16, 8, 8, 8, 8, 8, 8, 8, 16, 8,
    8, 8, 8, 8, 8, 8, 16, 8, 8, 8, 8, 8, 8, 8, 16, 8,
    8, 8, 8, 8, 8, 8, 16, 8, 8, 8, 8, 8, 8, 8, 16, 8,
    8, 8, 8, 8, 8, 8, 16, 8, 8, 8, 8, 8, 8, 8, 16, 8,
    8, 8, 8, 8, 8, 8, 16, 8, 8, 8, 8, 8, 8, 8, 16, 8,
    8, 8, 8, 8, 8, 8, 16, 8, 8, 8, 8, 8, 8, 8, 16, 8,
];

/// Mnemonic for the corresponding CB prefixed opcode. If empty string, that
/// means the opcode isn't used
const CB_MNEMONICS: [&str; 0x100] = [
    "RLC B", "RLC C", "RLC D", "RLC E", "RLC H", "RLC L", "RLC (HL)", "RLC A", "RRC B", "RRC C", "RRC D", "RRC E", "RRC H", "RRC L", "RRC (HL)", "RRC A",
    "RL B", "RL C", "RL D", "RL E", "RL H", "RL L", "RL (HL)", "RL A", "RR B", "RR C", "RR D", "RR E", "RR H", "RR L", "RR (HL)", "RR A",
    "SLA B", "SLA C", "SLA D", "SLA E", "SLA H", "SLA L", "SLA (HL)", "SLA A", "SRA B", "SRA C", "SRA D", "SRA E", "SRA H", "SRA L", "SRA (HL)", "SRA A",
    "SWAP B", "SWAP C", "SWAP D", "SWAP E", "SWAP H", "SWAP L", "SWAP (HL)", "SWAP A", "SRL B", "SRL C", "SRL D", "SRL E", "SRL H", "SRL L", "SRL (HL)", "SRL A",
    "BIT 0,B", "BIT 0,C", "BIT 0,D", "BIT 0,E", "BIT 0,H", "BIT 0,L", "BIT 0,(HL)", "BIT 0,A", "BIT 1,B", "BIT 1,C", "BIT 1,D", "BIT 1,E", "BIT 1,H", "BIT 1,L", "BIT 1,(HL)", "BIT 1,A",
    "BIT 2,B", "BIT 2,C", "BIT 2,D", "BIT 2,E", "BIT 2,H", "BIT 2,L", "BIT 2,(HL)", "BIT 2,A", "BIT 3,B", "BIT 3,C", "BIT 3,D", "BIT 3,E", "BIT 3,H", "BIT 3,L", "BIT 3,(HL)", "BIT 3,A",
    "BIT 4,B", "BIT 4,C", "BIT 4,D", "BIT 4,E", "BIT 4,H", "BIT 4,L", "BIT 4,(HL)", "BIT 4,A", "BIT 5,B", "BIT 5,C", "BIT 5,D", "BIT 5,E", "BIT 5,H", "BIT 5,L", "BIT 5,(HL)", "BIT 5,A",
    "BIT 6,B", "BIT 6,C", "BIT 6,D", "BIT 6,E", "BIT 6,H", "BIT 6,L", "BIT 6,(HL)", "BIT 6,A", "BIT 7,B", "BIT 7,C", "BIT 7,D", "BIT 7,E", "BIT 7,H", "BIT 7,L", "BIT 7,(HL)", "BIT 7,A",
    "RES 0,B", "RES 0,C", "RES 0,D", "RES 0,E", "RES 0,H", "RES 0,L", "RES 0,(HL)", "RES 0,A", "RES 1,B", "RES 1,C", "RES 1,D", "RES 1,E", "RES 1,H", "RES 1,L", "RES 1,(HL)", "RES 1,A",
    "RES 2,B", "RES 2,C", "RES 2,D", "RES 2,E", "RES 2,H", "RES 2,L", "RES 2,(HL)", "RES 2,A", "RES 3,B", "RES 3,C", "RES 3,D", "RES 3,E", "RES 3,H", "RES 3,L", "RES 3,(HL)", "RES 3,A",
    "RES 4,B", "RES 4,C", "RES 4,D", "RES 4,E", "RES 4,H", "RES 4,L", "RES 4,(HL)", "RES 4,A", "RES 5,B", "RES 5,C", "RES 5,D", "RES 5,E", "RES 5,H", "RES 5,L", "RES 5,(HL)", "RES 5,A",
    "RES 6,B", "RES 6,C", "RES 6,D", "RES 6,E", "RES 6,H", "RES 6,L", "RES 6,(HL)", "RES 6,A", "RES 7,B", "RES 7,C", "RES 7,D", "RES 7,E", "RES 7,H", "RES 7,L", "RES 7,(HL)", "RES 7,A",
    "SET 0,B", "SET 0,C", "SET 0,D", "SET 0,E", "SET 0,H", "SET 0,L", "SET 0,(HL)", "SET 0,A", "SET 1,B", "SET 1,C", "SET 1,D", "SET 1,E", "SET 1,H", "SET 1,L", "SET 1,(HL)", "SET 1,A",
    "SET 2,B", "SET 2,C", "SET 2,D", "SET 2,E", "SET 2,H", "SET 2,L", "SET 2,(HL)", "SET 2,A", "SET 3,B", "SET 3,C", "SET 3,D", "SET 3,E", "SET 3,H", "SET 3,L", "SET 3,(HL)", "SET 3,A",
    "SET 4,B", "SET 4,C", "SET 4,D", "SET 4,E", "SET 4,H", "SET 4,L", "SET 4,(HL)", "SET 4,A", "SET 5,B", "SET 5,C", "SET 5,D", "SET 5,E", "SET 5,H", "SET 5,L", "SET 5,(HL)", "SET 5,A",
    "SET 6,B", "SET 6,C", "SET 6,D", "SET 6,E", "SET 6,H", "SET 6,L", "SET 6,(HL)", "SET 6,A", "SET 7,B", "SET 7,C", "SET 7,D", "SET 7,E", "SET 7,H", "SET 7,L", "SET 7,(HL)", "SET 7,A",
];

// Link to a reddit thread with info on carries
//https://www.reddit.com/r/EmuDev/comments/4clh23/trouble_with_halfcarrycarry_flag/

/// Checks if the addition of two number results in a half carry
fn half_carry_add(a: u8, b: u8) -> bool {
    (((a & 0xf) + (b & 0xf)) & 0x10) == 0x10
}

/// Checks if the addition of two numbers results in a carry
fn carry_add(a: u8, b: u8) -> bool {
    a > (0xff - b)
}

/// Checks if the subtraction of two numbers results in a borrow in the 4th bit
fn half_carry_sub(a: u8, b: u8) -> bool {
    ((a & 0xF) - (b & 0xF)) < 0
}

/// Checks if the substraction of two numbers results in a borrow
fn carry_sub(a: u8, b: u8) -> bool {
    a < b
}

/// CPU and it's components: registers, memory
//TODO remove `pub` after done testing
pub struct CPU {
    pub registers: Registers,
    memory: Memory,
    /// Interrupt Master Enable
    pub ime: bool,
    /// Whether the CPU and LCD are halted at the moment
    pub halted: bool,
}

impl CPU {
    /// Create a new CPU struct
    pub fn new() -> CPU {
        CPU {
            registers: Registers::new(),
            memory: Memory::new(),
            ime: false, // IME is off by default
            halted: false,
        }
    }

    pub fn next_instruction(&mut self) -> u8{
        let cur_pc = self.registers.get_pc();
        let cur_op = self.memory.read(cur_pc);

        println!("Addr:{:X?}", cur_pc);
        //println!("{:X?}", cur_op);
        println!("{:?}", OP_MNEMONICS[cur_op as usize]);
        println!("Cycles: {:?}", OP_CYCLES[cur_op as usize]);
        println!("Length: {:?}", OP_LENGTHS[cur_op as usize]);
        println!();
        self.exec();
        self.registers.set_pc(cur_pc + OP_LENGTHS[cur_op as usize] as u16);

        cur_op
    }

    pub fn exec(&mut self) {
        let cur_pc = self.registers.get_pc();
        let cur_op = self.memory.read(cur_pc);

        // whether we should increment the PC. True most of the time, false when jumping.
        let mut inc_pc = true;

        match cur_op {
            0xD3 | 0xDB | 0xDD | 0xE3 | 0xE4 | 0xEB | 0xEC | 0xED | 0xF4 | 0xFC | 0xFD => {
                panic!("Opcode '{}' is unused and should not have occurred.", cur_op);
            },
            // NOP
            0x00 => {},
            // LD BC,d16
            0x01 => {
                let v = ( (self.memory.read(cur_pc + 1) as u16) << 8) | (self.memory.read(cur_pc + 2) as u16);
                self.registers.set_bc(v);
            },
            // LD (BC),A (register in parenthesis means store at memory location)
            0x02 => {
                let v = self.registers.get_bc();
                self.memory.write(v, self.registers.get_a());
            },
            // INC BC
            0x03 => self.registers.set_bc(self.registers.get_bc() + 1),
            // INC n
            0x04 | 0x0C | 0x14 | 0x1C | 0x24 | 0x2C | 0x34 | 0x3C => {
                // get original value and then calculate new value
                let v = match cur_op {
                    0x04 => self.registers.get_b(),
                    0x0C => self.registers.get_c(),
                    0x14 => self.registers.get_d(),
                    0x1C => self.registers.get_e(),
                    0x24 => self.registers.get_h(),
                    0x2C => self.registers.get_l(),
                    0x34 => self.memory.read(self.registers.get_hl()),
                    0x3C => self.registers.get_a(),
                    _ => panic!("Opcode '{:X?}' in INC n match arm", cur_op),
                };
                let res = v + 1;

                // set register to incremented version
                match cur_op {
                    0x04 => self.registers.set_b(v),
                    0x0C => self.registers.set_c(v),
                    0x14 => self.registers.set_d(v),
                    0x1C => self.registers.set_e(v),
                    0x24 => self.registers.set_h(v),
                    0x2C => self.registers.set_l(v),
                    0x34 => self.memory.write(self.registers.get_hl(), v),
                    0x3C => self.registers.set_a(v),
                    _ => panic!("Opcode '{:X?}' in INC n match arm", cur_op),
                }

                // set flags
                self.registers.set_flag_sub(0);
                self.registers.set_flag_half_carry(half_carry_add(v, res) as u8);
                self.registers.set_flag_zero((res == 0) as u8);
            },
            // DEC B
            0x05 => {
                // get original value and then calculate new value
                let v = self.registers.get_b();
                let res = v - 1;

                // set register to incremented version
                self.registers.set_b(res);

                // set flags
                self.registers.set_flag_sub(1);
                self.registers.set_flag_half_carry((v.trailing_zeros() >= 4) as u8);
                self.registers.set_flag_zero((res == 0) as u8);
            },
            // LD B,n
            0x06 => {
                let v = self.memory.read(cur_pc + 1);
                self.registers.set_b(v);
            },
            // RLCA
            0x07 => {
                let a = self.registers.get_a();
                // check if there is a new carry
                let c = ((a & 0x80) == 0x80) as u8;
                let res = (a << 1) | c;

                // set flags
                self.registers.set_flag_carry(c);
                self.registers.set_flag_half_carry(0);
                self.registers.set_flag_sub(0);
                self.registers.set_flag_zero((res == 0) as u8);
            },
            // LD (nn),SP
            0x08 => {
                let v = ( (self.memory.read(cur_pc + 1) as u16) << 8) | (self.memory.read(cur_pc + 2) as u16);
                self.registers.set_sp(v);
            },
            // ADD HL,n
            0x09 | 0x19 | 0x29 | 0x39 => {
                let v = match cur_op {
                    0x09 => self.registers.get_bc(),
                    0x19 => self.registers.get_de(),
                    0x29 => self.registers.get_hl(),
                    0x39 => self.registers.get_sp(),
                    _ => panic!("Opcode '{:X?}' in ADD HL,n match arm", cur_op),
                };
                let hl = self.registers.get_hl();
                let res = hl.wrapping_add(v);
                self.registers.set_hl(res);

                self.registers.set_flag_sub(0);
                // check for half carry in upper byte
                self.registers.set_flag_half_carry(half_carry_add(((v & 0xFF00) >> 8) as u8, ((hl & 0xFF00) >> 8) as u8) as u8);
                // check for carry in upper byte
                self.registers.set_flag_carry(carry_add(((hl & 0xFF00) >> 8) as u8, ((v & 0xFF00) >> 8) as u8) as u8);
            },
            // DEC BC
            0x0B => self.registers.set_bc(self.registers.get_bc() - 1),
            // DEC C
            0x0D => {
                // get original value and then calculate new value
                let v = self.registers.get_c();
                let res = v - 1;

                // set register to incremented version
                self.registers.set_c(res);

                // set flags
                self.registers.set_flag_sub(1);
                self.registers.set_flag_half_carry((v.trailing_zeros() >= 4) as u8);
                self.registers.set_flag_zero((res == 0) as u8);
            },
            // LD C,n
            0x0E => {
                let v = self.memory.read(cur_pc + 1);
                self.registers.set_c(v);
            },
            // RRCA
            0x0F => {
                let a = self.registers.get_a();
                // check if there is a new carry
                let c = ((a & 0x01) == 0x01) as u8;
                let res = if c == 0 {
                    a >> 1
                } else {
                    0x80 | (a >> 1)
                };

                // set flags
                self.registers.set_flag_carry(c);
                self.registers.set_flag_half_carry(0);
                self.registers.set_flag_sub(0);
                self.registers.set_flag_zero((res == 0) as u8);
            },
            // LD DE,nn
            0x11 => {
                let v = ( (self.memory.read(cur_pc + 1) as u16) << 8) | (self.memory.read(cur_pc + 2) as u16);
                self.registers.set_de(v);
            },
            // LD (DE),A
            0x12 => {
                let v = self.registers.get_de();
                self.memory.write(v, self.registers.get_a());
            },
            // INC DE
            0x13 => self.registers.set_de(self.registers.get_de() + 1),
            // DEC D
            0x15 => {
                // get original value and then calculate new value
                let v = self.registers.get_d();
                let res = v - 1;

                // set register to incremented version
                self.registers.set_d(res);

                // set flags
                self.registers.set_flag_sub(1);
                self.registers.set_flag_half_carry((v.trailing_zeros() >= 4) as u8);
                self.registers.set_flag_zero((res == 0) as u8);
            },
            // LD D,n
            0x16 => {
                let v = self.memory.read(cur_pc + 1);
                self.registers.set_d(v);
            },
            // RLA
            0x17 => {
                let a = self.registers.get_a();
                // check if there is a new carry
                let c = ((a & 0x80) == 0x80) as u8;
                let res = (a << 1) + self.registers.get_flag_carry();

                // set flags
                self.registers.set_flag_carry(c);
                self.registers.set_flag_half_carry(0);
                self.registers.set_flag_sub(0);
                self.registers.set_flag_zero((res == 0) as u8);
            },
            // JR n
            0x18 => {
                // get immediate value (have to do some interesting casting for addition later)
                let imm = self.get_imm_1byte(cur_pc) as i8 as i16 as u16;
                let new_pc = cur_pc + imm;
                // set PC to PC + immediate value
                self.registers.set_pc(new_pc);
                inc_pc = false;
            },
            // DEC DE
            0x1B => self.registers.set_de(self.registers.get_de() - 1),
            // DEC E
            0x1D => {
                // get original value and then calculate new value
                let v = self.registers.get_e();
                let res = v - 1;

                // set register to incremented version
                self.registers.set_e(res);

                // set flags
                self.registers.set_flag_sub(1);
                self.registers.set_flag_half_carry((v.trailing_zeros() >= 4) as u8);
                self.registers.set_flag_zero((res == 0) as u8);
            },
            // LD E,n
            0x1E => {
                let v = self.memory.read(cur_pc + 1);
                self.registers.set_e(v);
            },
            // RRA
            0x1F => {
                let a = self.registers.get_a();
                // check if there is a new carry
                let c = ((a & 0x01) == 0x01) as u8;
                let res = if self.registers.get_flag_carry() == 0 {
                    a >> 1
                } else {
                    0x80 | (a >> 1)
                };

                // set flags
                self.registers.set_flag_carry(c);
                self.registers.set_flag_half_carry(0);
                self.registers.set_flag_sub(0);
                self.registers.set_flag_zero((res == 0) as u8);
            },
            // JR cc,n
            0x20 | 0x28 | 0x30 | 0x38 => {
                // get immediate value (have to do some interesting casting for addition later)
                let imm = self.get_imm_1byte(cur_pc) as i8 as i16 as u16;
                let new_pc = cur_pc + imm;

                let is_jump = match cur_op {
                    0x20 => !(self.registers.get_flag_zero() != 0),
                    0x28 => self.registers.get_flag_zero() != 0,
                    0x30 => !(self.registers.get_flag_carry() != 0),
                    0x38 => self.registers.get_flag_carry() != 0,
                    _ => panic!("Opcode '{}' landed in JP cc,n match arm", cur_op),
                };

                if is_jump {
                    inc_pc = false;
                    self.registers.set_pc(new_pc);
                }

            }
            // LD HL,nn
            0x21 => {
                let v = ( (self.memory.read(cur_pc + 1) as u16) << 8) | (self.memory.read(cur_pc + 2) as u16);
                self.registers.set_hl(v);
            },
            // LDI (HL),A - LD (HL+),A - LD (HLI),A
            0x22 => {
                // store A into addr HL, increment HL
                self.memory.write(self.registers.get_hl(), self.registers.get_a());
                self.registers.set_hl(self.registers.get_hl() + 1);
            },
            // INC HL
            0x23 => self.registers.set_hl(self.registers.get_hl() + 1),
            // DEC H
            0x25 => {
                // get original value and then calculate new value
                let v = self.registers.get_h();
                let res = v - 1;

                // set register to incremented version
                self.registers.set_h(res);

                // set flags
                self.registers.set_flag_sub(1);
                self.registers.set_flag_half_carry((v.trailing_zeros() >= 4) as u8);
                self.registers.set_flag_zero((res == 0) as u8);
            },
            // LD H,n
            0x26 => {
                let v = self.memory.read(cur_pc + 1);
                self.registers.set_h(v);
            },
            // DAA
            //0x27 => {panic!("UNIMPLEMENTED 0x27");},
            // LDI A,(HL) - LD A,(HL+) - LD A,(HLI)
            0x2A => {
                // put value at addr HL into A, increment HL
                let v = self.memory.read(self.registers.get_hl());
                self.registers.set_a(v);
                self.registers.set_hl(self.registers.get_hl() + 1);
            },
            // DEC HL
            0x2B => self.registers.set_hl(self.registers.get_hl() - 1),
            // DEC L
            0x2D => {
                // get original value and then calculate new value
                let v = self.registers.get_l();
                let res = v - 1;

                // set register to incremented version
                self.registers.set_l(res);

                // set flags
                self.registers.set_flag_sub(1);
                self.registers.set_flag_half_carry((v.trailing_zeros() >= 4) as u8);
                self.registers.set_flag_zero((res == 0) as u8);
            },
            // LD L,n
            0x2E => {
                let v = self.memory.read(cur_pc + 1);
                self.registers.set_l(v);
            },
            // CPL
            0x2F => self.registers.set_a(!self.registers.get_a()),
            // LD SP,nn
            0x31 => {
                let v = ( (self.memory.read(cur_pc + 1) as u16) << 8) | (self.memory.read(cur_pc + 2) as u16);
                self.registers.set_sp(v);
            }
            // LDD (HL),A - LD (HL-),A - LD (HLD),A
            0x32 => {
                // put A into address HL, decrement HL
                self.memory.write(self.registers.get_hl(), self.registers.get_a());
                self.registers.set_hl(self.registers.get_hl() - 1);
            },
            // INC SP
            0x33 => self.registers.set_sp(self.registers.get_sp() + 1),
            // DEC (HL)
            0x35 => {
                // get original value and then calculate new value
                let v = self.memory.read(self.registers.get_hl());
                let res = v - 1;

                // set register to incremented version
                self.memory.write(self.registers.get_hl(), res);

                // set flags
                self.registers.set_flag_sub(1);
                self.registers.set_flag_half_carry((v.trailing_zeros() >= 4) as u8);
                self.registers.set_flag_zero((res == 0) as u8);
            },
            // LD (HL),n
            0x36 => {
                let imm = self.memory.read(cur_pc + 1);
                self.memory.write(self.registers.get_hl(), imm);
            }
            // SCF
            0x37 => {
                self.registers.set_flag_sub(0);
                self.registers.set_flag_half_carry(0);
                self.registers.set_flag_carry(1);
            }
            // LDD A,(HL) - LD A,(HL-) - LD A,(HLD)
            0x3A => {
                // store value at address HL into A, decrement HL
                let v = self.memory.read(self.registers.get_hl());
                self.registers.set_a(v);
                self.registers.set_hl(self.registers.get_hl() - 1);
            },
            // DEC sp
            0x3B => self.registers.set_sp(self.registers.get_sp() - 1),
            // DEC A
            0x3D => {
                // get original value and then calculate new value
                let v = self.registers.get_a();
                let res = v - 1;

                // set register to incremented version
                self.registers.set_a(res);

                // set flags
                self.registers.set_flag_sub(1);
                self.registers.set_flag_half_carry((v.trailing_zeros() >= 4) as u8);
                self.registers.set_flag_zero((res == 0) as u8);
            },
            // CCF
            0x3F => {
                let c = if self.registers.get_flag_carry() == 0 {
                    1
                } else {
                    0
                };
                self.registers.set_flag_sub(0);
                self.registers.set_flag_half_carry(0);
                self.registers.set_flag_carry(c);
            },
            // LD B,n
            0x40..=0x46 => {
                let v = match cur_op {
                    0x40 => self.registers.get_b(),
                    0x41 => self.registers.get_c(),
                    0x42 => self.registers.get_d(),
                    0x43 => self.registers.get_e(),
                    0x44 => self.registers.get_h(),
                    0x45 => self.registers.get_l(),
                    0x46 => self.memory.read(self.registers.get_hl()),
                    _ => panic!("Opcode: '{:X?}' seen within range 0x40..0x46 but was not", cur_op),
                };
                self.registers.set_b(v);
            },
            // LD n,A
            0x47 | 0x4F | 0x57 | 0x5F | 0x67 | 0x6F | 0x77 => {
                let a = self.registers.get_a();
                match cur_op {
                    0x47 => self.registers.set_b(a),
                    0x4F => self.registers.set_c(a),
                    0x57 => self.registers.set_d(a),
                    0x5F => self.registers.set_e(a),
                    0x67 => self.registers.set_h(a),
                    0x6F => self.registers.set_l(a),
                    0x77 => self.memory.write(self.registers.get_hl(), a),
                    _ => panic!("Opcode '{}' landed in LD n,A match arm", cur_op),
                }
            },
            // LD C,n
            0x48..=0x4E => {
                let v = match cur_op {
                    0x48 => self.registers.get_b(),
                    0x49 => self.registers.get_c(),
                    0x4A => self.registers.get_d(),
                    0x4B => self.registers.get_e(),
                    0x4C => self.registers.get_h(),
                    0x4D => self.registers.get_l(),
                    0x4E => self.memory.read(self.registers.get_hl()),
                    _ => panic!("Opcode: '{:X?}' seen within range 0x48..0x4E but was not", cur_op),
                };
                self.registers.set_c(v);
            },
            // LD D,n
            0x50..=0x56 => {
                let v = match cur_op {
                    0x50 => self.registers.get_b(),
                    0x51 => self.registers.get_c(),
                    0x52 => self.registers.get_d(),
                    0x53 => self.registers.get_e(),
                    0x54 => self.registers.get_h(),
                    0x55 => self.registers.get_l(),
                    0x56 => self.memory.read(self.registers.get_hl()),
                    _ => panic!("Opcode: '{:X?}' seen within range 0x50..0x56 but was not", cur_op),
                };
                self.registers.set_d(v);
            },
            // LD E,n
            0x58..=0x5E => {
                let v = match cur_op {
                    0x58 => self.registers.get_b(),
                    0x59 => self.registers.get_c(),
                    0x5A => self.registers.get_d(),
                    0x5B => self.registers.get_e(),
                    0x5C => self.registers.get_h(),
                    0x5D => self.registers.get_l(),
                    0x5E => self.memory.read(self.registers.get_hl()),
                    _ => panic!("Opcode: '{:X?}' seen within range 0x58..0x5E but was not", cur_op),
                };
                self.registers.set_e(v);
            },
            // LD H,n
            0x60..=0x66 => {
                let v = match cur_op {
                    0x60 => self.registers.get_b(),
                    0x61 => self.registers.get_c(),
                    0x62 => self.registers.get_d(),
                    0x63 => self.registers.get_e(),
                    0x64 => self.registers.get_h(),
                    0x65 => self.registers.get_l(),
                    0x66 => self.memory.read(self.registers.get_hl()),
                    _ => panic!("Opcode: '{:X?}' seen within range 0x60..0x46 but was not", cur_op),
                };
                self.registers.set_h(v);
            },
            // LD L,n
            0x68..=0x6E => {
                let v = match cur_op {
                    0x68 => self.registers.get_b(),
                    0x69 => self.registers.get_c(),
                    0x6A => self.registers.get_d(),
                    0x6B => self.registers.get_e(),
                    0x6C => self.registers.get_h(),
                    0x6D => self.registers.get_l(),
                    0x6E => self.memory.read(self.registers.get_hl()),
                    _ => panic!("Opcode: '{:X?}' seen within range 0x68..0x6E but was not", cur_op),
                };
                self.registers.set_l(v);
            },
            // LD (HL),n
            0x70..=0x75 => {
                let v = match cur_op {
                    0x70 => self.registers.get_b(),
                    0x71 => self.registers.get_c(),
                    0x72 => self.registers.get_d(),
                    0x73 => self.registers.get_e(),
                    0x74 => self.registers.get_h(),
                    0x75 => self.registers.get_l(),
                    _ => panic!("Opcode: '{:X?}' seen within range 0x60..0x46 but was not", cur_op),
                };
                self.memory.write(self.registers.get_hl(), v);
            },
            // HALT
            0x76 => self.halted = true,
            // LD A,n
            0x78..=0x7F | 0x0A | 0x1A | 0x3E | 0xFA => {
                let v = match cur_op {
                    0x0A => self.memory.read(self.registers.get_bc()),
                    0x1A => self.memory.read(self.registers.get_de()),
                    0x3E => self.get_imm_1byte(cur_pc),
                    0xFA => self.memory.read(self.get_imm_2byte(cur_pc)),
                    0x78 => self.registers.get_b(),
                    0x79 => self.registers.get_c(),
                    0x7A => self.registers.get_d(),
                    0x7B => self.registers.get_e(),
                    0x7C => self.registers.get_h(),
                    0x7D => self.registers.get_l(),
                    0x7E => self.memory.read(self.registers.get_hl()),
                    0x7F => self.registers.get_a(),
                    _ => panic!("Opcode: '{:X?}' seen within range 0x78..0x7F but was not", cur_op),
                };
                self.registers.set_a(v);
            },
            // ADD A,n
            0x80..=0x87 | 0xC6 => {
                let v = match cur_op {
                    0x80 => self.registers.get_b(),
                    0x81 => self.registers.get_c(),
                    0x82 => self.registers.get_d(),
                    0x83 => self.registers.get_e(),
                    0x84 => self.registers.get_h(),
                    0x85 => self.registers.get_l(),
                    0x86 => self.memory.read(self.registers.get_hl()),
                    0x87 => self.registers.get_a(),
                    0xC6 => self.get_imm_1byte(cur_pc),
                    _ => panic!("Opcode: '{:X?}' seen within range 0x80..0x87 and 0xC6 match arm", cur_op),
                };

                let a = self.registers.get_a();

                let res = a.wrapping_add(v);

                self.registers.set_a(res);

                // set flags
                self.registers.set_flag_zero((res == 0) as u8);
                self.registers.set_flag_carry(carry_add(a, v) as u8);
                self.registers.set_flag_half_carry(half_carry_add(a, v) as u8);
                self.registers.set_flag_sub(0);
            }
            // ADC A,n
            0x88..=0x8F | 0xCE => {
                let v = match cur_op {
                    0x88 => self.registers.get_b(),
                    0x89 => self.registers.get_c(),
                    0x8A => self.registers.get_d(),
                    0x8B => self.registers.get_e(),
                    0x8C => self.registers.get_h(),
                    0x8D => self.registers.get_l(),
                    0x8E => self.memory.read(self.registers.get_hl()),
                    0x8F => self.registers.get_a(),
                    0xCE => self.memory.read(cur_pc + 1),
                    _ => panic!("Opcode: '{:X?}' seen within range 0x88..0x8F and 0xCE match arm", cur_op),
                };
                let a = self.registers.get_a();
                let carry = self.registers.get_flag_carry();
                let res = a.wrapping_add(v.wrapping_add(carry));

                self.registers.set_a(res);

                // set flags
                self.registers.set_flag_zero((res == 0) as u8);
                // not using the carry_add function because we need 3 variables
                self.registers.set_flag_carry((a as u16 + carry as u16 + v as u16 > 0xFF) as u8);
                // not using the half_carry_add function because we need three variables here
                self.registers.set_flag_half_carry(((a & 0x0F) + (carry & 0x0F) + (v & 0x0F) > 0x0F) as u8);
                self.registers.set_flag_sub(0);
            },
            // SUB A,n
            0x90..=0x97 | 0xD6 => {
                let v = match cur_op {
                    0x90 => self.registers.get_b(),
                    0x91 => self.registers.get_c(),
                    0x92 => self.registers.get_d(),
                    0x93 => self.registers.get_e(),
                    0x94 => self.registers.get_h(),
                    0x95 => self.registers.get_l(),
                    0x96 => self.memory.read(self.registers.get_hl()),
                    0x97 => self.registers.get_a(),
                    0xD6 => self.memory.read(cur_pc + 1),
                    _ => panic!("Opcode: '{:X?}' got into inner match arm, not supposed to happen", cur_op),
                };
                let a = self.registers.get_a();
                let res = v - a;

                // store result in register A
                self.registers.set_a(res);

                // set flags
                self.registers.set_flag_zero((res == 0) as u8);
                self.registers.set_flag_sub(1);
                self.registers.set_flag_half_carry(half_carry_sub(v, a) as u8);
                self.registers.set_flag_carry(carry_sub(v, a) as u8);
            },
            // SBC A,n
            0x98..=0x9F | 0xDE => {
                let v = match cur_op {
                    0xDE => self.get_imm_1byte(cur_pc),
                    0x98 => self.registers.get_b(),
                    0x99 => self.registers.get_c(),
                    0x9A => self.registers.get_d(),
                    0x9B => self.registers.get_e(),
                    0x9C => self.registers.get_h(),
                    0x9D => self.registers.get_l(),
                    0x9E => self.memory.read(self.registers.get_hl()),
                    0x9F => self.registers.get_a(),
                    _ => panic!("Opcode '{}' landed in SBC A,n match arm", cur_op),
                };
                let a = self.registers.get_a();
                let c = self.registers.get_flag_carry();
                let res = v.wrapping_sub(a.wrapping_sub(c));


                self.registers.set_flag_zero((res == 0) as u8);
                self.registers.set_flag_sub(1);
                self.registers.set_flag_half_carry(((a & 0x0F) < (v & 0x0F) + c) as u8);
                self.registers.set_flag_carry(((a as u16) < (v as u16 + c as u16)) as u8);
            },
            // AND n
            0xA0..=0xA7 | 0xE6 => {
                let v = match cur_op {
                    0xA0 => self.registers.get_b(),
                    0xA1 => self.registers.get_c(),
                    0xA2 => self.registers.get_d(),
                    0xA3 => self.registers.get_e(),
                    0xA4 => self.registers.get_h(),
                    0xA5 => self.registers.get_l(),
                    0xA6 => self.memory.read(self.registers.get_hl()),
                    0xA7 => self.registers.get_a(),
                    0xE6 => self.get_imm_1byte(cur_pc),
                    _ => panic!("Opcode: '{:X?}' seen within range 0xA0..0xA7 but was not", cur_op),
                };
                // AND the wanted register with A
                let res = v & self.registers.get_a();
                self.registers.set_a(res);

                // set flags
                self.registers.set_flag_zero( (res == 0) as u8);
                self.registers.set_flag_carry(0);
                self.registers.set_flag_half_carry(1);
                self.registers.set_flag_sub(0);

            },
            // XOR n
            0xA8..=0xAF | 0xEE => {
                // get value store within wanted register
                let v = match cur_op {
                    0xAF => self.registers.get_a(),
                    0xA8 => self.registers.get_b(),
                    0xA9 => self.registers.get_c(),
                    0xAA => self.registers.get_d(),
                    0xAB => self.registers.get_e(),
                    0xAC => self.registers.get_h(),
                    0xAD => self.registers.get_l(),
                    0xAE => self.memory.read(self.registers.get_hl()),
                    0xEE => self.get_imm_1byte(cur_pc),
                    _ => panic!("Opcode: '{:X?}' seen within range 0xA8..0xAF but was not", cur_op),
                };
                // XOR the wanted register with A
                let res = v ^ self.registers.get_a();
                self.registers.set_a(res);

                // set flags
                self.registers.set_flag_zero( (res == 0) as u8);
                self.registers.set_flag_carry(0);
                self.registers.set_flag_half_carry(0);
                self.registers.set_flag_sub(0);
            }
            // OR n
            0xB0..=0xB7 | 0xF6 => {
                let v = match cur_op {
                    0xB0 => self.registers.get_b(),
                    0xB1 => self.registers.get_c(),
                    0xB2 => self.registers.get_d(),
                    0xB3 => self.registers.get_e(),
                    0xB4 => self.registers.get_h(),
                    0xB5 => self.registers.get_l(),
                    0xB6 => self.memory.read(self.registers.get_hl()),
                    0xB7 => self.registers.get_a(),
                    0xF6 => self.get_imm_1byte(cur_pc),
                    _ => panic!("Opcode: '{:X?}' seen within range 0xB0..0xB7 but was not", cur_op),
                };
                // OR the wanted register with A
                let res = v | self.registers.get_a();
                self.registers.set_a(res);

                // set flags
                self.registers.set_flag_zero( (res == 0) as u8);
                self.registers.set_flag_carry(0);
                self.registers.set_flag_half_carry(0);
                self.registers.set_flag_sub(0);
            },
            // CP n
            0xB8..=0xBF | 0xFE=> {
                let v = match cur_op {
                    0xB8 => self.registers.get_b(),
                    0xB9 => self.registers.get_c(),
                    0xBA => self.registers.get_d(),
                    0xBB => self.registers.get_e(),
                    0xBC => self.registers.get_h(),
                    0xBD => self.registers.get_l(),
                    0xBE => self.memory.read(self.registers.get_hl()),
                    0xBF => self.registers.get_a(),
                    0xFE => self.get_imm_1byte(cur_pc),
                    _ => panic!("Opcode '{}' landed in CP n branch arm", cur_op),
                };
                let a = self.registers.get_a();

                let res = a - v;

                self.registers.set_flag_zero((res == 0) as u8);
                self.registers.set_flag_sub(1);
                self.registers.set_flag_half_carry(half_carry_sub(a, v) as u8);
                self.registers.set_flag_carry(carry_sub(a, v) as u8);
            },
            // RET cc
            0xC0 | 0xC8 | 0xD0 | 0xD8 => {
                let is_jump = match cur_op {
                    0xC0 => !(self.registers.get_flag_zero() != 0),
                    0xC8 => self.registers.get_flag_zero() != 0,
                    0xD0 => !(self.registers.get_flag_carry() != 0),
                    0xD8 => self.registers.get_flag_carry() != 0,
                    _ => panic!("Opcode '{}' landed in RET cc match arm", cur_op),
                };

                if is_jump {
                    inc_pc = false;
                    // pop and move the stack
                    let addr = self.peek_stack();
                    self.registers.set_sp(self.registers.get_sp() + 2);

                    self.registers.set_pc(addr);
                }
            },
            // POP nn
            0xC1 | 0xD1 | 0xE1 | 0xF1 => {
                // get data at the top of the stack
                let data = self.peek_stack();
                // move the stack pointer back up
                self.registers.set_sp(self.registers.get_sp() + 2);

                // set wanted register to be popped data
                match cur_op {
                    0xC1 => self.registers.set_bc(data),
                    0xD1 => self.registers.set_de(data),
                    0xE1 => self.registers.set_hl(data),
                    0xF1 => self.registers.set_af(data),
                    _ => panic!("Opcode '{}' landed in match arm of POP nn", cur_op),
                }

            },
            // JP cc,nn
            0xC2 | 0xCA | 0xD2 | 0xDA => {
                let addr = self.get_imm_2byte(cur_pc);
                let is_jump = match cur_op {
                    0xC2 => !(self.registers.get_flag_zero() != 0),
                    0xCA => self.registers.get_flag_zero() != 0,
                    0xD2 => !(self.registers.get_flag_carry() != 0),
                    0xDA => self.registers.get_flag_carry() != 0,
                    _ => panic!("Opcode '{}' landed in JP cc,nn match arm", cur_op),
                };

                if is_jump {
                    inc_pc = false;
                    self.registers.set_pc(addr);
                }
            },
            // JP a16
            0xC3 => {
                // get two byte immediate value
                let addr = self.get_imm_2byte(cur_pc);

                self.registers.set_pc(addr);
                inc_pc = false;
            },
            // CALL cc,nn
            0xC4 | 0xCC | 0xD4 | 0xDC => {
                let addr = self.get_imm_2byte(cur_pc);
                let sp = self.registers.get_sp();
                let is_jump = match cur_op {
                    0xC4 => !(self.registers.get_flag_zero() != 0),
                    0xCC => self.registers.get_flag_zero() != 0,
                    0xD4 => !(self.registers.get_flag_carry() != 0),
                    0xDC => self.registers.get_flag_carry() != 0,
                    _ => panic!("Opcode '{}' landed in CALL cc,nn match arm", cur_op),
                };

                if is_jump {
                    inc_pc = false;
                    self.registers.set_pc(addr);
                    //TODO possibly write a push function?
                    // push memory next address onto stack
                    self.memory.write(sp, (addr & 0xFF) as u8);
                    self.memory.write(sp + 1, (addr >> 8) as u8);
                    self.registers.set_sp(sp + 2);
                }
            },
            // PUSH nn
            0xC5 | 0xD5 | 0xE5 | 0xF5 => {
                // push register pair onto stack, decrement stack by 2
                let cur_sp = self.registers.get_sp();

                let (high, low) = match cur_op {
                    0xC5 => (self.registers.get_b(), self.registers.get_c()),
                    0xD5 => (self.registers.get_d(), self.registers.get_e()),
                    0xE5 => (self.registers.get_h(), self.registers.get_l()),
                    0xF5 => (self.registers.get_a(), self.registers.get_f()),
                    _ => panic!("Opcode '{}' landed in PUSH nn match arm", cur_op),
                };

                // write upper byte to stack
                self.memory.write(cur_sp, high);
                // write lower byte to stack
                self.memory.write(cur_sp - 1, low);
                // decrement stack
                self.registers.set_sp(cur_sp - 2);
            },
            // RST n
            0xC7 | 0xCF | 0xD7 | 0xDF | 0xE7 | 0xEF | 0xF7 | 0xFF => {
                let addr = match cur_op {
                    0xC7 => 0x00,
                    0xCF => 0x08,
                    0xD7 => 0x10,
                    0xDF => 0x18,
                    0xE7 => 0x20,
                    0xEF => 0x28,
                    0xF7 => 0x30,
                    0xFF => 0x38,
                    _ => panic!("Opcode '{}' landed in RST n branch arm", {cur_op}),
                };
                let sp = self.registers.get_sp();
                //TODO possibly write a push function?
                // push current memory next address onto stack
                self.memory.write(sp, (cur_pc & 0xFF) as u8);
                self.memory.write(sp + 1, (cur_pc >> 8) as u8);

                self.registers.set_pc(addr);
                inc_pc = false;
            },
            // CALL nn
            0xCD => {
                let imm = self.get_imm_2byte(cur_pc);
                let next_addr = cur_pc + 1;
                let sp = self.registers.get_sp();

                //TODO possibly write a push function?
                // push memory next address onto stack
                self.memory.write(sp, (next_addr & 0xFF) as u8);
                self.memory.write(sp + 1, (next_addr >> 8) as u8);

                // jump to addr
                self.registers.set_pc(imm);
                inc_pc = false;
            },
            // RET
            0xC9 => {
                // pop two byte from the top of the stack
                let addr = self.peek_stack();
                self.registers.set_sp(self.registers.get_sp() + 2);

                // jump to the popped address
                inc_pc = false;
                self.registers.set_pc(addr);
            },
            // RETI
            0xD9 => {
                // pop address off of stack
                let addr = self.peek_stack();
                self.registers.set_sp(self.registers.get_sp() + 2);

                // jump to popped address
                inc_pc = false;
                self.registers.set_pc(addr);

                // enable interrupts
                self.ime = true;
            },
            // LDH (n),A
            0xE0 => {
                // put A into address $FF00+n
                let imm = self.memory.read(cur_pc + 1);
                self.memory.write(0xFF00 + imm as u16, self.registers.get_a());
            },
            // LD ($FF00+C),A
            0xE2 => {
                let addr = 0xFF00 + self.registers.get_c() as u16;
                self.memory.write(addr, self.registers.get_a());
            },
            // ADD SP,d8
            0xE8 => {
                let imm = self.memory.read(cur_pc + 1);
                let sp = self.registers.get_sp();

                // looks like carries are performed on lower bytes here
                self.registers.set_flag_half_carry(half_carry_add((sp & 0xFF) as u8, imm) as u8);
                self.registers.set_flag_carry(carry_add((sp & 0xFF) as u8, imm) as u8);
                self.registers.set_flag_zero(0);
                self.registers.set_flag_sub(0);
            },
            // JP (HL)
            0xE9 => {
                self.registers.set_pc(self.registers.get_hl());
                inc_pc = false;
            },
            // LD (nn),A
            0xEA => {
                let v = ( (self.memory.read(cur_pc + 1) as u16) << 8) | (self.memory.read(cur_pc + 2) as u16);
                self.memory.write(v, self.registers.get_a());
            },
            // LDH A,(n)
            0xF0 => {
                // put value at $FF00+n into A
                let imm = self.memory.read(cur_pc + 1);
                let v = self.memory.read(0xFF00 + imm as u16);
                self.registers.set_a(v);
            },
            // LD A,(C) (store value at 0xFF00 + register C into A
            0xF2 => {
                let v = self.memory.read(0xFF00 + self.registers.get_c() as u16);
                self.registers.set_a(v);
            },
            // DI
            0xF3 => self.ime = false, // TODO maybe change to enable after next instruction
            // LDHL SP,n
            0xF8 => {
                let imm = ((self.get_imm_1byte(cur_pc) as i8) as i16) as u16;
                let sp = self.registers.get_sp();

                let res = sp + imm;

                self.registers.set_sp(sp + imm);

                self.registers.set_flag_zero(0);
                self.registers.set_flag_sub(0);
                self.registers.set_flag_half_carry(half_carry_add((sp & 0xFF) as u8, (imm & 0xFF) as u8) as u8);
                self.registers.set_flag_carry(carry_add((sp & 0xFF) as u8, (imm & 0xFF) as u8) as u8);
            },
            // LD SP,HL
            0xF9 => self.registers.set_sp(self.registers.get_hl()),
            // EI
            0xFB => self.ime = true, // TODO maybe change to enable after next instruction
            _ => panic!("Unimplemented opcode: {:X?}", cur_op)
        }

        let cycles_passed = OP_CYCLES[cur_op as usize];

        //TODO possibly replace with table?
        // if we are supposed to increment the program counter, we do so
        if inc_pc {
            self.registers.set_pc(cur_pc + OP_LENGTHS[cur_op as usize] as u16);
        }

    }

    /// Returns the top two bytes of the stack as a u16. Doesn't move SP!
    fn peek_stack(&self) -> u16 {
        let cur_sp = self.registers.get_sp();
        // get least and most significant bytes.
        let ls = self.memory.read(cur_sp);
        let ms = self.memory.read(cur_sp + 1);
        ((ms as u16) << 8) | ((ls as u16) & 0xFF)
        //(self.memory.read(cur_pc + 1) as u16) << 8 | ((self.memory.read(cur_pc + 2) as u16) & 0xFF)
    }

    //TODO possibly remove the pc param if nothing modifies it in the match
    /// Returns the one byte immediate value following the given PC. Doesn't move PC!
    fn get_imm_1byte(&self, pc: u16) -> u8 {
        self.memory.read(pc + 1)
    }

    /// Returns the two byte immediate value following the given PC. Doesn't move the PC!
    fn get_imm_2byte(&self, pc: u16) -> u16 {
        // get least and most significant bytes.
        let ls = self.memory.read(pc);
        let ms = self.memory.read(pc + 1);
        ((ms as u16) << 8) | ((ls as u16) & 0xFF)
    }


}

#[cfg(test)]
mod tests {
    use super::*;

    fn before_each() -> CPU{
        CPU::new()
    }

    #[test]
    fn add() {
        let mut cpu = before_each();
        // set up registers
        cpu.registers.set_pc(0u16);
        cpu.registers.set_a(40u8);
        cpu.registers.set_b(19u8);
        cpu.registers.set_c(250u8);
        cpu.registers.set_d(179u8);

        // load ADD A,B
        cpu.memory.write(0, 0x80);
        // load ADD A,24
        cpu.memory.write(1,0xC6);
        cpu.memory.write(2, 24);
        cpu.memory.write(3, 0x81);
        cpu.memory.write(4, 0x82);

        // test ADD A,B
        cpu.exec();
        assert_eq!(cpu.registers.get_a(), 59);

        assert_eq!(cpu.registers.get_flag_zero(), 0);
        assert_eq!(cpu.registers.get_flag_sub(), 0);
        assert_eq!(cpu.registers.get_flag_half_carry(), 0);
        assert_eq!(cpu.registers.get_flag_carry(), 0);

        // test ADD A,24
        cpu.exec();
        assert_eq!(cpu.registers.get_a(), 83);
        assert_eq!(cpu.registers.get_flag_zero(), 0);
        assert_eq!(cpu.registers.get_flag_sub(), 0);
        assert_eq!(cpu.registers.get_flag_half_carry(), 1);
        assert_eq!(cpu.registers.get_flag_carry(), 0);

        // test ADD A,C
        cpu.exec();
        assert_eq!(cpu.registers.get_a(), 77);
        assert_eq!(cpu.registers.get_flag_zero(), 0);
        assert_eq!(cpu.registers.get_flag_sub(), 0);
        assert_eq!(cpu.registers.get_flag_half_carry(), 0);
        assert_eq!(cpu.registers.get_flag_carry(), 1);

        // test ADD A,D
        cpu.exec();
        assert_eq!(cpu.registers.get_a(), 0);
        assert_eq!(cpu.registers.get_flag_zero(), 1);
        assert_eq!(cpu.registers.get_flag_sub(), 0);
        assert_eq!(cpu.registers.get_flag_half_carry(), 1);
        assert_eq!(cpu.registers.get_flag_carry(), 1);

    }

    #[test]
    fn sub() {

    }
}