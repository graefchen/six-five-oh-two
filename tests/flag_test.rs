use sixfiveohtwo::chip::*;

/// ==========================
/// FLAG TESTS
/// ==========================

#[cfg(test)]
mod clear_carry {

    use crate::*;

    #[test]
    fn implied_addressing() {
        let mut c = Chip::new();

        // Code:
        // CLC
        let prog: Vec<u8> = [0x18].to_vec();
        c.f = C;
        c.load_program(prog);
        c.startup(0x0200);

        c.execute_cycle();
        assert_eq!(c.f, 0x0);
    }
}

#[cfg(test)]
mod clear_decimal {
    use crate::*;

    #[test]
    fn implied_addressing() {
        let mut c = Chip::new();

        // Code:
        // CLD
        let prog: Vec<u8> = [0xD8].to_vec();
        c.f = D;
        c.load_program(prog);
        c.startup(0x0200);

        c.execute_cycle();
        assert_eq!(c.f, 0x0);
    }
}

#[cfg(test)]
mod clear_interrupt_disable {
    use crate::*;

    #[test]
    fn implied_addressing() {
        let mut c = Chip::new();

        // Code:
        // CLI
        let prog: Vec<u8> = [0x58].to_vec();
        c.f = I;
        c.load_program(prog);
        c.startup(0x0200);

        c.execute_cycle();
        assert_eq!(c.f, 0x0);
    }
}

#[cfg(test)]
mod clear_overflow {
    use crate::*;

    #[test]
    fn implied_addressing() {
        let mut c = Chip::new();

        // Code:
        // CLV
        let prog: Vec<u8> = [0xB8].to_vec();
        c.f = V;
        c.load_program(prog);
        c.startup(0x0200);

        c.execute_cycle();
        assert_eq!(c.f, 0x0);
    }
}

#[cfg(test)]
mod set_carry {
    use crate::*;

    #[test]
    fn implied_addressing() {
        let mut c = Chip::new();

        // Code:
        // SEC
        let prog: Vec<u8> = [0x38].to_vec();
        c.load_program(prog);
        c.startup(0x0200);

        c.execute_cycle();
        assert_eq!(c.f, C)
    }
}

#[cfg(test)]
mod set_decimal {
    use crate::*;

    #[test]
    fn implied_addressing() {
        let mut c = Chip::new();

        // Code:
        // SED
        let prog: Vec<u8> = [0xF8].to_vec();
        c.load_program(prog);
        c.startup(0x0200);

        c.execute_cycle();
        assert_eq!(c.f, D);
    }
}

#[cfg(test)]
mod set_interrupt_disable {
    use crate::*;

    #[test]
    fn implied_addressing() {
        let mut c = Chip::new();

        // Code:
        // SEI
        let prog: Vec<u8> = [0x78].to_vec();
        c.load_program(prog);
        c.startup(0x0200);

        c.execute_cycle();
        assert_eq!(c.f, I);
    }
}
