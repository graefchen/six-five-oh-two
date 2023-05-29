use sixfiveohtwo::chip::*;

/// ==========================
/// TRANSFER INSTRUCTIONS TEST
/// ==========================

#[cfg(test)]
mod load_accumulator {

    use crate::*;

    #[test]
    fn immediate_addressing() {
        let mut c = Chip::new();

        // Code:
        // LDA #$1
        let prog: Vec<u8> = [0xA9, 0x01].to_vec();
        c.load_program(prog);
        c.startup(0x0200);

        c.execute_cycle();
        assert_eq!(0x01, c.acc);
    }

    #[test]
    fn zeropage_addressing() {
        let mut c = Chip::new();

        // Code:
        // LDA $01
        let prog: Vec<u8> = [0xA5, 0x01].to_vec();
        c.memory[0x01] = 0x12;
        c.load_program(prog);
        c.startup(0x0200);

        c.execute_cycle();
        assert_eq!(0x12, c.acc);
    }

    #[test]
    fn zeropage_x_addressing() {
        let mut c = Chip::new();

        // Code:
        // LDA $01,X
        let prog: Vec<u8> = [0xB5, 0x01].to_vec();
        c.rx = 0x01;
        c.memory[0x02] = 0x12;
        c.load_program(prog);
        c.startup(0x0200);

        c.execute_cycle();
        assert_eq!(0x12, c.acc);
    }

    #[test]
    fn absolute_addressing() {
        let mut c = Chip::new();

        // Code:
        // LDA $3010
        let prog: Vec<u8> = [0xAD, 0x10, 0x30].to_vec();
        c.memory[0x3010] = 0x12;
        c.load_program(prog);
        c.startup(0x0200);

        c.execute_cycle();
        assert_eq!(0x12, c.acc);
    }

    #[test]
    fn absolute_x_addressing() {
        let mut c = Chip::new();

        // Code:
        // LDA $3120,X
        let prog: Vec<u8> = [0xBD, 0x20, 0x31].to_vec();
        c.rx = 0x12;
        c.memory[0x3132] = 0x12;
        c.load_program(prog);
        c.startup(0x0200);

        c.execute_cycle();
        assert_eq!(0x12, c.acc);
    }

    #[test]
    fn absolute_y_addressing() {
        let mut c = Chip::new();

        // Code:
        // LDA $3120,Y
        let prog: Vec<u8> = [0xB9, 0x20, 0x31].to_vec();
        c.ry = 0x12;
        c.memory[0x3132] = 0x12;
        c.load_program(prog);
        c.startup(0x0200);

        c.execute_cycle();
        assert_eq!(0x12, c.acc);
    }

    #[test]
    fn indirect_x_addressing() {
        let mut c = Chip::new();

        // Code:
        // LDA ($70,X)
        let prog: Vec<u8> = [0xA1, 0x70].to_vec();
        c.rx = 0x05;
        c.memory[0x75] = 0x32;
        c.memory[0x76] = 0x30;
        c.memory[0x3032] = 0xA5;
        c.load_program(prog);
        c.startup(0x0200);

        c.execute_cycle();
        assert_eq!(0xA5, c.acc);
    }

    #[test]
    fn indirect_y_addressing() {
        let mut c = Chip::new();

        // Code:
        // LDA ($70),Y
        let prog: Vec<u8> = [0xB1, 0x70].to_vec();
        c.ry = 0x10;
        c.memory[0x70] = 0x43;
        c.memory[0x71] = 0x35;
        c.memory[0x3553] = 0x12;
        c.load_program(prog);
        c.startup(0x0200);

        c.execute_cycle();
        assert_eq!(0x12, c.acc);
    }

    #[test]
    fn flags() {
        let mut c = Chip::new();

        // Code:
        // LDA #$01
        let prog: Vec<u8> = [0xA9, 0x01].to_vec();
        c.load_program(prog);
        c.startup(0x0200);

        c.execute_cycle();
        assert_ne!(Z, c.f);
    }
}

#[cfg(test)]
mod load_x {
    use crate::*;

    #[test]
    fn immediate_addressing() {
        let mut c = Chip::new();

        // Code:
        // LDX #$01
        let prog: Vec<u8> = [0xA2, 0x01].to_vec();
        c.load_program(prog);
        c.startup(0x0200);

        c.execute_cycle();
        assert_eq!(0x01, c.rx);
    }

    #[test]
    fn zeropage_addressing() {
        let mut c = Chip::new();

        // Code:
        // LDX #01
        let prog: Vec<u8> = [0xA6, 0x01].to_vec();
        c.memory[0x01] = 0x12;
        c.load_program(prog);
        c.startup(0x0200);

        c.execute_cycle();
        assert_eq!(0x12, c.rx);
    }

    #[test]
    fn zeropage_y_addressing() {
        let mut c = Chip::new();

        // Code:
        // LDX $01,Y
        let prog: Vec<u8> = [0xB6, 0x01].to_vec();
        c.ry = 0x4;
        c.memory[0x05] = 0x12;
        c.load_program(prog);
        c.startup(0x0200);

        c.execute_cycle();
        assert_eq!(0x12, c.rx);
    }

    #[test]
    fn absolute_addressing() {
        let mut c = Chip::new();

        // Code:
        // LDX $3120
        let prog: Vec<u8> = [0xAE, 0x20, 0x31].to_vec();
        c.memory[0x3120] = 0x12;
        c.load_program(prog);
        c.startup(0x0200);

        c.execute_cycle();
        assert_eq!(0x12, c.rx);
    }

    #[test]
    fn absolute_y_addressing() {
        let mut c = Chip::new();

        // Code:
        // LDX $3120,Y
        let prog: Vec<u8> = [0xBE, 0x20, 0x31].to_vec();
        c.ry = 0x12;
        c.memory[0x3132] = 0x12;
        c.load_program(prog);
        c.startup(0x0200);

        c.execute_cycle();
        assert_eq!(0x12, c.rx);
    }

    #[test]
    fn flags() {
        let mut c = Chip::new();

        // Code:
        // LDX #$01
        let prog: Vec<u8> = [0xA2, 0x01].to_vec();
        c.load_program(prog);
        c.startup(0x0200);

        c.execute_cycle();
        assert_ne!(Z, c.f);
    }
}

#[cfg(test)]
mod load_y {
    use crate::*;

    #[test]
    fn immediate_addressing() {
        let mut c = Chip::new();

        // Code:
        // LDY #$01
        let prog: Vec<u8> = [0xA0, 0x01].to_vec();
        c.load_program(prog);
        c.startup(0x0200);

        c.execute_cycle();
        assert_eq!(0x01, c.ry);
    }

    #[test]
    fn zeropage_addressing() {
        let mut c = Chip::new();

        // Code:
        // LDY $01
        let prog: Vec<u8> = [0xA4, 0x01].to_vec();
        c.memory[0x01] = 0x12;
        c.load_program(prog);
        c.startup(0x0200);

        c.execute_cycle();
        assert_eq!(0x12, c.ry);
    }

    #[test]
    fn zeropage_x_addressing() {
        let mut c = Chip::new();

        // Code:
        // LDY $01,Y
        let prog: Vec<u8> = [0xB4, 0x01].to_vec();
        c.rx = 0x0A;
        c.memory[0x0B] = 0x12;
        c.load_program(prog);
        c.startup(0x0200);

        c.execute_cycle();
        assert_eq!(0x12, c.ry);
    }

    #[test]
    fn absolute_addressing() {
        let mut c = Chip::new();

        // Code:
        // LDY $FFF0
        let prog: Vec<u8> = [0xAC, 0xF0, 0xFF].to_vec();
        c.memory[0xFFF0] = 0x12;
        c.load_program(prog);
        c.startup(0x0200);

        c.execute_cycle();
        assert_eq!(0x12, c.ry);
    }

    #[test]
    fn absolute_x_addressing() {
        let mut c = Chip::new();

        // Code:
        // LDY $3120,X
        let prog: Vec<u8> = [0xBC, 0x20, 0x31].to_vec();
        c.rx = 0x12;
        c.memory[0x3132] = 0x12;
        c.load_program(prog);
        c.startup(0x0200);

        c.execute_cycle();
        assert_eq!(0x12, c.ry);
    }

    #[test]
    fn flags() {
        let mut c = Chip::new();

        // Code:
        // LDY #$01
        let prog: Vec<u8> = [0xA0, 0x01].to_vec();
        c.load_program(prog);
        c.startup(0x0200);

        c.execute_cycle();
        assert_ne!(Z, c.f);
    }
}

#[cfg(test)]
mod store_accumulator {
    use crate::*;

    #[test]
    fn zeropage_addressing() {
        let mut c = Chip::new();

        // Code:
        // STA $01
        let prog: Vec<u8> = [0x85, 0x01].to_vec();
        c.acc = 0x01;
        c.load_program(prog);
        c.startup(0x0200);

        c.execute_cycle();
        assert_eq!(c.memory[0x01], 0x01);
    }

    #[test]
    fn zeropage_x_addressing() {
        let mut c = Chip::new();

        // Code:
        // STA $01,X
        let prog: Vec<u8> = [0x95, 0x01].to_vec();
        c.acc = 0x01;
        c.rx = 0x01;
        c.load_program(prog);
        c.startup(0x0200);

        c.execute_cycle();
        assert_eq!(c.memory[0x02], 0x01);
    }

    #[test]
    fn absolute_addressing() {
        let mut c = Chip::new();

        // Code:
        // STA $FFF0
        let prog: Vec<u8> = [0x8D, 0xF0, 0xFF].to_vec();
        c.acc = 0x01;
        c.load_program(prog);
        c.startup(0x0200);

        c.execute_cycle();
        assert_eq!(c.memory[0xFFF0], 0x01);
    }

    #[test]
    fn absolute_x_addressing() {
        let mut c = Chip::new();

        // Code:
        // STA $3120,X
        let prog: Vec<u8> = [0x9D, 0x20, 0x31].to_vec();
        c.acc = 0x01;
        c.rx = 0x12;
        c.load_program(prog);
        c.startup(0x0200);

        c.execute_cycle();
        assert_eq!(c.memory[0x3132], 0x01);
    }

    #[test]
    fn absolute_y_addressing() {
        let mut c = Chip::new();

        // Code:
        // STA $3120,Y
        let prog: Vec<u8> = [0x99, 0x20, 0x31].to_vec();
        c.acc = 0x01;
        c.ry = 0x12;
        c.load_program(prog);
        c.startup(0x0200);

        c.execute_cycle();
        assert_eq!(c.memory[0x3132], 0x01);
    }

    #[test]
    fn indirect_x_addressing() {
        let mut c = Chip::new();

        // Code:
        // STA ($70,X)
        let prog: Vec<u8> = [0x81, 0x70].to_vec();
        c.acc = 0x01;
        c.rx = 0x05;
        c.memory[0x75] = 0x32;
        c.memory[0x76] = 0x30;
        c.load_program(prog);
        c.startup(0x0200);

        c.execute_cycle();
        assert_eq!(c.memory[0x3032], 0x01);
    }

    #[test]
    fn indirect_y_addressing() {
        let mut c = Chip::new();

        // Code:
        // STA ($70),Y
        let prog: Vec<u8> = [0x91, 0x70].to_vec();
        c.acc = 0x01;
        c.ry = 0x10;
        c.memory[0x70] = 0x43;
        c.memory[0x71] = 0x35;
        c.load_program(prog);
        c.startup(0x0200);

        c.execute_cycle();
        assert_eq!(c.memory[0x3553], 0x01);
    }
}

#[cfg(test)]
mod store_x {
    use crate::*;

    #[test]
    fn zeropage_addressing() {
        let mut c = Chip::new();

        // Code:
        // STX $01
        let prog: Vec<u8> = [0x86, 0x01].to_vec();
        c.rx = 0x01;
        c.load_program(prog);
        c.startup(0x0200);

        c.execute_cycle();
        assert_eq!(c.memory[0x01], 0x01);
    }

    #[test]
    fn zeropage_y_addressing() {
        let mut c = Chip::new();

        // Code:
        // STX $01,Y
        let prog: Vec<u8> = [0x96, 0x01].to_vec();
        c.rx = 0x01;
        c.ry = 0x01;
        c.load_program(prog);
        c.startup(0x0200);

        c.execute_cycle();
        assert_eq!(c.memory[0x02], 0x01);
    }

    #[test]
    fn absolute_addressing() {
        let mut c = Chip::new();

        // Code:
        // STX $FFF0
        let prog: Vec<u8> = [0x8E, 0xF0, 0xFF].to_vec();
        c.rx = 0x01;
        c.load_program(prog);
        c.startup(0x0200);

        c.execute_cycle();
        assert_eq!(c.memory[0xFFF0], 0x01);
    }
}

#[cfg(test)]
mod store_y {
    use crate::*;

    #[test]
    fn zeropage_addressing() {
        let mut c = Chip::new();

        // Code:
        // STA $01
        let prog: Vec<u8> = [0x84, 0x01].to_vec();
        c.ry = 0x01;
        c.load_program(prog);
        c.startup(0x0200);

        c.execute_cycle();
        assert_eq!(c.memory[0x01], 0x01);
    }

    #[test]
    fn zeropage_x_addressing() {
        let mut c = Chip::new();

        // Code:
        // STA $01,X
        let prog: Vec<u8> = [0x94, 0x01].to_vec();
        c.rx = 0x01;
        c.ry = 0x01;
        c.load_program(prog);
        c.startup(0x0200);

        c.execute_cycle();
        assert_eq!(c.memory[0x02], 0x01);
    }

    #[test]
    fn absolute_addressing() {
        let mut c = Chip::new();

        // Code:
        // STA $FFF0
        let prog: Vec<u8> = [0x8C, 0xF0, 0xFF].to_vec();
        c.ry = 0x01;
        c.load_program(prog);
        c.startup(0x0200);

        c.execute_cycle();
        assert_eq!(c.memory[0xFFF0], 0x01);
    }
}

#[cfg(test)]
mod transfer_accumulator_to_x {
    use crate::*;

    #[test]
    fn implied_addressing() {
        let mut c = Chip::new();

        // Code:
        // TAX
        let prog: Vec<u8> = [0xAA].to_vec();
        c.acc = 0x01;
        c.load_program(prog);
        c.startup(0x0200);

        c.execute_cycle();
        assert_eq!(c.rx, 0x01);
    }
}

#[cfg(test)]
mod transfer_accumulator_to_y {
    use crate::*;

    #[test]
    fn implied_addressing() {
        let mut c = Chip::new();

        // Code:
        // TAY
        let prog: Vec<u8> = [0xA8].to_vec();
        c.acc = 0x01;
        c.load_program(prog);
        c.startup(0x0200);

        c.execute_cycle();
        assert_eq!(c.ry, 0x01);
    }
}

#[cfg(test)]
mod transfer_stack_pointer_to_x {
    use crate::*;

    #[test]
    fn implied_addressing() {
        let mut c = Chip::new();

        // Code:
        // TSX
        let prog: Vec<u8> = [0xBA].to_vec();
        c.sp = 0x01;
        c.load_program(prog);
        c.startup(0x0200);

        c.execute_cycle();
        assert_eq!(c.rx, 0x01);
    }
}

#[cfg(test)]
mod transfer_x_to_accumulator {
    use crate::*;

    #[test]
    fn implied_addressing() {
        let mut c = Chip::new();

        // Code:
        // TXA
        let prog: Vec<u8> = [0x8A].to_vec();
        c.rx = 0x01;
        c.load_program(prog);
        c.startup(0x0200);

        c.execute_cycle();
        assert_eq!(c.acc, 0x01);
    }
}

#[cfg(test)]
mod transfer_x_to_stack_pointer {
    use crate::*;

    #[test]
    fn implied_addressing() {
        let mut c = Chip::new();

        // Code:
        // TXS
        let prog: Vec<u8> = [0x9A].to_vec();
        c.rx = 0x01;
        c.load_program(prog);
        c.startup(0x0200);

        c.execute_cycle();
        assert_eq!(c.sp, 0x01);
    }
}

#[cfg(test)]
mod transfer_y_to_accumulator {
    use crate::*;

    #[test]
    fn implied_addressing() {
        let mut c = Chip::new();

        // Code:
        // TYA
        let prog: Vec<u8> = [0x98].to_vec();
        c.ry = 0x01;
        c.load_program(prog);
        c.startup(0x0200);

        c.execute_cycle();
        assert_eq!(c.acc, 0x01);
    }
}
