use sixfiveohtwo::chip::*;

/// ==========================
/// SHIFT & ROTATE TESTS
/// ==========================

#[cfg(test)]
mod arithmtic_shift_left {
    use crate::*;

    #[test]
    fn accumulator_addressing() {
        let mut c = Chip::new();

        // Code:
        // ASL A
        let prog: Vec<u8> = [0x0A].to_vec();
        c.acc = 0x81;
        c.load_program(prog);
        c.startup(0x0200);

        c.execute_cycle();
        assert_eq!(c.acc, 0x2);
    }

    #[test]
    fn zeropage_addressing() {
        let mut c = Chip::new();

        // Code:
        // ASL $80
        let prog: Vec<u8> = [0x06, 0x80].to_vec();
        c.memory[0x80] = 0x01;
        c.load_program(prog);
        c.startup(0x0200);

        c.execute_cycle();
        assert_eq!(c.memory[0x80], 0x02);
    }

    #[test]
    fn zeropage_x_addressing() {
        let mut c = Chip::new();

        // Code:
        // ASL $80,X
        let prog: Vec<u8> = [0x16, 0x80].to_vec();
        c.rx = 0x02;
        c.memory[0x82] = 0x01;
        c.load_program(prog);
        c.startup(0x0200);

        c.execute_cycle();
        assert_eq!(c.memory[0x82], 0x02);
    }

    #[test]
    fn absolute_addressing() {
        let mut c = Chip::new();

        // Code:
        // ASL $3010
        let prog: Vec<u8> = [0x0E, 0x10, 0x30].to_vec();
        c.memory[0x3010] = 0x01;
        c.load_program(prog);
        c.startup(0x0200);

        c.execute_cycle();
        assert_eq!(c.memory[0x3010], 0x02);
    }

    #[test]
    fn aabsolute_x_addressing() {
        let mut c = Chip::new();

        // Code:
        // ASL $3120,X
        let prog: Vec<u8> = [0x1E, 0x20, 0x31].to_vec();
        c.rx = 0x12;
        c.memory[0x3132] = 0x01;
        c.load_program(prog);
        c.startup(0x0200);

        c.execute_cycle();
        assert_eq!(c.memory[0x3132], 0x02);
    }
}

#[cfg(test)]
mod logical_shift_right {
    use crate::*;

    #[test]
    fn accumulator_addressing() {
        let mut c = Chip::new();

        // Code:
        // LSR A
        let prog: Vec<u8> = [0x4A].to_vec();
        c.acc = 0x02;
        c.load_program(prog);
        c.startup(0x0200);

        c.execute_cycle();
        assert_eq!(c.acc, 0x01);
    }

    #[test]
    fn zeropage_addressing() {
        let mut c = Chip::new();

        // Code:
        // LSR $80
        let prog: Vec<u8> = [0x46, 0x80].to_vec();
        c.memory[0x80] = 0x02;
        c.load_program(prog);
        c.startup(0x0200);

        c.execute_cycle();
        assert_eq!(c.memory[0x80], 0x01);
    }

    #[test]
    fn zeropage_x_addressing() {
        let mut c = Chip::new();

        // Code:
        // LSR $80,X
        let prog: Vec<u8> = [0x56, 0x80].to_vec();
        c.rx = 0x02;
        c.memory[0x82] = 0x02;
        c.load_program(prog);
        c.startup(0x0200);

        c.execute_cycle();
        assert_eq!(c.memory[0x82], 0x01);
    }

    #[test]
    fn absolute_addressing() {
        let mut c = Chip::new();

        // Code:
        // LSR $3010
        let prog: Vec<u8> = [0x4E, 0x10, 0x30].to_vec();
        c.memory[0x3010] = 0x02;
        c.load_program(prog);
        c.startup(0x0200);

        c.execute_cycle();
        assert_eq!(c.memory[0x3010], 0x01);
    }

    #[test]
    fn aabsolute_x_addressing() {
        let mut c = Chip::new();

        // Code:
        // LSR $3120,X
        let prog: Vec<u8> = [0x5E, 0x20, 0x31].to_vec();
        c.rx = 0x12;
        c.memory[0x3132] = 0x02;
        c.load_program(prog);
        c.startup(0x0200);

        c.execute_cycle();
        assert_eq!(c.memory[0x3132], 0x01);
    }
}

#[cfg(test)]
mod rotate_left {
    use crate::*;

    #[test]
    fn accumulator_addressing() {
        let mut c = Chip::new();

        // Code:
        // ROL A
        let prog: Vec<u8> = [0x2A].to_vec();
        c.f = C;
        c.acc = 0xFF;
        c.load_program(prog);
        c.startup(0x0200);

        c.execute_cycle();
        assert_eq!(c.acc, 0xFF);
    }

    #[test]
    fn zeropage_addressing() {
        let mut c = Chip::new();

        // Code:
        // ROL $80
        let prog: Vec<u8> = [0x26, 0x80].to_vec();
        c.f = C;
        c.memory[0x80] = 0xFF;
        c.load_program(prog);
        c.startup(0x0200);

        c.execute_cycle();
        assert_eq!(c.memory[0x80], 0xFF);
    }

    #[test]
    fn zeropage_x_addressing() {
        let mut c = Chip::new();

        // Code:
        // ROL $80,X
        let prog: Vec<u8> = [0x36, 0x80].to_vec();
        c.f = C;
        c.rx = 0x02;
        c.memory[0x82] = 0xFF;
        c.load_program(prog);
        c.startup(0x0200);

        c.execute_cycle();
        assert_eq!(c.memory[0x82], 0xFF);
    }

    #[test]
    fn absolute_addressing() {
        let mut c = Chip::new();

        // Code:
        // ROL $3010
        let prog: Vec<u8> = [0x2E, 0x10, 0x30].to_vec();
        c.f = C;
        c.memory[0x3010] = 0xFF;
        c.load_program(prog);
        c.startup(0x0200);

        c.execute_cycle();
        assert_eq!(c.memory[0x3010], 0xFF);
    }

    #[test]
    fn absolute_x_addressing() {
        let mut c = Chip::new();

        // Code:
        // ROL $3120,X
        let prog: Vec<u8> = [0x3E, 0x20, 0x31].to_vec();
        c.f = C;
        c.rx = 0x12;
        c.memory[0x3132] = 0xFF;
        c.load_program(prog);
        c.startup(0x0200);

        c.execute_cycle();
        assert_eq!(c.memory[0x3132], 0xFF);
    }
}

#[cfg(test)]
mod rotate_right {
    use crate::*;

    #[test]
    fn accumulator_addressing() {
        let mut c = Chip::new();

        // Code:
        // ROR A
        let prog: Vec<u8> = [0x6A].to_vec();
        c.f = C;
        c.acc = 0xFF;
        c.load_program(prog);
        c.startup(0x0200);

        c.execute_cycle();
        assert_eq!(c.acc, 0xFF);
    }

    #[test]
    fn zeropage_addressing() {
        let mut c = Chip::new();

        // Code:
        // ROR $80
        let prog: Vec<u8> = [0x66, 0x80].to_vec();
        c.f = C;
        c.memory[0x80] = 0xFF;
        c.load_program(prog);
        c.startup(0x0200);

        c.execute_cycle();
        assert_eq!(c.memory[0x80], 0xFF);
    }

    #[test]
    fn zeropage_x_addressing() {
        let mut c = Chip::new();

        // Code:
        // ROR $80,X
        let prog: Vec<u8> = [0x76, 0x80].to_vec();
        c.f = C;
        c.rx = 0x02;
        c.memory[0x82] = 0xFF;
        c.load_program(prog);
        c.startup(0x0200);

        c.execute_cycle();
        assert_eq!(c.memory[0x82], 0xFF);
    }

    #[test]
    fn absolute_addressing() {
        let mut c = Chip::new();

        // Code:
        // ROR $3010
        let prog: Vec<u8> = [0x7E, 0x10, 0x30].to_vec();
        c.f = C;
        c.memory[0x3010] = 0xFF;
        c.load_program(prog);
        c.startup(0x0200);

        c.execute_cycle();
        assert_eq!(c.memory[0x3010], 0xFF);
    }

    #[test]
    fn aabsolute_x_addressing() {
        let mut c = Chip::new();

        // Code:
        // ROR $3120,X
        let prog: Vec<u8> = [0x6E, 0x20, 0x31].to_vec();
        c.f = C;
        c.rx = 0x12;
        c.memory[0x3132] = 0xFF;
        c.load_program(prog);
        c.startup(0x0200);

        c.execute_cycle();
        assert_eq!(c.memory[0x3132], 0xFF);
    }
}
