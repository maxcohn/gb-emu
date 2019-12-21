
/// A set of registers
pub struct Registers {
    a: u8,
    f: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    sp: u16,
    pc: u16,
}

const ZERO_FLAG: u8 = 0b1000_0000;
const SUB_FLAG: u8 = 0b0100_0000;
const HALF_CARRY_FLAG: u8 = 0b0010_0000;
const CARRY_FLAG: u8 = 0b0001_0000;

impl Registers {
    /// Create a new Register struct with default SP and PC
    pub fn new() -> Registers {
        Registers {
            a: 0,
            f: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            sp: 0xfffe, // default for SP, but should not be relied upon.
            pc: 0x100, // All GB programs start at 0x100
        }
    }

    /// Print the contents of all registers to stdout
    pub fn print_registers(&self) {
        println!("a: 0x{:X?}", self.a);
        println!("f: 0x{:X?}", self.f);
        println!("b: 0x{:X?}", self.b);
        println!("c: 0x{:X?}", self.c);
        println!("d: 0x{:X?}", self.d);
        println!("e: 0x{:X?}", self.e);
        println!("h: 0x{:X?}", self.h);
        println!("l: 0x{:X?}", self.l);
        println!("sp: 0x{:X?}", self.sp);
        println!("pc: 0x{:X?}", self.pc);
    }

    ///////////////////////////////////////////////////////////////////////////
    // Get and set 16 bit registers
    ///////////////////////////////////////////////////////////////////////////
    pub fn set_af(&mut self, value: u16) {
        self.a = ((value & 0xFF00) >> 8) as u8;
        self.f = (value & 0x00FF) as u8;
    }

    pub fn set_bc(&mut self, value: u16) {
        self.b = ((value & 0xFF00) >> 8) as u8;
        self.c = (value & 0x00FF) as u8;
    }

    pub fn set_de(&mut self, value: u16) {
        self.d = ((value & 0xFF00) >> 8) as u8;
        self.e = (value & 0x00FF) as u8;
    }

    pub fn set_hl(&mut self, value: u16) {
        self.h = ((value & 0xFF00) >> 8) as u8;
        self.l = (value & 0x00FF) as u8;
    }

    pub fn get_af(&self) -> u16 {
        ((self.a as u16) << 8) | (self.f as u16)
    }

    pub fn get_bc(&self) -> u16 {
        ((self.b as u16) << 8) | (self.c as u16)
    }

    pub fn get_de(&self) -> u16 {
        ((self.d as u16) << 8) | (self.e as u16)
    }

    pub fn get_hl(&self) -> u16 {
        ((self.h as u16) << 8) | (self.l as u16)
    }

    ///////////////////////////////////////////////////////////////////////////
    // Get and set individual registers
    ///////////////////////////////////////////////////////////////////////////

    pub fn set_a(&mut self, value: u8) {
        self.a = value;
    }

    pub fn set_b(&mut self, value: u8) {
        self.b = value;
    }

    pub fn set_c(&mut self, value: u8) {
        self.c = value;
    }

    pub fn set_d(&mut self, value: u8) {
        self.d = value;
    }

    pub fn set_e(&mut self, value: u8) {
        self.e = value;
    }

    pub fn set_h(&mut self, value: u8) {
        self.h = value;
    }

    pub fn set_l(&mut self, value: u8) {
        self.l = value;
    }
    
    pub fn get_a(&self) -> u8 {
        self.a
    }

    pub fn get_b(&self) -> u8 {
        self.b
    }

    pub fn get_c(&self) -> u8 {
        self.c
    }

    pub fn get_d(&self) -> u8 {
        self.d
    }

    pub fn get_e(&self) -> u8 {
        self.e
    }

    pub fn get_h(&self) -> u8 {
        self.h
    }

    pub fn get_l(&self) -> u8 {
        self.l
    }

    ///////////////////////////////////////////////////////////////////////////
    // Get and set flag register
    ///////////////////////////////////////////////////////////////////////////
    pub fn get_flag_zero(&self) -> u8 {
        (self.f & ZERO_FLAG) >> 7
    }

    pub fn get_flag_sub(&self) -> u8 {
        (self.f & SUB_FLAG) >> 6
    }

    pub fn get_flag_half_carry(&self) -> u8 {
        (self.f & HALF_CARRY_FLAG) >> 5
    }

    pub fn get_flag_carry(&self) -> u8 {
        (self.f & CARRY_FLAG) >> 4
    }

    pub fn set_flag_zero(&mut self, f: u8) {
        if f == 0 {
            // set flag to 0
            self.f &= 0b0 << 7
        } else {
            // set flag to 1
            self.f |= ZERO_FLAG
        }
    }

    pub fn set_flag_sub(&mut self, f: u8) {
        if f == 0 {
            // set flag to 0
            self.f &= 0b0 << 6
        } else {
            // set flag to 1
            self.f |= SUB_FLAG
        }
    }

    pub fn set_flag_half_carry(&mut self, f: u8) {

        if f == 0 {
            // set flag to 0
            self.f &= 0b0 << 5
        } else {
            // set flag to 1
            self.f |= HALF_CARRY_FLAG;
        }

    }

    pub fn set_flag_carry(&mut self, f: u8) {
        if f == 0 {
            // set flag to 0
            self.f &= 0b0 << 4
        } else {
            // set flag to 1
            self.f |= CARRY_FLAG;
        }
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
        let r = create_registers();

        assert_eq!(((r.a as u16) << 8) | (r.f as u16), r.get_af());
        assert_eq!(((r.b as u16) << 8) | (r.c as u16), r.get_bc());
        assert_eq!(((r.d as u16) << 8) | (r.e as u16), r.get_de());
        assert_eq!(((r.h as u16) << 8) | (r.l as u16), r.get_hl());

        // check against raw values
        assert_eq!(0x1060, r.get_af());
        assert_eq!(0x2030, r.get_bc());
        assert_eq!(0x4050, r.get_de());
        assert_eq!(0x7080, r.get_hl());
    }

    #[test]
    fn test_get_8bit() {
        let r = create_registers();

        assert_eq!(r.a, r.get_a());
        assert_eq!(r.b, r.get_b());
        assert_eq!(r.c, r.get_c());
        assert_eq!(r.d, r.get_d());
        assert_eq!(r.e, r.get_e());
        //assert_eq!(r.f, r.get_f());
        assert_eq!(r.h, r.get_h());
        assert_eq!(r.l, r.get_l());

    }

    #[test]
    fn test_set_16bit() {
        let mut r = create_registers();
        r.set_af(0x1020);
        r.set_bc(0x2030);
        r.set_de(0x4050);
        r.set_hl(0x6070);

        assert_eq!(r.get_af(), 0x1020);
        assert_eq!(r.get_bc(), 0x2030);
        assert_eq!(r.get_de(), 0x4050);
        assert_eq!(r.get_hl(), 0x6070);

        //assert_eq!(add(1, 2), 3);
    }

    #[test]
    fn test_set_8bit() {
        let mut r = create_registers();
        r.set_a(0x11);
        r.set_b(0x22);
        r.set_c(0x33);
        r.set_d(0x44);
        r.set_e(0x55);
        //r.set_f(0x66); // f register can't be accessed directly
        r.set_h(0x77);
        r.set_l(0x88);

        assert_eq!(r.get_a(), 0x11);
        assert_eq!(r.get_b(), 0x22);
        assert_eq!(r.get_c(), 0x33);
        assert_eq!(r.get_d(), 0x44);
        assert_eq!(r.get_e(), 0x55);
        //assert_eq!(r.get_f(), 0x66);
        assert_eq!(r.get_h(), 0x77);
        assert_eq!(r.get_l(), 0x88);
        
        
        //assert_eq!(add(1, 2), 3);
    }

    #[test]
    fn test_get_flags() {
        let mut r = create_registers();

        r.f = 0b1011_0000;

        assert_eq!(r.get_flag_zero(), 0b1);
        assert_eq!(r.get_flag_sub(), 0b0);
        assert_eq!(r.get_flag_half_carry(), 0b1);
        assert_eq!(r.get_flag_carry(), 0b1);
    }

    #[test]
    fn test_set_flags() {
        let mut r = create_registers();

        r.set_flag_zero(0);
        r.set_flag_sub(0);
        r.set_flag_half_carry(0);
        r.set_flag_carry(0);

        assert_eq!(r.get_flag_zero(), 0);
        assert_eq!(r.get_flag_sub(), 0);
        assert_eq!(r.get_flag_half_carry(), 0);
        assert_eq!(r.get_flag_carry(), 0);


        r.set_flag_zero(1);
        r.set_flag_sub(1);
        r.set_flag_half_carry(1);
        r.set_flag_carry(1);

        assert_eq!(r.get_flag_zero(), 1);
        assert_eq!(r.get_flag_sub(), 1);
        assert_eq!(r.get_flag_half_carry(), 1);
        assert_eq!(r.get_flag_carry(), 1);
    }
}
