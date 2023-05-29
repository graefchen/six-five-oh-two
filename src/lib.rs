use chip::Chip;

pub mod chip;

pub fn run_testprogramm() {
    let mut c = Chip::new();
    c.load_exe("bin/6502_functional_test.bin".to_string(), 0x000A)
        .unwrap();
    c.load_program([].to_vec());
    c.pc = 0x400; // 1024

    loop {
        c.execute_cycle();
    }
}

pub fn run() {
    let mut c = Chip::new();

    c.load_program([0x80].to_vec());
}
