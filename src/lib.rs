use chip::Chip;

pub mod chip;

pub fn run() {
    let mut c = Chip::new();

    c.load_exe("bin/6502_functional_test.bin".to_string())
        .unwrap();
    c.load_program([].to_vec());
    c.startup(0x200);

    // for i in 0..MEMORY {
    //     println!("{:>04X}: {:>2X}", i,  c.memory[i]);
    // }

    loop {
        c.execute_cycle();
    }
}