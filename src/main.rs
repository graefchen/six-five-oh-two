use std::fs::File;
use std::io;
use std::io::prelude::*;

const MEMORY: usize = 65536;

// The flag for the flag in the Chip
// enum Flag {
//     N = 0x80, // [1000 0000] negative
//     V = 0x40, // [0100 0000] overflow
//               // [0010 0000] Reserved
//     B = 0x10, // [0001 0000] break
//     D = 0x08, // [0000 1000] decimale
//     I = 0x04, // [0000 0100] interrpt disable
//     Z = 0x02, // [0000 0010] zero
//     C = 0x00, // [0000 0001] carry
// }

struct Chip {
    // Registers:
    // Accumulator:
    // pub acc: u8,
    // // Index's x and y:
    // pub rx: u8,
    // pub ry: u8,
    // // Process Status flag:
    // pub f: Flag,
    // // Stack Pointer:
    // pub sp: u8,
    // Program Counter:
    pub pc: u16,
    // TODO: Putting Memory in it's own struct
    // Memory:
    // RESERVED: 256 bytes 0x0000 to 0x00FF -> Zero Page
    // RESERVED: 256 bytes 0x0100 to 0x01FF -> System Stack
    // PROGRAM DATA: 0x10000 - 0x206
    // RESERVED: last 6 bytes of memory
    pub memory: [u8; MEMORY],
}

impl Chip {
    pub fn new() -> Chip {
        Chip {
            // acc: 0,
            // rx: 0,
            // ry: 0,
            // f: Flag::Z,
            // sp: 0,
            pc: 0,
            memory: [0; MEMORY],
        }
    }
    pub fn load_exe(&mut self, file_path: String) -> io::Result<()> {
        let mut f = File::open(file_path)?;
        let mut buffer = Vec::new();

        f.read_to_end(&mut buffer)?;
        // println!("buffer.len() = {}", buffer.len());
        // println!("self.memory.len() = {}", self.memory.len());
        for i in 0..buffer.len() {
            // println!("i is {i}");
            // println!("{:>08b}", self.memory[i]);
            self.memory[i] = buffer[i];
        }
        Ok(())
    }

    pub fn execute_cycle(&mut self) {
        let opcode: u8 = read_word(self.memory, self.pc);
        self.process_opcode(opcode);
    }

    fn process_opcode(&mut self, opcode: u8) {
        // Format: AAA-BBB-CC
        // let aaa = opcode >> 5;
        // let bbb = (opcode & 0x1C) >> 3;
        // let cc = opcode & 0x03;

        // Getting High nibble and low nibble
        let op_1 = (opcode & 0xF0) >> 4;
        let op_2 = opcode & 0x0F;

        if self.pc < self.memory.len() as u16 {
            self.pc += 1;
        }

        // Very much long ... but I let it stay it that way ... for now ...
        match (op_1, op_2) {
            // BRK impl
            (0x0, 0x0) => {},
            // ORA X, ind
            (0x0, 0x1) => {},
            // FIXME: Illegal
            (0x0, 0x4) => {},
            // ORA zpg
            (0x0, 0x5) => {},
            // ASL zpg
            (0x0, 0x6) => {},
            // PHP impl
            (0x0, 0x8) => {},
            // ORA #
            (0x0, 0x9) => {},
            // ASL A
            (0x0, 0xA) => {},
            // FIXME: Illegal
            (0x0, 0xC) => {},
            // ORA abs
            (0x0, 0xD) => {},
            // ASL abs
            (0x0, 0xE) => {},

            // BPL rel
            (0x1, 0x0) => {},
            // ORA ind, Y
            (0x1, 0x1) => {},
            // FIXME: Illegal
            (0x1, 0x2) => {},
            // FIXME: Illegal
            (0x1, 0x4) => {},
            // ORA zpg, X
            (0x1, 0x5) => {},
            // ASL zpg, X
            (0x1, 0x6) => {},
            // CLC impl
            (0x1, 0x8) => {},
            // OR abs, y
            (0x1, 0x9) => {},
            // FIXME: Illegal
            (0x1, 0xA) => {},
            // FIXME: Illegal
            (0x1, 0xC) => {},
            // ORA abs, X
            (0x1, 0xD) => {},
            // ASL abs, X
            (0x1, 0xE) => {},

            // JSR abs
            (0x2, 0x0) => {},
            // AND X, ind
            (0x2, 0x1) => {},
            // BIT zpg
            (0x2, 0x4) => {},
            // AND #
            (0x2, 0x5) => {},
            // ROL zpg
            (0x2, 0x6) => {},
            // PLP impl
            (0x2, 0x8) => {},
            // AND #
            (0x2, 0x9) => {},
            // ROL A
            (0x2, 0xA) => {},
            // BIT abs
            (0x2, 0xC) => {},
            // AND abs
            (0x2, 0xD) => {},
            // ROL abs
            (0x2, 0xE) => {},

            // BMI rel
            (0x3, 0x0) => {},
            // AND ind, Y
            (0x3, 0x1) => {},
            // FIXME: Illegal
            (0x3, 0x4) => {},
            // AND zgp, X
            (0x3, 0x5) => {},
            // ROL zgp, X
            (0x3, 0x6) => {},
            // SEC impl
            (0x3, 0x8) => {},
            // AND abs, Y
            (0x3, 0x9) => {},
            // FIXME: Illegal
            (0x3, 0xA) => {},
            // FIXME: Illegal
            (0x3, 0xC) => {},
            // AND abs, X
            (0x3, 0xD) => {},
            // ROL abs, X
            (0x3, 0xE) => {},

            // RTI impl
            (0x4, 0x0) => {},
            // EOR X, ind
            (0x4, 0x1) => {},
            // FIXME: Illegal
            (0x4, 0x4) => {},
            // EOR zgp
            (0x4, 0x5) => {},
            // LSR zgp
            (0x4, 0x6) => {},
            // PHA impl
            (0x4, 0x8) => {},
            // EOR #
            (0x4, 0x9) => {},
            // LSR A
            (0x4, 0xA) => {},
            // JMP abs
            (0x4, 0xC) => {},
            // EOR abs
            (0x4, 0xD) => {},
            // LSR abs
            (0x4, 0xE) => {},

            // BVC rel
            (0x5, 0x0) => {},
            // EOR ind, y
            (0x5, 0x1) => {},
            // FIXME: Illegal
            (0x5, 0x4) => {},
            // EOR zgp,X
            (0x5, 0x5) => {},
            // LSR zgp,X
            (0x5, 0x6) => {},
            // CLI impl
            (0x5, 0x8) => {},
            // EOR abs, Y
            (0x5, 0x9) => {},
            // FIXME: Illegal
            (0x5, 0xA) => {},
            // FIXME: Illegal
            (0x5, 0xC) => {},
            // EOR abs, X
            (0x5, 0xD) => {},
            // LSR abs,X
            (0x5, 0xE) => {},

            // RTS impl
            (0x6, 0x0) => {},
            // ADC x, indx
            (0x6, 0x1) => {},
            // FIXME: Illegal
            (0x6, 0x4) => {},
            // ABC zpg
            (0x6, 0x5) => {},
            // ROR zgp
            (0x6, 0x6) => {},
            // PLA impl
            (0x6, 0x8) => {},
            // ADC #
            (0x6, 0x9) => {},
            // ROR A
            (0x6, 0xA) => {},
            // JMP ind
            (0x6, 0xC) => {},
            // ADB abs
            (0x6, 0xD) => {},
            // ROR abs
            (0x6, 0xE) => {},

            // BVS rel
            (0x7, 0x0) => {},
            // ADC ind, Y
            (0x7, 0x1) => {},
            // FIXME: Illegal
            (0x7, 0x2) => {},
            // FIXME: Illegal
            (0x7, 0x4) => {},
            // ADC zpg, X
            (0x7, 0x5) => {},
            // ROR zpg, X
            (0x7, 0x6) => {},
            // SEI impl
            (0x7, 0x8) => {},
            // ABD abs, Y
            (0x7, 0x9) => {},
            // FIXME: Illegal
            (0x7, 0xA) => {},
            // FIXME: Illegal
            (0x7, 0xC) => {},
            // ADC abs, X
            (0x7, 0xD) => {},
            // ROR abs, X
            (0x7, 0xE) => {},

            // FIXME: Illegal
            (0x8, 0x0) => {},
            // STR x, ind
            (0x8, 0x1) => {},
            // FIXME: Illegal
            (0x8, 0x2) => {},
            // STY zpg
            (0x8, 0x4) => {},
            // STA zpg
            (0x8, 0x5) => {},
            // STX zpg
            (0x8, 0x6) => {},
            // DAY impl
            (0x8, 0x8) => {},
            // FIXME: Illegal
            (0x8, 0x9) => {},
            // TXA impl
            (0x8, 0xA) => {},
            // STY abs
            (0x8, 0xC) => {},
            // STA abs
            (0x8, 0xD) => {},
            // STX abs
            (0x8, 0xE) => {},

            // BCC rel
            (0x9, 0x0) => {},
            // STA ind, Y
            (0x9, 0x1) => {},
            // FIXME: Illegal
            (0x9, 0x2) => {},
            // STY zgp, X
            (0x9, 0x4) => {},
            // STA zgp, X
            (0x9, 0x5) => {},
            // STX zgp, Y
            (0x9, 0x6) => {},
            // TYA impl
            (0x9, 0x8) => {},
            // LDA #
            (0x9, 0x9) => {},
            // TSX impl
            (0x9, 0xA) => {},
            // LDY abs
            (0x9, 0xC) => {},
            // LDA abs
            (0x9, 0xD) => {},
            // LDX abs
            (0x9, 0xE) => {},

            // LDY #
            (0xA, 0x0) => {},
            // LDA X, ind
            (0xA, 0x1) => {},
            // LDX #
            (0xA, 0x2) => {},
            // LDY zpg
            (0xA, 0x4) => {},
            // LDA zgp
            (0xA, 0x5) => {},
            // LDX zpg
            (0xA, 0x6) => {},
            // TAY impl
            (0xA, 0x8) => {},
            // LDA #
            (0xA, 0x9) => {},
            // TAX impl
            (0xA, 0xA) => {},
            // LDY abs
            (0xA, 0xC) => {},
            // LDA abs
            (0xA, 0xE) => {},
            // LDX abs
            (0xA, 0xD) => {},

            // BCS rel
            (0xB, 0x0) => {},
            // LDA ind, Y
            (0xB, 0x1) => {},
            // FIXME: Illegal
            (0xB, 0x2) => {},
            // LDY zpg, X
            (0xB, 0x4) => {},
            // LDA zpg, X
            (0xB, 0x5) => {},
            // LDX zpg, Y
            (0xB, 0x6) => {},
            // CLV impl
            (0xB, 0x8) => {},
            // LDA abs, Y
            (0xB, 0x9) => {},
            // TSV impl
            (0xB, 0xA) => {},
            // LDY abs, X
            (0xB, 0xC) => {},
            // LDA abs, X
            (0xB, 0xD) => {},
            // LDX abs, Y
            (0xB, 0xE) => {},

            // CPY #
            (0xC, 0x0) => {},
            // CMP x, ind
            (0xC, 0x1) => {},
            // FIXME: Illegal
            (0xC, 0x2) => {},
            // CPY zpg
            (0xC, 0x4) => {},
            // CPM zpg
            (0xC, 0x5) => {},
            // DEC zpg
            (0xC, 0x6) => {},
            // INY impl
            (0xC, 0x8) => {},
            // CMP #
            (0xC, 0x9) => {},
            // DEX impl
            (0xC, 0xA) => {},
            // CPY abs
            (0xC, 0xC) => {},
            // CMP abs
            (0xC, 0xD) => {},
            // DEC abs
            (0xC, 0xE) => {},

            // BNE rel
            (0xD, 0x0) => {},
            // CMP ind, Y
            (0xD, 0x1) => {},
            // FIXME: Illegal
            (0xD, 0x2) => {},
            // FIXME: Illegal
            (0xD, 0x4) => {},
            // CMP zpg, X
            (0xD, 0x5) => {},
            // DEC zpg, X
            (0xD, 0x6) => {},
            // CLD imp
            (0xD, 0x8) => {},
            // CLD impl
            (0xD, 0x9) => {},
            // FIXME: Illegal
            (0xD, 0xA) => {},
            // FIXME: Illegal
            (0xD, 0xC) => {},
            // CPM abs, X
            (0xD, 0xD) => {},
            // DEC abs, X
            (0xD, 0xE) => {},

            // CPX #
            (0xE, 0x0) => {},
            // SPX X, ind
            (0xE, 0x1) => {},
            // FIXME: Illegal
            (0xE, 0x2) => {},
            // CPX zpg
            (0xE, 0x4) => {},
            // SBC zpg
            (0xE, 0x5) => {},
            // INC zpg
            (0xE, 0x6) => {},
            // INX impl
            (0xE, 0x8) => {},
            // SBC #
            (0xE, 0x9) => {},
            // NOP impl
            (0xE, 0xA) => {},
            // CPX abs
            (0xE, 0xC) => {},
            // SBC abs
            (0xE, 0xD) => {},
            // INC abs
            (0xE, 0xE) => {},

            // BEQ rel
            (0xF, 0x0) => {},
            // SBC ind, Y
            (0xF, 0x1) => {},
            // FIXME: Illegal
            (0xF, 0x2) => {},
            // FIXME: Illegal
            (0xF, 0x4) => {},
            // SBC zpg, X
            (0xF, 0x5) => {},
            // INC zpg, X
            (0xF, 0x6) => {},
            // SED impl
            (0xF, 0x8) => {},
            // SBC abs, Y
            (0xF, 0x9) => {},
            // FIXME: Illegal
            (0xF, 0xA) => {},
            // FIXME: Illegal
            (0xF, 0xC) => {},
            // SBC abs, X
            (0xF, 0xD) => {},
            // INC abs, X
            (0xF, 0xE) => {},

            // TODO: revision after adding all opcodes
            (_, 0x2) => {},
            // TODO: revision after adding all opcodes
            (_, 0x3) => {},
            // TODO: revision after adding all opcodes
            (_, 0x7) => {},
            // TODO: revision after adding all opcodes
            (_, 0xB) => {},
            // TODO: revision after adding all opcodes
            (_, 0xF) => {},

            (_, _) => todo!("unknown upcode: hi:{:X}; low: {:X}", op_1, op_2),
        }
    }
}

fn read_word(memory: [u8; MEMORY], index: u16) -> u8 {
    memory[(index) as usize]
}

fn main() {
    // println!("Hello, world!");

    let mut c = Chip::new();

    c.load_exe("bin/6502_functional_test.bin".to_string())
        .unwrap();

    loop {
        c.execute_cycle();
    }
}
