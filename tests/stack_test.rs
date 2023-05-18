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

        c.execute_cycle();
        assert_eq!(c.memory[0x100], 0x07);
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

        c.execute_cycle();
        assert_eq!(c.memory[0x100], Z | B);
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
        c.memory[0x0100] = 0x07;
        c.sp = 1;

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
        c.memory[0x0100] = B;
        c.sp = 1;

        c.execute_cycle();
        assert_eq!(c.f, 0x0);
    }
}
