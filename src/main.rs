use core::panic;
// Imports for reading a file
use std::fs::File;
use std::io;
use std::io::prelude::*;

const DEBUGLOG: bool = false;

const MEMORY: usize = 65536;

// The flag's for the flag in the Chip
// defined as u8 so to or them
const N: u8 = 0x80; // [1000 0000] negative
const V: u8 = 0x40; // [0100 0000] overflow
                    // [0010 0000] Reserved
const B: u8 = 0x10; // [0001 0000] break
const D: u8 = 0x08; // [0000 1000] decimale
const I: u8 = 0x04; // [0000 0100] interrpt disable
const Z: u8 = 0x02; // [0000 0010] zero
const C: u8 = 0x01; // [0000 0001] carry

#[derive(Debug)]
enum AddressMode {
    Accumulator,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Immediate,
    Implied,
    Indirect,
    XIndirect,
    IndirectY,
    Relative,
    Zeropage,
    ZeropageX,
    ZeropageY,
}

struct Chip {
    // Registers:
    // Accumulator:
    pub acc: u8,
    // Index's x and y:
    pub rx: u8,
    pub ry: u8,
    // Process Status flag:
    pub f: u8,
    // Stack Pointer:
    pub sp: u8,
    // Program Counter:
    pub pc: u16,
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
            acc: 0,
            rx: 0,
            ry: 0,
            f: 0,
            sp: 0,
            pc: 0x0200, // 256 + 256
            memory: [0; MEMORY],
        }
    }

    /// =====================
    /// Helper functions
    /// =====================

    fn push_stack(&mut self, address: u8) {
        if DEBUGLOG {
            println!("push_stack");
        }
        self.memory[0x0100 + self.sp as usize] = address;
        self.sp += 1;
    }

    fn pop_stack(&mut self) -> u8 {
        if DEBUGLOG {
            println!("pop_stack");
        }
        self.sp -= 1;
        let data = self.memory[0x0100 + self.sp as usize];
        data
    }

    fn read_byte(&mut self, address: u16) -> u8 {
        if DEBUGLOG {
            println!("read_byte");
        }
        self.memory[(address) as usize]
    }

    fn fetch_byte(&mut self) -> u8 {
        if DEBUGLOG {
            println!("fetch_byte");
        }
        let data = self.memory[(self.pc) as usize];
        self.pc += 1;
        data
    }

    fn write_byte(&mut self, byte: u8, address: u16) {
        if DEBUGLOG {
            println!("write_byte");
        }
        self.memory[address as usize] = byte;
    }

    fn read_word(&mut self, address: u16) -> u16 {
        if DEBUGLOG {
            println!("read_word");
        }
        let b1 = self.read_byte(address);
        let b2 = self.read_byte(address + 1);
        self.bytes_to_word(b1, b2)
    }

    fn fetch_word(&mut self) -> u16 {
        if DEBUGLOG {
            println!("fetch_word");
        }
        let ll = self.fetch_byte();
        let hh = self.fetch_byte();
        (ll as u16) + ((hh as u16) << 8)
    }

    fn write_word(&mut self, word: u16, address: u16) {
        if DEBUGLOG {
            println!("write_word");
        }
        let (ll, hh) = self.word_to_bytes(word);
        self.memory[address as usize] = ll;
        self.memory[(address + 1) as usize] = hh;
    }

    fn word_to_bytes(&self, word: u16) -> (u8, u8) {
        ( word as u8 & 0xFF , (word >> 8) as u8 )
    }

    fn bytes_to_word(&self, ll: u8, hh: u8) -> u16 {
        (ll as u16) + ((hh as u16) << 8)
    }

    /// Returns the address depending on the given AddreessMode
    fn get_address(&mut self, addr: AddressMode) -> u16 {
        match addr {
            AddressMode::Immediate => {
                return self.pc;
            }
            AddressMode::Absolute => {
                let address = self.fetch_word();
                return address;
            }
            AddressMode::Zeropage => {
                let ll = self.fetch_byte();
                let address = ll as u16;
                return address;
            }
            AddressMode::AbsoluteX => {
                let address = self.fetch_word();
                let x = self.rx;
                return address + x as u16;
            }
            AddressMode::AbsoluteY => {
                let address = self.fetch_word();
                let y = self.ry;
                return address + y as u16;
            }
            AddressMode::ZeropageX => {
                let ll = self.fetch_byte();
                let address = ll as u16;
                let x = self.rx;
                return address + x as u16;
            }
            AddressMode::ZeropageY => {
                let ll = self.fetch_byte();
                let address = ll as u16;
                let y = self.ry;
                return address + y as u16;
            }
            AddressMode::Indirect => {
                let address = self.fetch_word();
                let address2 = self.read_word(address);
                return address2;
            }
            AddressMode::XIndirect => {
                let ll = self.fetch_byte();
                let x = self.rx;
                let address = (ll + x) as u16;
                let address2 = self.read_word(address);
                return address2;
            }
            AddressMode::IndirectY => {
                let ll = self.fetch_byte();
                let y = self.ry;
                let address = ll as u16;
                let address2 = self.read_word(address) + y as u16;
                return address2;
            }
            AddressMode::Relative => {
                let off = self.read_byte(self.pc);
                return off as u16;
            }
            _ => { return 0; }
        }
    }

    pub fn load_program(&mut self, prog: Vec<u8>) {
        for i in 0..prog.len() {
            self.memory[0x200 + i] = prog[i];
        }
    }

    pub fn load_exe(&mut self, file_path: String) -> io::Result<()> {
        let mut f = File::open(file_path)?;
        let mut buffer = Vec::new();

        f.read_to_end(&mut buffer)?;
        // println!("buffer.len() = {}", buffer.len());
        // println!("{:?}", buffer);
        // println!("self.memory.len() = {}", self.memory.len());
        for i in 0..buffer.len() {
            // println!("i is {i}");
            // println!("{:>08b}", self.memory[i]);
            self.memory[i] = buffer[i];
        }
        Ok(())
    }

    pub fn execute_cycle(&mut self) {
        let opcode: u8 = self.fetch_byte();
        self.process_opcode(opcode);
    }

    fn process_opcode(&mut self, opcode: u8) {
        if DEBUGLOG {
            println!("Processsing Opcode '${:X}'", opcode)
        }

        // Format: AAA-BBB-CC
        // let aaa = opcode >> 5;
        // let bbb = (opcode & 0x1C) >> 3;
        // let cc = opcode & 0x03;

        // If we are still on memory we should make stuff ...
        if self.pc < (self.memory.len() - 0x206) as u16 {

        } else {
            panic!("We enter a loop");
        }

        // opcodes
        let op_1 = (opcode & 0xF0) >> 4;
        let op_2 = opcode & 0x0F;

        // println!("{:X} {:X}", op_1, op_2);

        match (op_1, op_2) {
            (0x0, 0x0) => self.brk(),
            (0x0, 0x1) => self.ora(AddressMode::XIndirect),
            (0x0, 0x5) => self.ora(AddressMode::Zeropage),
            (0x0, 0x6) => self.asl(AddressMode::Zeropage),
            (0x0, 0x8) => self.php(AddressMode::Implied),
            (0x0, 0x9) => self.ora(AddressMode::Immediate),
            (0x0, 0xA) => self.asl(AddressMode::Accumulator),
            (0x0, 0xD) => self.ora(AddressMode::Absolute),
            (0x0, 0xE) => self.asl(AddressMode::Absolute),
            (0x1, 0x0) => self.bpl(AddressMode::Relative),
            (0x1, 0x1) => self.ora(AddressMode::IndirectY),
            (0x1, 0x5) => self.ora(AddressMode::ZeropageX),
            (0x1, 0x6) => self.asl(AddressMode::ZeropageX),
            (0x1, 0x8) => self.clc(AddressMode::Implied),
            (0x1, 0x9) => self.ora(AddressMode::AbsoluteY),
            (0x1, 0xD) => self.ora(AddressMode::AbsoluteX),
            (0x1, 0xE) => self.asl(AddressMode::AbsoluteX),
            (0x2, 0x0) => self.jsr(),
            (0x2, 0x4) => self.bit(AddressMode::Zeropage),
            (0x2, 0x1) => self.and(AddressMode::XIndirect),
            (0x2, 0x5) => self.and(AddressMode::Zeropage),
            (0x2, 0x6) => self.rol(AddressMode::Zeropage),
            (0x2, 0x8) => self.plp(AddressMode::Implied),
            (0x2, 0x9) => self.and(AddressMode::Immediate),
            (0x2, 0xA) => self.rol(AddressMode::Accumulator),
            (0x2, 0xC) => self.bit(AddressMode::Absolute),
            (0x2, 0xD) => self.and(AddressMode::Absolute),
            (0x2, 0xE) => self.rol(AddressMode::Absolute),
            (0x3, 0x0) => self.bmi(AddressMode::Relative),
            (0x3, 0x1) => self.and(AddressMode::IndirectY),
            (0x3, 0x5) => self.and(AddressMode::ZeropageX),
            (0x3, 0x6) => self.rol(AddressMode::ZeropageX),
            (0x3, 0x8) => self.sec(AddressMode::Implied),
            (0x3, 0x9) => self.and(AddressMode::AbsoluteY),
            (0x3, 0xD) => self.and(AddressMode::AbsoluteX),
            (0x3, 0xE) => self.rol(AddressMode::AbsoluteX),
            (0x4, 0x0) => self.rti(AddressMode::Implied),
            (0x4, 0x1) => self.eor(AddressMode::XIndirect),
            (0x4, 0x5) => self.eor(AddressMode::Zeropage),
            (0x4, 0x6) => self.lsr(AddressMode::Zeropage),
            (0x4, 0x8) => self.pha(AddressMode::Implied),
            (0x4, 0x9) => self.eor(AddressMode::Immediate),
            (0x4, 0xA) => self.lsr(AddressMode::Accumulator),
            (0x4, 0xC) => self.jmp(AddressMode::Absolute),
            (0x4, 0xE) => self.lsr(AddressMode::Absolute),
            (0x4, 0xD) => self.eor(AddressMode::Absolute),
            (0x5, 0x0) => self.bvc(AddressMode::Relative),
            (0x5, 0x1) => self.eor(AddressMode::IndirectY),
            (0x5, 0x5) => self.eor(AddressMode::ZeropageX),
            (0x5, 0x6) => self.lsr(AddressMode::ZeropageX),
            (0x5, 0x8) => self.cli(AddressMode::Implied),
            (0x5, 0x9) => self.eor(AddressMode::AbsoluteY),
            (0x5, 0xD) => self.eor(AddressMode::AbsoluteX),
            (0x5, 0xE) => self.lsr(AddressMode::AbsoluteX),
            (0x6, 0x0) => self.rts(),
            (0x6, 0x1) => self.adc(AddressMode::XIndirect),
            (0x6, 0x5) => self.adc(AddressMode::Zeropage),
            (0x6, 0x6) => self.ror(AddressMode::Zeropage),
            (0x6, 0x8) => self.pla(AddressMode::Implied),
            (0x6, 0x9) => self.adc(AddressMode::Immediate),
            (0x6, 0xC) => self.jmp(AddressMode::Indirect),
            (0x6, 0xA) => self.ror(AddressMode::Accumulator),
            (0x6, 0xD) => self.adc(AddressMode::Absolute),
            (0x6, 0xE) => self.ror(AddressMode::Absolute),
            (0x7, 0x0) => self.bvs(AddressMode::Relative),
            (0x7, 0x1) => self.adc(AddressMode::IndirectY),
            (0x7, 0x5) => self.adc(AddressMode::ZeropageX),
            (0x7, 0x6) => self.ror(AddressMode::ZeropageX),
            (0x7, 0x8) => self.sei(AddressMode::Implied),
            (0x7, 0x9) => self.adc(AddressMode::AbsoluteY),
            (0x7, 0xD) => self.adc(AddressMode::AbsoluteX),
            (0x7, 0xE) => self.ror(AddressMode::AbsoluteX),
            (0x8, 0x1) => self.sta(AddressMode::XIndirect),
            (0x8, 0x4) => self.sty(AddressMode::Zeropage),
            (0x8, 0x5) => self.sta(AddressMode::Zeropage),
            (0x8, 0x6) => self.stx(AddressMode::Zeropage),
            (0x8, 0x8) => self.dey(),
            (0x8, 0xC) => self.sty(AddressMode::Absolute),
            (0x8, 0xA) => self.txa(AddressMode::Implied),
            (0x8, 0xD) => self.sta(AddressMode::Absolute),
            (0x8, 0xE) => self.stx(AddressMode::Absolute),
            (0x9, 0x0) => self.bcc(AddressMode::Relative),
            (0x9, 0x1) => self.sta(AddressMode::IndirectY),
            (0x9, 0x4) => self.sty(AddressMode::ZeropageX),
            (0x9, 0x5) => self.sta(AddressMode::ZeropageX),
            (0x9, 0x6) => self.stx(AddressMode::ZeropageY),
            (0x9, 0x8) => self.tya(AddressMode::Implied),
            (0x9, 0x9) => self.sta(AddressMode::AbsoluteY),
            (0x9, 0xD) => self.sta(AddressMode::AbsoluteX),
            (0x9, 0xA) => self.txs(AddressMode::Implied),
            (0xA, 0x0) => self.ldy(AddressMode::Immediate),
            (0xA, 0x1) => self.lda(AddressMode::XIndirect),
            (0xA, 0x2) => self.ldx(AddressMode::Immediate),
            (0xA, 0x4) => self.ldy(AddressMode::Zeropage),
            (0xA, 0x5) => self.lda(AddressMode::Zeropage),
            (0xA, 0x6) => self.ldx(AddressMode::Zeropage),
            (0xA, 0x8) => self.tay(AddressMode::Implied),
            (0xA, 0x9) => self.lda(AddressMode::Immediate),
            (0xA, 0xA) => self.tax(AddressMode::Implied),
            (0xA, 0xC) => self.ldy(AddressMode::Absolute),
            (0xA, 0xD) => self.lda(AddressMode::Absolute),
            (0xA, 0xE) => self.ldx(AddressMode::Absolute),
            (0xB, 0x0) => self.bcs(AddressMode::Relative),
            (0xB, 0x1) => self.lda(AddressMode::IndirectY),
            (0xB, 0x4) => self.ldy(AddressMode::ZeropageX),
            (0xB, 0x5) => self.lda(AddressMode::ZeropageX),
            (0xB, 0x6) => self.ldx(AddressMode::ZeropageY),
            (0xB, 0x8) => self.clv(AddressMode::Implied),
            (0xB, 0x9) => self.lda(AddressMode::AbsoluteY),
            (0xB, 0xA) => self.tsx(AddressMode::Implied),
            (0xB, 0xC) => self.ldy(AddressMode::AbsoluteX),
            (0xB, 0xD) => self.lda(AddressMode::AbsoluteX),
            (0xB, 0xE) => self.ldx(AddressMode::AbsoluteY),
            (0xC, 0x0) => self.cpy(AddressMode::Immediate),
            (0xC, 0x1) => self.cmp(AddressMode::XIndirect),
            (0xC, 0x4) => self.cpy(AddressMode::Zeropage),
            (0xC, 0x5) => self.cmp(AddressMode::Zeropage),
            (0xC, 0x6) => self.dec(AddressMode::Zeropage),
            (0xC, 0x8) => self.iny(),
            (0xC, 0x9) => self.cmp(AddressMode::Immediate),
            (0xC, 0xC) => self.cpy(AddressMode::Absolute),
            (0xC, 0xD) => self.cmp(AddressMode::Absolute),
            (0xC, 0xA) => self.dex(),
            (0xC, 0xE) => self.dec(AddressMode::Absolute),
            (0xD, 0x0) => self.bne(AddressMode::Relative),
            (0xD, 0x1) => self.cmp(AddressMode::IndirectY),
            (0xD, 0x5) => self.cmp(AddressMode::ZeropageX),
            (0xD, 0x6) => self.dec(AddressMode::ZeropageX),
            (0xD, 0x8) => self.cld(AddressMode::Implied),
            (0xD, 0x9) => self.cmp(AddressMode::AbsoluteY),
            (0xD, 0xD) => self.cmp(AddressMode::AbsoluteX),
            (0xD, 0xE) => self.dec(AddressMode::AbsoluteX),
            (0xE, 0x0) => self.cpx(AddressMode::Immediate),
            (0xE, 0x1) => self.sbc(AddressMode::XIndirect),
            (0xE, 0x4) => self.cpx(AddressMode::Zeropage),
            (0xE, 0x5) => self.sbc(AddressMode::Zeropage),
            (0xE, 0x6) => self.inc(AddressMode::Zeropage),
            (0xE, 0x8) => self.inx(),
            (0xE, 0x9) => self.sbc(AddressMode::Immediate),
            (0xE, 0xA) => self.nop(),
            (0xE, 0xC) => self.cpx(AddressMode::Absolute),
            (0xE, 0xD) => self.sbc(AddressMode::Absolute),
            (0xE, 0xE) => self.inc(AddressMode::Absolute),
            (0xF, 0x0) => self.beq(AddressMode::Relative),
            (0xF, 0x1) => self.sbc(AddressMode::IndirectY),
            (0xF, 0x5) => self.sbc(AddressMode::ZeropageX),
            (0xF, 0x8) => self.sed(AddressMode::Implied),
            (0xF, 0x9) => self.sbc(AddressMode::AbsoluteY),
            (0xF, 0xD) => self.sbc(AddressMode::AbsoluteX),
            (0xF, 0x6) => self.inc(AddressMode::ZeropageX),
            (0xF, 0xE) => self.inc(AddressMode::AbsoluteX),
            _ => {}
        }
    }

    /// ======================
    /// TRANSFER INSTRUCTIONS
    /// ======================

    // load accumulator
    fn lda(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("lda");
        };
        let address = self.get_address(addr);
        self.acc = self.read_byte(address);
        if self.acc == 0 {
            self.f = Z;
        }
    }

    // load X
    fn ldx(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("ldx");
        }
        let address = self.get_address(addr);
        self.rx = self.read_byte(address);
        if self.rx == 0 {
            self.f = Z;
        }
    }

    // load Y
    fn ldy(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("ldy");
        }
        let address = self.get_address(addr);
        self.ry = self.read_byte(address);
        if self.ry == 0 {
            self.f = Z;
        }
    }

    // store accumulator
    fn sta(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("sta");
        }
        todo!("sta");
    }

    // store X
    fn stx(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("stx");
        }
        todo!("stx");
    }

    // store Y
    fn sty(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("sty");
        }
        todo!("sty");
    }

    // transfer accumulator to X
    fn tax(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("tax");
        }
        todo!("tax");
    }

    // transfer accumulator to Y
    fn tay(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("tay");
        }
        todo!("tay");
    }

    // transfer stack pointer to X
    fn tsx(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("tsx");
        }
        todo!("tsx");
    }

    // transfer X to accumulator
    fn txa(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("txa");
        }
        todo!("txa");
    }

    // transfer X to stack pointer
    fn txs(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("txs");
        }
        todo!("txs");
    }

    // transfer Y to accumulator
    fn tya(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("tya");
        }
        todo!("tya");
    }

    /// ======================
    /// STACK INSTRUCTIONS
    /// ======================

    // push accumulator
    fn pha(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("pha");
        }
        todo!("pha");
    }

    // push processor status (SR)
    fn php(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("php");
        }
        todo!("php");
    }

    // pull accumulator
    fn pla(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("pla");
        }
        todo!("pla");
    }

    // pull processor status (SR)
    fn plp(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("plp");
        }
        todo!("plp");
    }

    /// ======================
    /// DECREMENTS & INCREMENTS
    /// ======================

    // decrement
    fn dec(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("dec");
        }
        let address = self.get_address(addr);
        let res = self.read_byte(address) - 1;
        self.write_byte(res, address);
        if res == 0 {
            self.f = Z;
        }
    }

    // decrement X
    fn dex(&mut self) {
        if DEBUGLOG {
            println!("dex");
        }
        self.rx -= 1;
        if self.rx == 0 {
            self.f = Z;
        }
    }

    // decrement Y
    fn dey(&mut self) {
        if DEBUGLOG {
            println!("dey");
        }
        self.ry -= 1;
        if self.ry == 0 {
            self.f = Z;
        }
    }

    // increment
    fn inc(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("inc");
        }
        let address = self.get_address(addr);
        let res = self.read_byte(address) + 1;
        self.write_byte(res, address);
        if res == 0 {
            self.f = Z;
        }
    }

    // increment X
    fn inx(&mut self) {
        if DEBUGLOG {
            println!("inx");
        }
        self.rx += 1;
        if self.rx  == 0 {
            self.f = Z;
        }
    }

    // increment Y
    fn iny(&mut self) {
        if DEBUGLOG {
            println!("iny");
        }
        self.ry += 1;
        if self.ry == 0 {
            self.f = Z;
        }
    }

    /// ======================
    /// ARITHMETIC OPERATIONS
    /// ======================

    // add with carry
    fn adc(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("adc");
        }
        todo!("adc");
    }

    // subtract with carry
    fn sbc(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("sbc");
        }
        todo!("sbc");
    }

    /// ======================
    /// ALOGICAL OPERATIONS
    /// ======================

    // and (with accumulator)
    fn and(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("and")
        }
        let address = self.get_address(addr);
        let and = self.read_byte(address);
        self.acc &= and;
        if self.acc == 0 {
            self.f = Z;
        }
    }

    // exclusive or (with accumulator)
    fn eor(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("eor")
        }
        todo!("eor");
    }

    // or with accumulator
    fn ora(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("ora");
        }
        todo!("ora");
    }

    /// ======================
    /// SHIFT & ROTATE INSTRUCTIONS
    /// ======================

    // arithmetic shift left
    fn asl(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("asl");
        }
        todo!("asl");
    }

    // logical shift right
    fn lsr(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("lsr");
        }
        todo!("lsr");
    }

    // rotate left
    fn rol(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("rol");
        }
        todo!("rol");
    }

    // rotate right
    fn ror(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("ror");
        }
        todo!("ror");
    }

    /// ======================
    /// FLAG INSTRUCTIONS
    /// ======================

    // clear carry
    fn clc(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("clc");
        }
        todo!("clc");
    }

    // clear decimal
    fn cld(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("cld");
        }
        todo!("cld");
    }

    // clear interrupt disable
    fn cli(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("cli");
        }
        todo!("cli");
    }

    // clear overflow
    fn clv(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("clv");
        }
        todo!("clv");
    }

    fn sec(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("sec");
        }
        todo!("sec");
    }

    // set decimal
    fn sed(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("sed");
        }
        todo!("sed");
    }

    // set interrupt disable
    fn sei(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("sei");
        }
        todo!("sei");
    }

    /// ======================
    /// COMPARISON
    /// ======================

    // compare (with accumulator)
    fn cmp(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("cmp");
        }
        todo!("cmp");
    }

    // compare with X
    fn cpx(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("cpx");
        }
        todo!("cpx");
    }

    // compare with Y
    fn cpy(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("cpy");
        }
        todo!("cpy");
    }

    /// ======================
    /// CONDITIONAL BRANCH INSTRUCTION
    /// ======================

    // branch on carry clear
    fn bcc(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("bcc");
        }
        todo!("bcc");
    }

    // branch on carry set
    fn bcs(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("bcs");
        }
        todo!("bcs");
    }

    // branch on equal (zero set)
    fn beq(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("beq");
        }
        todo!("beq");
    }

    // branch on minus (negative set)
    fn bmi(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("bmi");
        }
        todo!("bmi");
    }

    // branch on not equal (zero clear)
    fn bne(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("bne");
        }
        todo!("bne");
    }

    // branch on plus (negative clear)
    fn bpl(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("bpl");
        }
        todo!("bpl");
    }

    // branch on overflow clear
    fn bvc(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("bvc");
        }
        todo!("bvc");
    }

    // branch on overflow set
    fn bvs(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("bvs");
        }
        todo!("bvs");
    }

    /// ======================
    /// JUMP & SUBROUTINES
    /// ======================

    // jump
    fn jmp(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("jmp");
        }
        self.pc = self.get_address(addr);
    }

    // jump subroutine
    fn jsr(&mut self) {
        if DEBUGLOG {
            println!("jsr");
        }
        let subaddr = self.fetch_word();
        // The write_word function is more or less the same as the
        // Used push_stack functions...
        // I just prefer the push stack a little bit more
        // Else it does exactly the same job
        // self.write_word(self.pc, self.sp as u16);
        // self.sp += 2;
        let (ll, hh) = self.word_to_bytes(self.pc);
        self.push_stack(ll);
        self.push_stack(hh);

        self.pc = subaddr;
    }

    // return from subroutine
    fn rts(&mut self) {
        if DEBUGLOG {
            println!("rts");
        }
        // The read_word function is more or less the same as the
        // Used pop_stack functions...
        // I just prefer the pop stack a little bit more
        // Else it does exactly the same job
        // self.sp -= 2;
        // self.pc = self.read_word(self.sp as u16);
        let hh = self.pop_stack();
        let ll = self.pop_stack();
        self.bytes_to_word(ll, hh);
        self.pc = self.bytes_to_word(ll, hh);
    }

    /// ======================
    /// INTERRUPTS
    /// ======================

    // break / interrupt
    /// Force Break
    fn brk(&mut self) {
        if DEBUGLOG {
            println!("brk")
        }
        self.push_stack(self.memory[(self.pc + 2) as usize]);
        self.f = B;
        todo!("brk");
    }

    // return from interrupt
    fn rti(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("rti");
        }
        todo!("rti");
    }

    /// ======================
    /// OTHER
    /// ======================

    // bit test
    fn bit(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("bit");
        }
        todo!("bit");
    }

    // no operation
    fn nop(&mut self) {
        if DEBUGLOG {
            println!("nop");
        }
        // A simple empty field
        // because what says truly more than
        // no operation than simply
        // not doing anything?
    }
}

fn main() {
    // println!("Hello, world!");

    let mut c = Chip::new();

    c.load_exe("bin/6502_functional_test.bin".to_string())
        .unwrap();

    // loop {
    //     c.execute_cycle();
    // }
}

/// ==========================
/// TRANSFER INSTRUCTIONS TEST
/// ==========================

#[cfg(test)]
mod load_accumulator {
    use crate::*;

    #[test]
    fn immediate_mode() {
        let mut c = Chip::new();

        let prog: Vec<u8> = [0xA9, 0x01].to_vec();
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(0x01, c.acc);
    }

    #[test]
    fn zeropage_mode() {
        let mut c = Chip::new();

        let prog: Vec<u8> = [0xA5, 0x01].to_vec();
        c.memory[0x01] = 0x12;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(0x12, c.acc);
    }

    #[test]
    fn zeropage_x_mode() {
        let mut c = Chip::new();

        let prog: Vec<u8> = [0xB5, 0x01].to_vec();
        c.rx = 0x01;
        c.memory[0x02] = 0x12;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(0x12, c.acc);
    }
    
    #[test]
    fn absolute_mode() {
        let mut c = Chip::new();

        let prog: Vec<u8> = [0xAD, 0xF0, 0xFF].to_vec();
        c.memory[0xFFF0] = 0x12;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(0x12, c.acc);
    }

    #[test]
    fn absolute_x_mode() {
        let mut c = Chip::new();

        let prog: Vec<u8> = [0xBD, 0xF0, 0xFF].to_vec();
        c.rx = 0x2;
        c.memory[0xFFF2] = 0x12;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(0x12, c.acc);
    }

    #[test]
    fn absolute_y_mode() {
        let mut c = Chip::new();

        let prog: Vec<u8> = [0xB9, 0xF0, 0xFF].to_vec();
        c.ry = 0x5;
        c.memory[0xFFF5] = 0x12;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(0x12, c.acc);
    }

    // #[test]
    // fn indirect_x_mode() {

    // }

    // #[test]
    // fn indirect_y_mode() {

    // }

    #[test]
    fn flags() {
        let mut c = Chip::new();

        let prog: Vec<u8> = [0xA9, 0x01].to_vec();
        c.load_program(prog);

        c.execute_cycle();
        assert_ne!(Z, c.f);
    }
}

#[cfg(test)]
mod load_x {
    use crate::*;

    #[test]
    fn immediate_mode() {
        let mut c = Chip::new();

        let prog: Vec<u8> = [0xA2, 0x01].to_vec();
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(0x01, c.rx);
    }

    #[test]
    fn zeropage_mode() {
        let mut c = Chip::new();

        let prog: Vec<u8> = [0xA6, 0x01].to_vec();
        c.memory[0x01] = 0x12;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(0x12, c.rx);
    }

    #[test]
    fn zeropage_y_mode() {
        let mut c = Chip::new();

        let prog: Vec<u8> = [0xB6, 0x01].to_vec();
        c.ry = 0x4;
        c.memory[0x05] = 0x12;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(0x12, c.rx);
    }

    #[test]
    fn absolute_mode() {
        let mut c = Chip::new();

        let prog: Vec<u8> = [0xAE, 0xF0, 0xFF].to_vec();
        c.memory[0xFFF0] = 0x12;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(0x12, c.rx);
    }

    // #[test]
    // fn absolute_y_mode() {

    // }

    #[test]
    fn flags() {
        let mut c = Chip::new();

        let prog: Vec<u8> = [0xA2, 0x01].to_vec();
        c.load_program(prog);

        c.execute_cycle();
        assert_ne!(Z, c.f);
    }
}

#[cfg(test)]
mod load_y {
    use crate::*;

    #[test]
    fn immediate_mode() {
        let mut c = Chip::new();

        let prog: Vec<u8> = [0xA0, 0x01].to_vec();
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(0x01, c.ry);
    }

    #[test]
    fn zeropage_mode() {
        let mut c = Chip::new();

        let prog: Vec<u8> = [0xA4, 0x01].to_vec();
        c.memory[0x01] = 0x12;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(0x12, c.ry);
    }

    #[test]
    fn zeropage_x_mode() {
        let mut c = Chip::new();

        let prog: Vec<u8> = [0xB4, 0x01].to_vec();
        c.rx = 0x0A;
        c.memory[0x0B] = 0x12;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(0x12, c.ry);
    }

    #[test]
    fn absolute_mode() {
        let mut c = Chip::new();

        let prog: Vec<u8> = [0xAC, 0xF0, 0xFF].to_vec();
        c.memory[0xFFF0] = 0x12;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(0x12, c.ry);
    }

    // #[test]
    // fn absolute_x_mode() {

    // }

    #[test]
    fn flags() {
        let mut c = Chip::new();

        let prog: Vec<u8> = [0xA0, 0x01].to_vec();
        c.load_program(prog);

        c.execute_cycle();
        assert_ne!(Z, c.f);
    }
}


/// ==============================
/// DECREMENNT & INCREMENNT TESTS
/// ==============================

#[cfg(test)]
mod decrement {
    use crate::*;

    #[test]
    fn zeropage_mode() {
        let mut c = Chip::new();

        let prog: Vec<u8> = [0xC6, 0x01].to_vec();
        c.memory[0x01] = 0x12;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(c.memory[0x01], 0x11);
    }

    #[test]
    fn zeropage_x_mode() {
        let mut c = Chip::new();

        let prog: Vec<u8> = [0xD6, 0x01].to_vec();
        c.rx = 0x01;
        c.memory[0x02] = 0x13;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(c.memory[0x02], 0x12);
    }

    #[test]
    fn absolute_mode() {
        let mut c = Chip::new();

        let prog: Vec<u8> = [0xCE, 0xF0, 0xFF].to_vec();
        c.memory[0xFFF0] = 0xAA;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(c.memory[0xFFF0], 0xA9);
    }

    #[test]
    fn absolute_x_mode() {
        let mut c = Chip::new();

        let prog: Vec<u8> = [0xDE, 0xF0, 0xF0].to_vec();
        c.rx = 0x03;
        c.memory[0xF0F3] = 0xAA;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(c.memory[0xF0F3], 0xA9);
    }

    #[test]
    fn flag() {
        let mut c = Chip::new();

        let prog: Vec<u8> = [0xC6, 0x01].to_vec();
        c.memory[0x01] = 0x01;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(Z, c.f);
    }
}

#[cfg(test)]
mod decrement_x {
    use crate::*;

    #[test]
    fn implied() {
        let mut c = Chip::new();

        let prog: Vec<u8> = [0xCA].to_vec();
        c.rx = 0x01;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(c.rx, 0x00);
    }

    #[test]
    fn flag() {
        let mut c = Chip::new();

        let prog: Vec<u8> = [0xCA].to_vec();
        c.rx = 0x01;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(Z, c.f);
    }
}

#[cfg(test)]
mod decrement_y {
    use crate::*;

    #[test]
    fn implied() {
        let mut c = Chip::new();

        let prog: Vec<u8> = [0x88].to_vec();
        c.ry = 0x01;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(c.rx, 0x00);
    }

    #[test]
    fn flag() {
        let mut c = Chip::new();

        let prog: Vec<u8> = [0x88].to_vec();
        c.ry = 0x01;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(Z, c.f);
    }
}

#[cfg(test)]
mod increment {
    use crate::*;

    #[test]
    fn zeropage_mode() {
        let mut c = Chip::new();

        let prog: Vec<u8> = [0xE6, 0x01].to_vec();
        c.memory[0x01] = 0x12;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(c.memory[0x01], 0x13);
    }

    #[test]
    fn zeropage_x_mode() {
        let mut c = Chip::new();

        let prog: Vec<u8> = [0xF6, 0x01].to_vec();
        c.rx = 0x01;
        c.memory[0x02] = 0x14;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(c.memory[0x02], 0x15);
    }

    #[test]
    fn absolute_mode() {
        let mut c = Chip::new();

        let prog: Vec<u8> = [0xEE, 0xF0, 0xFF].to_vec();
        c.memory[0xFFF0] = 0xAA;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(c.memory[0xFFF0], 0xAB);
    }

    #[test]
    fn absolute_x_mode() {
        let mut c = Chip::new();

        let prog: Vec<u8> = [0xFE, 0xF0, 0xF0].to_vec();
        c.rx = 0x03;
        c.memory[0xF0F3] = 0xAA;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(c.memory[0xF0F3], 0xAB);
    }

    #[test]
    fn flag() {
        let mut c = Chip::new();

        let prog: Vec<u8> = [0xE6, 0x01].to_vec();
        c.memory[0x01] = 0x01;
        c.load_program(prog);

        c.execute_cycle();
        assert_ne!(Z, c.f);
    }
}

#[cfg(test)]
mod increment_x {
    use crate::*;

    #[test]
    fn implied() {
        let mut c = Chip::new();

        let prog: Vec<u8> = [0xE8].to_vec();
        c.rx = 0x11;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(c.rx, 0x12);
    }

    #[test]
    fn flag() {
        let mut c = Chip::new();

        let prog: Vec<u8> = [0xE8].to_vec();
        c.ry = 0x01;
        c.load_program(prog);

        c.execute_cycle();
        assert_ne!(Z, c.f);
    }
}

#[cfg(test)]
mod increment_y {
    use crate::*;

    #[test]
    fn implied() {
        let mut c = Chip::new();

        let prog: Vec<u8> = [0xC8].to_vec();
        c.ry = 0x02;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(c.ry, 0x03);
    }

    #[test]
    fn flag() {
        let mut c = Chip::new();

        let prog: Vec<u8> = [0xC8].to_vec();
        c.ry = 0x01;
        c.load_program(prog);

        c.execute_cycle();
        assert_ne!(Z, c.f);
    }
}

/// ==========================
/// JUMP & SUBROUTINE TESTS
/// ==========================

#[cfg(test)]
mod jump {
    use crate::*;

    #[test]
    fn to_new_location_absolute() {
        let mut c = Chip::new();

        let prog: Vec<u8> = [0x4C, 0x40, 0x42].to_vec();
        c.load_program(prog);
        c.memory[0x4240] = 0xA9;
        c.memory[0x4241] = 0xFF;

        c.execute_cycle();
        c.execute_cycle();
        assert_eq!(0xFF, c.acc);
    }

    #[test]
    fn to_new_location_indirect() {
        let mut c = Chip::new();

        let prog: Vec<u8> = [0x6C, 0x82, 0xFF].to_vec();
        c.load_program(prog);
        c.memory[0x4240] = 0xA9;
        c.memory[0x4241] = 0xFF;
        c.memory[0xFF82] = 0x40;
        c.memory[0xFF83] = 0x42;

        c.execute_cycle();
        c.execute_cycle();
        assert_eq!(0xFF, c.acc);
    }

    #[test]
    fn to_new_location_saving_return_address() {
        let mut c = Chip::new();

        let prog: Vec<u8> = [0x20, 0x40, 0x42].to_vec();
        c.load_program(prog);
        c.memory[0x4240] = 0xA9;
        c.memory[0x4241] = 0xFF;

        c.execute_cycle();
        c.execute_cycle();
        assert_eq!(0xFF, c.acc);
        assert_eq!(c.memory[0x0100], 0x03);
        assert_eq!(c.memory[0x0101], 0x02);
    }

    #[test]
    fn return_jump() {
        let mut c = Chip::new();

        let prog: Vec<u8> = [0x20, 0x40, 0x42, 0xA9, 0xF0].to_vec();
        c.load_program(prog);
        c.memory[0x4240] = 0xA9;
        c.memory[0x4241] = 0xFF;
        c.memory[0x4242] = 0x60;

        c.execute_cycle();
        c.execute_cycle();
        c.execute_cycle();
        c.execute_cycle();
        assert_eq!(0xFF, c.acc);
        c.execute_cycle();
        assert_eq!(0xF0, c.acc);
    }
}
