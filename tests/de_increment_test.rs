use sixfiveohtwo::chip::*;

/// ==============================
/// DECREMENT & INCREMENT TESTS
/// ==============================

#[cfg(test)]
mod decrement {
    use crate::*;

    #[test]
    fn zeropage_addressing() {
        let mut c = Chip::new();

        // Code:
        // DEC $01
        let prog: Vec<u8> = [0xC6, 0x01].to_vec();
        c.memory[0x01] = 0x12;
        c.load_program(prog);
        c.startup(0x0200);
        c.startup(0x0200);

        c.execute_cycle();
        assert_eq!(c.memory[0x01], 0x11);
    }

    #[test]
    fn zeropage_x_addressing() {
        let mut c = Chip::new();

        // Code:
        // DEC $01,X
        let prog: Vec<u8> = [0xD6, 0x01].to_vec();
        c.rx = 0x01;
        c.memory[0x02] = 0x13;
        c.load_program(prog);
        c.startup(0x0200);

        c.execute_cycle();
        assert_eq!(c.memory[0x02], 0x12);
    }

    #[test]
    fn absolute_addressing() {
        let mut c = Chip::new();

        // Code:
        // DEC $FFF0
        let prog: Vec<u8> = [0xCE, 0xF0, 0xFF].to_vec();
        c.memory[0xFFF0] = 0xAA;
        c.load_program(prog);
        c.startup(0x0200);

        c.execute_cycle();
        assert_eq!(c.memory[0xFFF0], 0xA9);
    }

    #[test]
    fn absolute_x_addressing() {
        let mut c = Chip::new();

        // Code:
        // DEC $3120,X
        let prog: Vec<u8> = [0xDE, 0x20, 0x31].to_vec();
        c.rx = 0x12;
        c.memory[0x3132] = 0xAA;
        c.load_program(prog);
        c.startup(0x0200);

        c.execute_cycle();
        assert_eq!(c.memory[0x3132], 0xA9);
    }

    #[test]
    fn flag() {
        let mut c = Chip::new();

        // Code:
        // DEC $01
        let prog: Vec<u8> = [0xC6, 0x01].to_vec();
        c.memory[0x01] = 0x01;
        c.load_program(prog);
        c.startup(0x0200);

        c.execute_cycle();
        assert_eq!(Z, c.f);
    }
}

#[cfg(test)]
mod decrement_x {
    use crate::*;

    #[test]
    fn implied_addressing() {
        let mut c = Chip::new();

        // Code:
        // DEX
        let prog: Vec<u8> = [0xCA].to_vec();
        c.rx = 0x01;
        c.load_program(prog);
        c.startup(0x0200);

        c.execute_cycle();
        assert_eq!(c.rx, 0x00);
    }

    #[test]
    fn flag() {
        let mut c = Chip::new();

        // Code:
        // DEX
        let prog: Vec<u8> = [0xCA].to_vec();
        c.rx = 0x01;
        c.load_program(prog);
        c.startup(0x0200);

        c.execute_cycle();
        assert_eq!(Z, c.f);
    }
}

#[cfg(test)]
mod decrement_y {
    use crate::*;

    #[test]
    fn implied_addressing() {
        let mut c = Chip::new();

        // Code:
        // DEY
        let prog: Vec<u8> = [0x88].to_vec();
        c.ry = 0x01;
        c.load_program(prog);
        c.startup(0x0200);

        c.execute_cycle();
        assert_eq!(c.rx, 0x00);
    }

    #[test]
    fn flag() {
        let mut c = Chip::new();

        // Code:
        // DEY
        let prog: Vec<u8> = [0x88].to_vec();
        c.ry = 0x01;
        c.load_program(prog);
        c.startup(0x0200);

        c.execute_cycle();
        assert_eq!(Z, c.f);
    }
}

#[cfg(test)]
mod increment {
    use crate::*;

    #[test]
    fn zeropage_addressing() {
        let mut c = Chip::new();

        // Code:
        // INC $01
        let prog: Vec<u8> = [0xE6, 0x01].to_vec();
        c.memory[0x01] = 0x12;
        c.load_program(prog);
        c.startup(0x0200);

        c.execute_cycle();
        assert_eq!(c.memory[0x01], 0x13);
    }

    #[test]
    fn zeropage_x_addressing() {
        let mut c = Chip::new();

        // Code:
        // INC $01,X
        let prog: Vec<u8> = [0xF6, 0x01].to_vec();
        c.rx = 0x01;
        c.memory[0x02] = 0x14;
        c.load_program(prog);
        c.startup(0x0200);

        c.execute_cycle();
        assert_eq!(c.memory[0x02], 0x15);
    }

    #[test]
    fn absolute_addressing() {
        let mut c = Chip::new();

        // Code:
        // INC $3010
        let prog: Vec<u8> = [0xEE, 0x10, 0x30].to_vec();
        c.memory[0x3010] = 0xAA;
        c.load_program(prog);
        c.startup(0x0200);

        c.execute_cycle();
        assert_eq!(c.memory[0x3010], 0xAB);
    }

    #[test]
    fn absolute_x_addressing() {
        let mut c = Chip::new();

        // Code:
        // INC $3120,X
        let prog: Vec<u8> = [0xFE, 0x20, 0x31].to_vec();
        c.rx = 0x12;
        c.memory[0x3132] = 0xAA;
        c.load_program(prog);
        c.startup(0x0200);

        c.execute_cycle();
        assert_eq!(c.memory[0x3132], 0xAB);
    }

    #[test]
    fn flag() {
        let mut c = Chip::new();

        // Code:
        // INC $01
        let prog: Vec<u8> = [0xE6, 0x01].to_vec();
        c.memory[0x01] = 0x01;
        c.load_program(prog);
        c.startup(0x0200);

        c.execute_cycle();
        assert_ne!(Z, c.f);
    }
}

#[cfg(test)]
mod increment_x {
    use crate::*;

    #[test]
    fn implied_addressing() {
        let mut c = Chip::new();

        // Code:
        // INX
        let prog: Vec<u8> = [0xE8].to_vec();
        c.rx = 0x11;
        c.load_program(prog);
        c.startup(0x0200);

        c.execute_cycle();
        assert_eq!(c.rx, 0x12);
    }

    #[test]
    fn flag() {
        let mut c = Chip::new();

        // Code:
        // INX
        let prog: Vec<u8> = [0xE8].to_vec();
        c.ry = 0x01;
        c.load_program(prog);
        c.startup(0x0200);

        c.execute_cycle();
        assert_ne!(Z, c.f);
    }
}

#[cfg(test)]
mod increment_y {
    use crate::*;

    #[test]
    fn implied_addressing() {
        let mut c = Chip::new();

        // Code:
        // INY
        let prog: Vec<u8> = [0xC8].to_vec();
        c.ry = 0x02;
        c.load_program(prog);
        c.startup(0x0200);

        c.execute_cycle();
        assert_eq!(c.ry, 0x03);
    }

    #[test]
    fn flag() {
        let mut c = Chip::new();

        // Code:
        // INY
        let prog: Vec<u8> = [0xC8].to_vec();
        c.ry = 0x01;
        c.load_program(prog);
        c.startup(0x0200);

        c.execute_cycle();
        assert_ne!(Z, c.f);
    }
}
