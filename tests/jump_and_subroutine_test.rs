use sixfiveohtwo::chip::*;

/// ==========================
/// JUMP & SUBROUTINE TESTS
/// ==========================

#[cfg(test)]
mod jump {
    use crate::*;

    #[test]
    fn absolute_addressing() {
        let mut c = Chip::new();

        // Code:
        // JMP $3010
        // LDA #$FF
        let prog: Vec<u8> = [0x4C, 0x10, 0x30].to_vec();
        c.load_program(prog);
        c.startup(0x0200);
        c.memory[0x3010] = 0xA9;
        c.memory[0x3011] = 0xFF;

        c.execute_cycle();
        c.execute_cycle();
        assert_eq!(0xFF, c.acc);
    }

    #[test]
    fn indirect_addressing() {
        let mut c = Chip::new();

        // Code:
        // JMP ($4FF82)
        // LDA #$FF
        let prog: Vec<u8> = [0x6C, 0x82, 0xFF].to_vec();
        c.load_program(prog);
        c.startup(0x0200);
        c.memory[0x4240] = 0xA9;
        c.memory[0x4241] = 0xFF;
        c.memory[0xFF82] = 0x40;
        c.memory[0xFF83] = 0x42;

        c.execute_cycle();
        c.execute_cycle();
        assert_eq!(0xFF, c.acc);
    }
}

#[cfg(test)]
mod jump_soubroutine {
    use crate::*;

    #[test]
    fn absolute_addressing() {
        let mut c = Chip::new();

        // Code:
        // JSR $4240
        // LDA #$FF
        let prog: Vec<u8> = [0x20, 0x40, 0x42].to_vec();
        c.load_program(prog);
        c.startup(0x0200);
        c.memory[0x4240] = 0xA9;
        c.memory[0x4241] = 0xFF;

        c.execute_cycle();
        c.execute_cycle();
        assert_eq!(0xFF, c.acc);
        assert_eq!(c.memory[0x01FE], 0x02);
        assert_eq!(c.memory[0x01FF], 0x02);
    }
}

#[cfg(test)]
mod return_from_subroutine {
    use crate::*;

    #[test]
    fn implied_addressing() {
        let mut c = Chip::new();

        // Code:
        // JSR $4240
        // LDA #$FF
        // RTS
        // LDA #$F0
        let prog: Vec<u8> = [0x20, 0x40, 0x42, 0xA9, 0xF0].to_vec();
        c.load_program(prog);
        c.startup(0x0200);
        c.memory[0x4240] = 0xA9;
        c.memory[0x4241] = 0xFF;
        c.memory[0x4242] = 0x60;

        c.execute_cycle();
        c.execute_cycle();
        c.execute_cycle();
        assert_eq!(0xFF, c.acc);
        c.execute_cycle();
        assert_eq!(0xF0, c.acc);
    }
}
