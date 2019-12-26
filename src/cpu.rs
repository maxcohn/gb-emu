use crate::registers::Registers;
use crate::memory::Memory;

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

/// Checks if the addition of two number results in a half carry
fn half_carry_add(a: u8, b: u8) -> bool {
    (((a & 0xf) + (b & 0xf)) & 0x10) == 0x10
}
//TODO implement 16bit half carry

/// CPU and it's components: registers
//TODO remove `pub` after done testing
pub struct CPU {
    pub registers: Registers,
    pub memory: Memory,
}

impl CPU {
    /// Create a new CPU struct
    pub fn new(mem: Memory) -> CPU {
        CPU {
            registers: Registers::new(),
            memory: mem,//Memory::new(),
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

        match cur_op {
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
            // INC B
            0x04 => {
                // get original value and then calculate new value
                let v = self.registers.get_b();
                let res = v + 1;

                // set register to incremented version
                self.registers.set_b(res);

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
            // LD (nn),SP
            0x08 => {
                let v = ( (self.memory.read(cur_pc + 1) as u16) << 8) | (self.memory.read(cur_pc + 2) as u16);
                self.registers.set_sp(v);
            },
            // DEC BC
            0x0B => self.registers.set_bc(self.registers.get_bc() - 1),
            // INC C
            0x0C => {
                // get original value and then calculate new value
                let v = self.registers.get_c();
                let res = v + 1;

                // set register to incremented version
                self.registers.set_c(res);

                // set flags
                self.registers.set_flag_sub(0);
                self.registers.set_flag_half_carry(half_carry_add(v, res) as u8);
                self.registers.set_flag_zero((res == 0) as u8);
            },
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
            // INC D
            0x14 => {
                // get original value and then calculate new value
                let v = self.registers.get_d();
                let res = v + 1;

                // set register to incremented version
                self.registers.set_d(res);

                // set flags
                self.registers.set_flag_sub(0);
                self.registers.set_flag_half_carry(half_carry_add(v, res) as u8);
                self.registers.set_flag_zero((res == 0) as u8);
            },
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
            // DEC DE
            0x1B => self.registers.set_de(self.registers.get_de() - 1),
            // INC E
            0x1C => {
                // get original value and then calculate new value
                let v = self.registers.get_e();
                let res = v + 1;

                // set register to incremented version
                self.registers.set_e(res);

                // set flags
                self.registers.set_flag_sub(0);
                self.registers.set_flag_half_carry(half_carry_add(v, res) as u8);
                self.registers.set_flag_zero((res == 0) as u8);
            },
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
            // LD HL,nn
            0x21 => {
                let v = ( (self.memory.read(cur_pc + 1) as u16) << 8) | (self.memory.read(cur_pc + 2) as u16);
                self.registers.set_HL(v);
            },
            // LDI (HL),A - LD (HL+),A - LD (HLI),A
            0x22 => {
                // store A into addr HL, increment HL
                self.memory.write(self.registers.get_hl(), self.registers.get_a());
                self.registers.set_hl(self.registers.get_hl() + 1);
            },
            // INC HL
            0x23 => self.registers.set_hl(self.registers.get_hl() + 1),
            // INC H
            0x24 => {
                // get original value and then calculate new value
                let v = self.registers.get_h();
                let res = v + 1;

                // set register to incremented version
                self.registers.set_h(res);

                // set flags
                self.registers.set_flag_sub(0);
                self.registers.set_flag_half_carry(half_carry_add(v, res) as u8);
                self.registers.set_flag_zero((res == 0) as u8);
            },
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
            // LDI A,(HL) - LD A,(HL+) - LD A,(HLI)
            0x2A => {
                // put value at addr HL into A, increment HL
                let v = self.memory.read(self.registers.get_hl());
                self.registers.set_a(v);
                self.registers.set_hl(self.registers.get_hl() + 1);
            },
            // DEC HL
            0x2B => self.registers.set_hl(self.registers.get_hl() - 1),
            // INC L
            0x2C => {
                // get original value and then calculate new value
                let v = self.registers.get_l();
                let res = v + 1;

                // set register to incremented version
                self.registers.set_l(res);

                // set flags
                self.registers.set_flag_sub(0);
                self.registers.set_flag_half_carry(half_carry_add(v, res) as u8);
                self.registers.set_flag_zero((res == 0) as u8);
            },
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
            // INC (HL)
            0x34 => {
                // get original value and then calculate new value
                let v = self.memory.read(self.registers.get_hl());
                let res = v + 1;

                // set register to incremented version
                self.memory.write(self.registers.get_hl(), res);

                // set flags
                self.registers.set_flag_sub(0);
                self.registers.set_flag_half_carry(half_carry_add(v, res) as u8);
                self.registers.set_flag_zero((res == 0) as u8);
            },
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
            // LDD A,(HL) - LD A,(HL-) - LD A,(HLD)
            0x3A => {
                // store value at address HL into A, decrement HL
                let v = self.memory.read(self.registers.get_hl());
                self.registers.set_a(v);
                self.registers.set_hl(self.registers.get_hl() - 1);
            },
            // DEC sp
            0x3B => self.registers.set_sp(self.registers.get_sp() - 1),
            // INC A
            0x3C => {
                // get original value and then calculate new value
                let v = self.registers.get_a();
                let res = v + 1;

                // set register to incremented version
                self.registers.set_a(res);

                // set flags
                self.registers.set_flag_sub(0);
                self.registers.set_flag_half_carry(half_carry_add(v, res) as u8);
                self.registers.set_flag_zero((res == 0) as u8);
            },
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
            // LD B,n
            0x40..0x46 => {
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
            // LD B,A
            0x47 => self.registers.set_b(self.registers.get_a()),
            // LD C,n
            0x48..0x4E => {
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
            // LD C,A
            0x4F => self.registers.set_c(self.registers.get_a()),
            // LD D,n
            0x50..0x56 => {
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
            // LD D,A
            0x57 => self.registers.set_d(self.registers.get_a()),
            // LD E,n
            0x58..0x5E => {
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
            // LD E,A
            0x5F => self.registers.set_e(self.registers.get_a()),
            // LD H,n
            0x60..0x66 => {
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
            // LD H,A
            0x67 => self.registers.set_h(self.registers.get_a()),
            // LD L,n
            0x68..0x6E => {
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
            // LD L,A
            0x6F => self.registers.set_l(self.registers.get_a()),
            // LD (HL),n
            0x70..0x75 => {
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
            }
            // LD (HL),A
            0x77 => {
                let v = self.registers.get_hl();
                self.memory.write(v, self.registers.get_a());
            }
            // LD A,n
            0x78..0x7F => {
                let v = match cur_op {
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
            // LD A,A (doesn't seem necessary, but ok)
            0x7F => self.registers.set_a(self.registers.get_a()),
            // AND n
            0xA0..0xA7 => {
                let v = match cur_op {
                    0xA0 => self.registers.get_b(),
                    0xA1 => self.registers.get_c(),
                    0xA2 => self.registers.get_d(),
                    0xA3 => self.registers.get_e(),
                    0xA4 => self.registers.get_h(),
                    0xA5 => self.registers.get_l(),
                    0xA6 => self.memory.read(self.registers.get_hl()),
                    0xA7 => self.registers.get_a(),
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
            0xA8..0xAF => {
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
            0xB0..0xB7 => {
                let v = match cur_op {
                    0xB0 => self.registers.get_b(),
                    0xB1 => self.registers.get_c(),
                    0xB2 => self.registers.get_d(),
                    0xB3 => self.registers.get_e(),
                    0xB4 => self.registers.get_h(),
                    0xB5 => self.registers.get_l(),
                    0xB6 => self.memory.read(self.registers.get_hl()),
                    0xB7 => self.registers.get_a(),
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
            }
            // POP BC
            0xC1 => {
                let cur_sp = self.registers.get_sp();
                let high = self.memory.read(cur_sp);
                let low = self.memory.read(cur_sp + 1);
                self.registers.set_bc( ((high as u16) << 8) | (low as u16) );
                self.registers.set_sp(cur_sp + 2);
            },
            // PUSH BC
            0xC5 => {
                // push register pair onto stack, decrement stack by 2
                let cur_sp = self.registers.get_sp();

                // write upper byte to stack
                self.memory.write(cur_sp, self.registers.get_b());
                // write lower byte to stack
                self.memory.write(cur_sp - 1, self.registers.get_c());
                // decrement stack
                self.registers.set_sp(cur_sp - 2);
            },
            // POP DE
            0xD1 => {
                let cur_sp = self.registers.get_sp();
                let high = self.memory.read(cur_sp);
                let low = self.memory.read(cur_sp + 1);
                self.registers.set_de( ((high as u16) << 8) | (low as u16) );
                self.registers.set_sp(cur_sp + 2);
            },
            // PUSH DE
            0xD5 => {
                // push register pair onto stack, decrement stack by 2
                let cur_sp = self.registers.get_sp();

                // write upper byte to stack
                self.memory.write(cur_sp, self.registers.get_d());
                // write lower byte to stack
                self.memory.write(cur_sp - 1, self.registers.get_e());
                // decrement stack
                self.registers.set_sp(cur_sp - 2);
            },
            // LDH (n),A
            0xE0 => {
                // put A into address $FF00+n
                let imm = self.memory.read(cur_pc + 1);
                self.memory.write(0xFF00 + imm as u16, self.registers.get_a());
            },
            // POP HL
            0xE1 => {
                let cur_sp = self.registers.get_sp();
                let high = self.memory.read(cur_sp);
                let low = self.memory.read(cur_sp + 1);
                self.registers.set_hl( ((high as u16) << 8) | (low as u16) );
                self.registers.set_sp(cur_sp + 2);
            }
            // LD ($FF00+C),A
            0xE2 => {
                let addr = (0xFF00 + self.registers.get_c()) as u16;
                self.memory.write(addr, self.registers.get_a());
            },
            // PUSH HL
            0xE5 => {
                // push register pair onto stack, decrement stack by 2
                let cur_sp = self.registers.get_sp();

                // write upper byte to stack
                self.memory.write(cur_sp, self.registers.get_h());
                // write lower byte to stack
                self.memory.write(cur_sp - 1, self.registers.get_l());
                // decrement stack
                self.registers.set_sp(cur_sp - 2);
            },
            // AND d8
            0xE6 => {
                let imm = self.memory.read(cur_pc + 1);
                // AND the immediate value with register A
                let res = self.registers.get_a() & imm;
                self.registers.set_a(res);

                // set flags
                self.registers.set_flag_zero( (res == 0) as u8);
                self.registers.set_flag_carry(0);
                self.registers.set_flag_half_carry(1);
                self.registers.set_flag_sub(0);
            }
            // LD (nn),A
            0xEA => {
                let v = ( (self.memory.read(cur_pc + 1) as u16) << 8) | (self.memory.read(cur_pc + 2) as u16);
                self.memory.write(v, self.registers.get_a());
            },
            // XOR d8
            0xEE => {
                let imm = self.memory.read(cur_pc + 1);
                // XOR the immediate value with register A
                let res = self.registers.get_a() ^ imm;
                self.registers.set_a(res);

                // set flags
                self.registers.set_flag_zero( (res == 0) as u8);
                self.registers.set_flag_carry(0);
                self.registers.set_flag_half_carry(0);
                self.registers.set_flag_sub(0);
            }
            // LDH A,(n)
            0xF0 => {
                // put value at $FF00+n into A
                let imm = self.memory.read(cur_pc + 1);
                let v = self.memory.read(0xFF00 + imm as u16);
                self.registers.set_a(v);
            },
            // POP AF
            0xF1 => {
                let cur_sp = self.registers.get_sp();
                let high = self.memory.read(cur_sp);
                let low = self.memory.read(cur_sp + 1);
                self.registers.set_af( ((high as u16) << 8) | (low as u16) );
                self.registers.set_sp(cur_sp + 2);
            },
            // LD A,(C) (store value at 0xFF00 + register C into A
            0xF2 => {
                let v = self.memory.read((0xFF00 + self.registers.get_c()) as u16);
                self.registers.set_a(v);
            },
            // PUSH AF
            0xF5 => {
                // push register pair onto stack, decrement stack by 2
                let cur_sp = self.registers.get_sp();

                // write upper byte to stack
                self.memory.write(cur_sp, self.registers.get_a());
                // write lower byte to stack
                self.memory.write(cur_sp - 1, self.registers.get_f());
                // decrement stack
                self.registers.set_sp(cur_sp - 2);
            },
            // OR d8
            0xF6 => {
                let imm = self.memory.read(cur_pc + 1);
                // OR the immediate value with register A
                let res = self.registers.get_a() | imm;
                self.registers.set_a(res);

                // set flags
                self.registers.set_flag_zero( (res == 0) as u8);
                self.registers.set_flag_carry(0);
                self.registers.set_flag_half_carry(0);
                self.registers.set_flag_sub(0);
            }
            // LD SP,HL
            0xF9 => self.registers.set_sp(self.registers.get_hl()),

            _ => panic!("Unimplemented opcode: {:X?}", cur_op)
        }

        let cycles_passed = OP_CYCLES[cur_op as usize];


    }
}
