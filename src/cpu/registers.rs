

struct Registers {
    pub a: u8,
    pub f: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
    pub pc: u16,
}

const ZERO_FLAG: u8 = 0b1000_0000;
const SUB_FLAG: u8 = 0b0100_0000;
const HALF_CARRY_FLAG: u8 = 0b0010_0000;
const CARRY_FLAG: u8 = 0b0001_0000;

impl Registers {
    fn new() -> Registers {
        Registers {
            a: 0,
            f: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            sp: 0,
            pc: 0x100, // All GB programs start at 0x100
        }
    }

    ///////////////////////////////////////////////////////////////////////////
    // Get and set 16 bit registers
    ///////////////////////////////////////////////////////////////////////////
    fn set_af(&mut self, value: u16) {
        self.a = ((value & 0xFF00) >> 8) as u8;
        self.c = (value & 0x00FF) as u8;
    }

    fn set_bc(&mut self, value: u16) {
        self.b = ((value & 0xFF00) >> 8) as u8;
        self.c = (value & 0x00FF) as u8;
    }

    fn set_de(&mut self, value: u16) {
        self.d = ((value & 0xFF00) >> 8) as u8;
        self.e = (value & 0x00FF) as u8;
    }

    fn set_hl(&mut self, value: u16) {
        self.h = ((value & 0xFF00) >> 8) as u8;
        self.l = (value & 0x00FF) as u8;
    }

    fn get_af(&self) -> u16 {
        ((self.a as u16) << 8) | (self.f as u16)
    }

    fn get_bc(&self) -> u16 {
        ((self.b as u16) << 8) | (self.c as u16)
    }

    fn get_de(&self) -> u16 {
        ((self.d as u16) << 8) | (self.e as u16)
    }

    fn get_hl(&self) -> u16 {
        ((self.h as u16) << 8) | (self.l as u16)
    }

    ///////////////////////////////////////////////////////////////////////////
    // Get and set individual registers
    ///////////////////////////////////////////////////////////////////////////

    fn set_a(&mut self, value: u8) {
        self.a = value;
    }

    fn set_b(&mut self, value: u8) {
        self.b = value;
    }

    fn set_c(&mut self, value: u8) {
        self.c = value;
    }

    fn set_d(&mut self, value: u8) {
        self.d = value;
    }

    fn set_e(&mut self, value: u8) {
        self.e = value;
    }

    fn set_h(&mut self, value: u8) {
        self.h = value;
    }

    fn set_l(&mut self, value: u8) {
        self.l = value;
    }
    
    fn get_a(&self) -> u8 {
        self.a
    }

    fn get_b(&self) -> u8 {
        self.b
    }

    fn get_c(&self) -> u8 {
        self.c
    }

    fn get_d(&self) -> u8 {
        self.d
    }

    fn get_e(&self) -> u8 {
        self.e
    }

    fn get_h(&self) -> u8 {
        self.h
    }

    fn get_l(&self) -> u8 {
        self.l
    }

    ///////////////////////////////////////////////////////////////////////////
    // Get and set flag register
    ///////////////////////////////////////////////////////////////////////////
    fn get_flag_zero(&self) -> u8 {
        (self.f & ZERO_FLAG) >> 7
    }

    fn get_flag_sub(&self) -> u8 {
        (self.f & SUB_FLAG) >> 6
    }

    fn get_flag_half_carry(&self) -> u8 {
        (self.f & HALF_CARRY_FLAG) >> 5
    }

    fn get_flag_carry(&self) -> u8 {
        (self.f & CARRY_FLAG) >> 4
    }

    fn set_flag_zero(&mut self) {
        self.f |= ZERO_FLAG;
    }

    fn set_flag_sub(&mut self) {
        self.f |= SUB_FLAG;
    }

    fn set_flag_half_carry(&mut self) {
        self.f |= HALF_CARRY_FLAG;
    }

    fn set_flag_carry(&mut self) {
        self.f |= CARRY_FLAG;
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    fn create_registers() -> Registers {
        let mut r = Registers::new();

        r.a = 0x10;
        r.b = 0x20;
        r.c = 0x30;
        r.d = 0x40;
        r.e = 0x50;
        r.f = 0x60;
        r.h = 0x70;
        r.l = 0x80;
        r.pc = 0x0100;
        r.sp = 0xE000;

        return r;
    }

    #[test]
    fn test_get_16bit() {
        let mut r = create_registers();

        assert_eq!(((r.a as u16) << 8) | (r.f as u16), r.get_af());
        assert_eq!(((r.b as u16) << 8) | (r.c as u16), r.get_bc());
        assert_eq!(((r.d as u16) << 8) | (r.e as u16), r.get_de());
        assert_eq!(((r.h as u16) << 8) | (r.l as u16), r.get_hl());
    }

    #[test]
    #[ignore]
    fn test_get_8bit() {
        //assert_eq!(add(1, 2), 3);
    }

    #[test]
    #[ignore]
    fn test_set_16bit() {
        //assert_eq!(add(1, 2), 3);
    }

    #[test]
    #[ignore]
    fn test_set_8bit() {
        //assert_eq!(add(1, 2), 3);
    }

    #[test]
    #[ignore]
    fn test_get_flags() {

    }

    #[test]
    #[ignore]
    fn test_set_flags() {

    }
}