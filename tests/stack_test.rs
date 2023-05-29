use sixfiveohtwo::chip::*;

/// ==============================
/// STACK INSTRUCTIONS TEST
/// ==============================

#[cfg(test)]
mod push_accumulator {
    use crate::*;

    #[test]
    fn implied_addressing() {
        let mut c = Chip::new();

        // Code:
        // PHA
        let prog: Vec<u8> = [0x48].to_vec();
        c.acc = 0x07;
        c.load_program(prog);
        c.startup(0x0200);

        c.execute_cycle();
        assert_eq!(c.memory[0x1FF], 0x07);
    }
}

#[cfg(test)]
mod push_processor_status_register {
    use crate::*;

    #[test]
    fn implied_addressing() {
        let mut c = Chip::new();

        // Code:
        // PHP
        let prog: Vec<u8> = [0x08].to_vec();
        c.f = Z;
        c.load_program(prog);
        c.startup(0x0200);

        c.execute_cycle();
        assert_eq!(c.memory[0x1FF], Z | B | 0x20);
    }
}

#[cfg(test)]
mod pull_accumulator {
    use crate::*;

    #[test]
    fn implied_addressing() {
        let mut c = Chip::new();

        // Code:
        // PLA
        let prog: Vec<u8> = [0x68].to_vec();
        c.load_program(prog);
        c.startup(0x0200);
        c.memory[0x01FF] = 0x07;
        c.sp = 0xFE;

        c.execute_cycle();
        assert_eq!(c.acc, 0x07);
    }
}

#[cfg(test)]
mod pull_processor_status_register {
    use crate::*;

    #[test]
    fn implied_addressing() {
        let mut c = Chip::new();

        // Code:
        // PLP
        let prog: Vec<u8> = [0x28].to_vec();
        c.load_program(prog);
        c.startup(0x0200);
        c.memory[0x01FF] = C;
        c.sp = 0xFE;

        c.execute_cycle();
        assert_eq!(c.f, C);
    }
}
