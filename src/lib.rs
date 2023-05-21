use chip::Chip;

pub mod chip;

pub fn run() {
    let mut c = Chip::new();

    c.load_exe("bin/6502_functional_test.bin".to_string())
        .unwrap();
    c.load_program([].to_vec());
    c.pc = 0x400; // 1024

    loop {
        c.execute_cycle();
    }
}