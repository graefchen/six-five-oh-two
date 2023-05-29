use sixfiveohtwo::chip::*;

/// ==========================
/// INTERRUPT TESTS
/// ==========================

#[cfg(test)]
mod break_software_interrupt {
    use crate::*;

    #[test]
    fn implied_addressing() {
        let mut c = Chip::new();

        // Code:
        // BRK
        let prog: Vec<u8> = [0x00].to_vec();
        c.load_program(prog);
        c.startup(0x0200);

        c.execute_cycle();
        assert_eq!(c.memory[0x1FF], (c.pc + 2) as u8);
    }
}

#[cfg(test)]
mod return_from_interrupt {
    use crate::*;

    #[test]
    fn implied_addressing() {
        let mut c = Chip::new();

        // Code:
        // RTI
        let prog: Vec<u8> = [0x40].to_vec();
        c.load_program(prog);
        c.startup(0x0200);
        c.sp = 0xFC;
        c.memory[0x1FD] = C;
        c.memory[0x1FE] = 0x10;
        c.memory[0x1FF] = 0x30;

        c.execute_cycle();
        assert_eq!(c.f, C);
        assert_eq!(c.pc, 0x3010);
    }
}
