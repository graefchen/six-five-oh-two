use sixfiveohtwo::chip::*;

/// ==========================
/// LOGICAL TESTS
/// ==========================

#[cfg(test)]
mod and_with_accumulator {
    use crate::*;

    #[test]
    fn immediate_addressing() {
        let mut c = Chip::new();

        // Code:
        // AND #$7
        let prog: Vec<u8> = [0x29, 0x07].to_vec();
        c.acc = 0x07;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(c.acc, 0x07);
    }

    #[test]
    fn zeropage_addressing() {
        let mut c = Chip::new();

        // Code:
        // AND $80
        let prog: Vec<u8> = [0x25, 0x80].to_vec();
        c.memory[0x80] = 0x07;
        c.acc = 0x07;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(c.acc, 0x07);
    }

    #[test]
    fn zeropage_x_addressing() {
        let mut c = Chip::new();

        // Code:
        // AND $80,X
        let prog: Vec<u8> = [0x35, 0x80].to_vec();
        c.rx = 0x2;
        c.acc = 0x07;
        c.memory[0x82] = 0x07;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(c.acc, 0x07);
    }

    #[test]
    fn absolute_addressing() {
        let mut c = Chip::new();

        // Code:
        // AND $3010
        let prog: Vec<u8> = [0x2D, 0x10, 0x30].to_vec();
        c.acc = 0x07;
        c.memory[0x3010] = 0x07;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(c.acc, 0x07);
    }

    #[test]
    fn absolute_x_addressing() {
        let mut c = Chip::new();

        // Code:
        // AND $3120,X
        let prog: Vec<u8> = [0x3D, 0x20, 0x031].to_vec();
        c.rx = 0x12;
        c.acc = 0x07;
        c.memory[0x3132] = 0x07;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(c.acc, 0x07);
    }

    #[test]
    fn absolute_y_addressing() {
        let mut c = Chip::new();

        // Code:
        // AND $3120,Y
        let prog: Vec<u8> = [0x39, 0x20, 0x031].to_vec();
        c.ry = 0x12;
        c.acc = 0x07;
        c.memory[0x3132] = 0x07;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(c.acc, 0x07);
    }

    #[test]
    fn indirect_x_addressing() {
        let mut c = Chip::new();

        // Code:
        // AND ($70,X)
        let prog: Vec<u8> = [0x21, 0x70].to_vec();
        c.rx = 0x05;
        c.acc = 0x07;
        c.memory[0x75] = 0x32;
        c.memory[0x76] = 0x30;
        c.memory[0x3032] = 0x07;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(c.acc, 0x07);
    }

    #[test]
    fn indirect_y_addressing() {
        let mut c = Chip::new();

        // Code:
        // AND ($70),Y
        let prog: Vec<u8> = [0x31, 0x70].to_vec();
        c.ry = 0x10;
        c.acc = 0x07;
        c.memory[0x70] = 0x43;
        c.memory[0x71] = 0x35;
        c.memory[0x3553] = 0x07;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(c.acc, 0x07);
    }
}

#[cfg(test)]
mod exclusive_or_with_accumulator {
    use crate::*;

    #[test]
    fn immediate_addressing() {
        let mut c = Chip::new();

        // Code:
        // EOR #$7
        let prog: Vec<u8> = [0x49, 0x07].to_vec();
        c.acc = 0x07;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(c.acc, 0x00);
    }

    #[test]
    fn zeropage_addressing() {
        let mut c = Chip::new();

        // Code:
        // EOR $80
        let prog: Vec<u8> = [0x45, 0x80].to_vec();
        c.memory[0x80] = 0x07;
        c.acc = 0x07;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(c.acc, 0x00);
    }

    #[test]
    fn zeropage_x_addressing() {
        let mut c = Chip::new();

        // Code:
        // EOR $80,X
        let prog: Vec<u8> = [0x55, 0x80].to_vec();
        c.rx = 0x2;
        c.acc = 0x07;
        c.memory[0x82] = 0x07;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(c.acc, 0x00);
    }

    #[test]
    fn absolute_addressing() {
        let mut c = Chip::new();

        // Code:
        // EOR $3010
        let prog: Vec<u8> = [0x4D, 0x10, 0x30].to_vec();
        c.acc = 0x07;
        c.memory[0x3010] = 0x07;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(c.acc, 0x00);
    }

    #[test]
    fn absolute_x_addressing() {
        let mut c = Chip::new();

        // Code:
        // EOR $3120,X
        let prog: Vec<u8> = [0x5D, 0x20, 0x031].to_vec();
        c.rx = 0x12;
        c.acc = 0x07;
        c.memory[0x3132] = 0x07;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(c.acc, 0x00);
    }

    #[test]
    fn absolute_y_addressing() {
        let mut c = Chip::new();

        // Code:
        // EOR $3120,Y
        let prog: Vec<u8> = [0x59, 0x20, 0x031].to_vec();
        c.ry = 0x12;
        c.acc = 0x07;
        c.memory[0x3132] = 0x07;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(c.acc, 0x00);
    }

    #[test]
    fn indirect_x_addressing() {
        let mut c = Chip::new();

        // Code:
        // EOR ($70,X)
        let prog: Vec<u8> = [0x41, 0x70].to_vec();
        c.rx = 0x05;
        c.acc = 0x07;
        c.memory[0x75] = 0x32;
        c.memory[0x76] = 0x30;
        c.memory[0x3032] = 0x07;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(c.acc, 0x00);
    }

    #[test]
    fn indirect_y_addressing() {
        let mut c = Chip::new();

        // Code:
        // EOR ($70),Y
        let prog: Vec<u8> = [0x51, 0x70].to_vec();
        c.ry = 0x10;
        c.acc = 0x07;
        c.memory[0x70] = 0x43;
        c.memory[0x71] = 0x35;
        c.memory[0x3553] = 0x07;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(c.acc, 0x00);
    }
}

#[cfg(test)]
mod or_with_accumulator {
    use crate::*;

    #[test]
    fn immediate_addressing() {
        let mut c = Chip::new();

        // Code:
        // ORA #$7
        let prog: Vec<u8> = [0x49, 0x07].to_vec();
        c.acc = 0x00;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(c.acc, 0x07);
    }

    #[test]
    fn zeropage_addressing() {
        let mut c = Chip::new();

        // Code:
        // ORA $80
        let prog: Vec<u8> = [0x45, 0x80].to_vec();
        c.memory[0x80] = 0x00;
        c.acc = 0x07;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(c.acc, 0x07);
    }

    #[test]
    fn zeropage_x_addressing() {
        let mut c = Chip::new();

        // Code:
        // ORA $80,X
        let prog: Vec<u8> = [0x55, 0x80].to_vec();
        c.rx = 0x2;
        c.acc = 0x07;
        c.memory[0x82] = 0x00;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(c.acc, 0x07);
    }

    #[test]
    fn absolute_addressing() {
        let mut c = Chip::new();

        // Code:
        // ORA $3010
        let prog: Vec<u8> = [0x4D, 0x10, 0x30].to_vec();
        c.acc = 0x07;
        c.memory[0x3010] = 0x00;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(c.acc, 0x07);
    }

    #[test]
    fn absolute_x_addressing() {
        let mut c = Chip::new();

        // Code:
        // ORA $3120,X
        let prog: Vec<u8> = [0x5D, 0x20, 0x031].to_vec();
        c.rx = 0x12;
        c.acc = 0x07;
        c.memory[0x3132] = 0x00;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(c.acc, 0x07);
    }

    #[test]
    fn absolute_y_addressing() {
        let mut c = Chip::new();

        // Code:
        // ORA $3120,Y
        let prog: Vec<u8> = [0x59, 0x20, 0x031].to_vec();
        c.ry = 0x12;
        c.acc = 0x07;
        c.memory[0x3132] = 0x00;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(c.acc, 0x07);
    }

    #[test]
    fn indirect_x_addressing() {
        let mut c = Chip::new();

        // Code:
        // ORA ($70,X)
        let prog: Vec<u8> = [0x41, 0x70].to_vec();
        c.rx = 0x05;
        c.acc = 0x07;
        c.memory[0x75] = 0x32;
        c.memory[0x76] = 0x30;
        c.memory[0x3032] = 0x00;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(c.acc, 0x07);
    }

    #[test]
    fn indirect_y_addressing() {
        let mut c = Chip::new();

        // Code:
        // ORA ($70),Y
        let prog: Vec<u8> = [0x51, 0x70].to_vec();
        c.ry = 0x10;
        c.acc = 0x07;
        c.memory[0x70] = 0x43;
        c.memory[0x71] = 0x35;
        c.memory[0x3553] = 0x00;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(c.acc, 0x07);
    }
}
