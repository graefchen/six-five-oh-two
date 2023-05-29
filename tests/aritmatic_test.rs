use sixfiveohtwo::chip::*;

/// ==========================
/// ARITHMETIC OPERATION TESTS
/// ==========================

// TODO: Making these tests...

#[cfg(test)]
mod add_with_carry {
    use crate::*;

    #[test]
    fn add_00000000_to_01111111_with_carry() {
        let mut c = Chip::new();

        // Code:
        // ADC #$00
        let prog: Vec<u8> = [0x69, 0x00].to_vec();
        c.startup(0x0200);
        c.load_program(prog);
        c.acc = 0b01111111;
        c.f = C;

        c.execute_cycle();
        // assert_eq!(c.acc, 0b10000000);
        assert_eq!(c.f, 0b11000000);
    }

    #[test]
    fn add_00000000_to_00111111_with_carry() {
        let mut c = Chip::new();

        // Code:
        // ADC #$00
        let prog: Vec<u8> = [0x69, 0x00].to_vec();
        c.startup(0x0200);
        c.load_program(prog);
        c.acc = 0b00111111;
        c.f = C;

        c.execute_cycle();
        assert_eq!(c.acc, 0b01000000);
        assert_eq!(c.f, 0b00000000);
    }
}

#[cfg(test)]
mod subtract_with_carry {
    use crate::*;

    #[test]
    fn subtract_11111111_from_00111111_with_carry() {
        let mut c = Chip::new();

        // Code:
        // SBC #$FF
        let prog: Vec<u8> = [0xE9, 0xFF].to_vec();
        c.startup(0x0200);
        c.load_program(prog);
        c.acc = 0b00111111;
        c.f = C;

        c.execute_cycle();
        assert_eq!(c.acc, 0b01000000);
        assert_eq!(c.f, 0b00000000);
    }

    #[test]
    fn subtract_11111111_from_01111111_with_carry() {
        let mut c = Chip::new();

        // Code:
        // SBC #$FF
        let prog: Vec<u8> = [0xE9, 0xFF].to_vec();
        c.startup(0x0200);
        c.load_program(prog);
        c.acc = 0b01111111;
        c.f = C;

        c.execute_cycle();
        assert_eq!(c.acc, 0b10000000);
        assert_eq!(c.f, 0b11000000, "Flag incorrect!");
    }

    #[test]
    fn subtract_11111110_from_11111111_without_carry() {
        let mut c = Chip::new();

        // Code:
        // SBC #$FF
        let prog: Vec<u8> = [0xE9, 0b11111110].to_vec();
        c.startup(0x0200);
        c.load_program(prog);
        c.acc = 0b11111111;
        c.f = 0;

        c.execute_cycle();
        assert_eq!(c.acc, 0b00000000);
        assert_eq!(c.f, 0b00000011, "Flag incorrect!");
    }
}
