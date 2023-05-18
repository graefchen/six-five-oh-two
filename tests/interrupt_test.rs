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

        c.execute_cycle();
        assert_eq!(c.memory[0x100], (c.pc + 2) as u8);
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
        c.sp = 0x03;
        c.memory[0x100] = 0x30;
        c.memory[0x101] = 0x10;
        c.memory[0x102] = B;

        c.execute_cycle();
        assert_eq!(c.f, 0x00);
        assert_eq!(c.pc, 0x3010);
    }
}
