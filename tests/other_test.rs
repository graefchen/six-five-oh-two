use sixfiveohtwo::chip::*;

/// ==========================
/// OTHER TESTS
/// ==========================

#[cfg(test)]
mod bit_test {
    use crate::*;

    #[test]
    fn zeropage_addressing() {
        let mut c = Chip::new();

        // Code:
        // BIT $80
        let prog: Vec<u8> = [0x24, 0x80].to_vec();
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(c.memory[0x80], 0x0);
    }

    #[test]
    fn absolute_addressing() {
        let mut c = Chip::new();

        // Code:
        // BIT $3010
        let prog: Vec<u8> = [0x2C, 0x10, 0x30].to_vec();
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(c.memory[0x3010], 0x0);
    }

    #[test]
    fn flags() {
        let mut c = Chip::new();

        // Code:
        // BIT $80
        let prog: Vec<u8> = [0x24, 0x80].to_vec();
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(c.f, 0x0);
    }
}

#[cfg(test)]
mod no_operation {
    use crate::*;

    #[test]
    fn implied_addressing() {
        let mut c = Chip::new();

        // Code:
        // NOP
        // NOP
        // NOP
        // NOP
        // NOP
        let prog: Vec<u8> = [0xEA, 0xEA, 0xEA, 0xEA, 0xEA].to_vec();
        c.load_program(prog);

        c.execute_cycle();
        c.execute_cycle();
        c.execute_cycle();
        c.execute_cycle();
        c.execute_cycle();
        assert_eq!(0x205, c.pc);
    }
}
