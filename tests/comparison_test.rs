use sixfiveohtwo::chip::*;

/// ==========================
/// COMPARISON TESTS
/// ==========================

#[cfg(test)]
mod compare_with_accumulator {
    use crate::*;

    #[test]
    fn immediate_addressing() {
        let mut c = Chip::new();

        // Code:
        // CMP
        let prog: Vec<u8> = [0xC9, 0x01].to_vec();
        c.acc = 0x01;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(c.f, C | Z);
    }

    #[test]
    fn zeropage_addressing() {
        let mut c = Chip::new();

        // Code:
        // CMP $80
        let prog: Vec<u8> = [0xC5, 0x80].to_vec();
        c.acc = 0x01;
        c.memory[0x80] = 0x01;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(c.f, C | Z);
    }

    #[test]
    fn zeropage_x_addressing() {
        let mut c = Chip::new();

        // Code:
        // CMP $80,X
        let prog: Vec<u8> = [0xD5, 0x80].to_vec();
        c.acc = 0x01;
        c.rx = 0x02;
        c.memory[0x82] = 0x01;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(c.f, C | Z);
    }

    #[test]
    fn absolute_addressing() {
        let mut c = Chip::new();

        // Code:
        // CMP $3010
        let prog: Vec<u8> = [0xCD].to_vec();
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(c.f, C | Z);
    }

    #[test]
    fn absolute_x_addressing() {
        let mut c = Chip::new();

        // Code:
        // CMP $3120,X
        let prog: Vec<u8> = [0xDD, 0x20, 0x31].to_vec();
        c.acc = 0x01;
        c.rx = 0x12;
        c.memory[0x3132] = 0x1;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(c.f, C | Z);
    }

    #[test]
    fn absolute_y_addressing() {
        let mut c = Chip::new();

        // Code:
        // CMP $3120,Y
        let prog: Vec<u8> = [0xD9, 0x20, 0x31].to_vec();
        c.acc = 0x01;
        c.ry = 0x12;
        c.memory[0x3132] = 0x1;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(c.f, C | Z);
    }

    #[test]
    fn indirect_x_addressing() {
        let mut c = Chip::new();

        // Code:
        // CMP ($70,X)
        let prog: Vec<u8> = [0xC1, 0x70].to_vec();
        c.acc = 0x01;
        c.rx = 0x05;
        c.memory[0x75] = 0x32;
        c.memory[0x76] = 0x30;
        c.memory[0x3032] = 0x01;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(c.f, C | Z);
    }

    #[test]
    fn indirect_y_addressing() {
        let mut c = Chip::new();

        // Code:
        // CMP ($70),Y
        let prog: Vec<u8> = [0xD1, 0x70].to_vec();
        c.acc = 0x01;
        c.ry = 0x10;
        c.memory[0x70] = 0x43;
        c.memory[0x71] = 0x35;
        c.memory[0x3553] = 0x01;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(c.f, C | Z);
    }
}

#[cfg(test)]
mod compare_with_x {
    use crate::*;

    #[test]
    fn immediate_addressing() {
        let mut c = Chip::new();

        // Code:
        // CPX #$01
        let prog: Vec<u8> = [0xE0, 0x01].to_vec();
        c.rx = 0x01;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(c.f, C | Z);
    }

    #[test]
    fn zeropage_addressing() {
        let mut c = Chip::new();

        // Code:
        // CPX $80
        let prog: Vec<u8> = [0xE4, 0x80].to_vec();
        c.memory[0x80] = 0x01;
        c.rx = 0x01;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(c.f, C | Z);
    }

    #[test]
    fn absolute_addressing() {
        let mut c = Chip::new();

        // Code:
        // CPX $3010
        let prog: Vec<u8> = [0xEC, 0x10, 0x30].to_vec();
        c.memory[0x3010] = 0x01;
        c.rx = 0x01;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(c.f, C | Z);
    }
}

#[cfg(test)]
mod compare_with_y {
    use crate::*;

    #[test]
    fn immediate_addressing() {
        let mut c = Chip::new();

        // Code:
        // CPY #$01
        let prog: Vec<u8> = [0xC0, 0x01].to_vec();
        c.ry = 0x01;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(c.f, C | Z);
    }

    #[test]
    fn zeropage_addressing() {
        let mut c = Chip::new();

        // Code:
        // CPY $80
        let prog: Vec<u8> = [0xC4, 0x80].to_vec();
        c.memory[0x80] = 0x01;
        c.ry = 0x01;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(c.f, C | Z);
    }

    #[test]
    fn absolute_addressing() {
        let mut c = Chip::new();

        // Code:
        // CPY $3010
        let prog: Vec<u8> = [0xCC, 0x10, 0x30].to_vec();
        c.memory[0x3010] = 0x01;
        c.ry = 0x01;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(c.f, C | Z);
    }
}
