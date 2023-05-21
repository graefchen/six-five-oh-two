use sixfiveohtwo::chip::*;

/// ==========================
/// CONDITIONAL BRANCH TESTS
/// ==========================

#[cfg(test)]
mod branch_on_carry_clear {
    use crate::*;

    #[test]
    fn relative_addressing() {
        let mut c = Chip::new();

        // Code:
        // BCC $01
        // LDA $01
        let prog: Vec<u8> = [0x90, 0x01, 0x00, 0xA9, 0x01].to_vec();
        //                               ^ this value is not read
        c.load_program(prog);
        c.f = Z;

        c.execute_cycle();
        c.execute_cycle();
        assert_eq!(0x01, c.acc);
    }
}

#[cfg(test)]
mod branch_on_carry_set {
    use crate::*;

    #[test]
    fn relative_addressing() {
        let mut c = Chip::new();

        // Code:
        // BCS $01
        // LDA $01
        let prog: Vec<u8> = [0xB0, 0x01, 0x00, 0xA9, 0x01].to_vec();
        //                               ^ this value is not read
        c.load_program(prog);
        c.f = C;

        c.execute_cycle();
        c.execute_cycle();
        assert_eq!(0x01, c.acc);
    }
}

#[cfg(test)]
mod branch_on_equal {
    use crate::*;

    #[test]
    fn relative_addressing() {
        let mut c = Chip::new();

        // Code:
        // BEQ $01
        // LDA $01
        let prog: Vec<u8> = [0xF0, 0x01, 0x00, 0xA9, 0x01].to_vec();
        //                               ^ this value is not read
        c.load_program(prog);
        c.f = Z;

        c.execute_cycle();
        c.execute_cycle();
        assert_eq!(0x01, c.acc);
    }

    #[test]
    fn flag_not_zero() {
        let mut c = Chip::new();

        // Code:
        // BEQ $01
        // LDA $20
        // LDA $01
        let prog: Vec<u8> = [0xF0, 0x01, 0xA9, 0x20, 0xA9, 0x01].to_vec();
        c.load_program(prog);
        c.f = N;

        c.execute_cycle();
        c.execute_cycle();
        assert_ne!(0x01, c.acc);
        assert_eq!(0x20, c.acc);
        assert_eq!(0x204, c.pc);
    }
}

#[cfg(test)]
mod branch_on_minus {
    use crate::*;

    #[test]
    fn relative_addressing() {
        let mut c = Chip::new();

        // Code:
        // BMI $01
        // LDA $01
        let prog: Vec<u8> = [0x30, 0x01, 0x00, 0xA9, 0x01].to_vec();
        //                               ^ this value is not read
        c.load_program(prog);
        c.f = N;

        c.execute_cycle();
        c.execute_cycle();
        assert_eq!(0x01, c.acc);
    }
}

#[cfg(test)]
mod branch_not_equal {
    use crate::*;

    #[test]
    fn relative_addressing() {
        let mut c = Chip::new();

        // Code:
        // BNE $01
        // LDA $01
        let prog: Vec<u8> = [0xD0, 0x01, 0x00, 0xA9, 0x01].to_vec();
        //                               ^ this value is not read
        c.load_program(prog);
        c.f = N;

        c.execute_cycle();
        c.execute_cycle();
        assert_eq!(0x01, c.acc);
    }
}

#[cfg(test)]
mod branch_on_plus {
    use crate::*;

    #[test]
    fn relative_addressing() {
        let mut c = Chip::new();

        // Code:
        // BPL $01
        // LDA $01
        let prog: Vec<u8> = [0x10, 0x01, 0x00, 0xA9, 0x01].to_vec();
        //                               ^ this value is not read
        c.load_program(prog);
        c.f = Z;

        c.execute_cycle();
        c.execute_cycle();
        assert_eq!(0x01, c.acc);
    }
}

#[cfg(test)]
mod branch_on_overflow_clear {
    use crate::*;

    #[test]
    fn relative_addressing() {
        let mut c = Chip::new();

        // Code:
        // BVC $01
        // LDA $01
        let prog: Vec<u8> = [0x50, 0x01, 0x00, 0xA9, 0x01].to_vec();
        //                               ^ this value is not read
        c.load_program(prog);
        c.f = Z;

        c.execute_cycle();
        c.execute_cycle();
        assert_eq!(0x01, c.acc);
    }
}

#[cfg(test)]
mod branch_on_overflow_set {
    use crate::*;

    #[test]
    fn relative_addressing() {
        let mut c = Chip::new();

        // Code:
        // BVS $01
        // LDA $01
        let prog: Vec<u8> = [0x70, 0x01, 0x00, 0xA9, 0x01].to_vec();
        //                               ^ this value is not read
        c.load_program(prog);
        c.f = V;

        c.execute_cycle();
        c.execute_cycle();
        assert_eq!(0x01, c.acc);
    }
}
