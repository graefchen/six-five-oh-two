// TODO: MORE TESTS
// TODO: MORE COMMENTS
// TODO: MORE DOCUMENTATION
// TODO: RELOOK AT THE FLAG SETTING

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
const I: u8 = 0x04; // [0000 0100] interupt disable
const Z: u8 = 0x02; // [0000 0010] zero
const C: u8 = 0x01; // [0000 0001] carry

#[derive(Debug, PartialEq, Copy, Clone)]
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
    // $FFFA, $FFFB ... NMI (Non-Maskable Interrupt) vector, 16-bit (LB, HB)
    // $FFFC, $FFFD ... RES (Reset) vector, 16-bit (LB, HB)
    // $FFFE, $FFFF ... IRQ (Interrupt Request) vector, 16-bit (LB, HB)
    // Last 6 bytes therefor be: [ ]
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

    pub fn startup(&mut self, address: u16) {
        self.pc = self.read_word(address);
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
            println!("read_byte at position {:X}", address);
        }
        self.memory[(address) as usize]
    }

    fn fetch_byte(&mut self) -> u8 {
        if DEBUGLOG {
            println!("fetch_byte at position {:X}", self.pc);
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
            println!("read_word at position {:X} and {:X}", address, address + 1);
        }
        let b1 = self.read_byte(address);
        let b2 = self.read_byte(address + 1);
        self.bytes_to_word(b1, b2)
    }

    fn fetch_word(&mut self) -> u16 {
        if DEBUGLOG {
            println!("fetch_word at position {:X} and {:X}", self.pc, self.pc + 1);
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

    fn  set_zero_neg_flags(&mut self, value: u8) {
        if value == 0 {
            self.f = Z;
        }
        if value > 0x80 {
            self.f = N;
        }
    }

    /// Returns the address depending on the given AddressMode.
    /// 
    /// 
    /// TODO: Refactor this into an own function that I do not need to use
    ///       this weird algorithm in getting first the address and then
    ///       reading from the address like:
    /// ```rust
    /// let address = get_address(addr);
    /// let byte = read_byte(address);
    /// ````
    fn get_address(&mut self, addr: AddressMode) -> u16 {
        if DEBUGLOG {
            println!("get_address with AddressMode: {:?}", addr);
        }
        match addr {
            AddressMode::Immediate => {
                // Need to increment the pc
                // Else it would not register that we have
                // "read" the 
                self.pc += 1;
                return self.pc - 1;
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
                return self.read_word(address);
            }
            AddressMode::IndirectY => {
                let ll = self.fetch_byte();
                let y = self.ry;
                let address = ll as u16;
                let address2 = self.read_word(address) + y as u16;
                return address2;
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

        // opcodes
        let op_1 = (opcode & 0xF0) >> 4;
        let op_2 = opcode & 0x0F;

        // println!("{:X} {:X}", op_1, op_2);

        match (op_1, op_2) {
            (0x0, 0x0) => self.brk(),
            (0x0, 0x1) => self.ora(AddressMode::XIndirect),
            (0x0, 0x5) => self.ora(AddressMode::Zeropage),
            (0x0, 0x6) => self.asl(AddressMode::Zeropage),
            (0x0, 0x8) => self.php(),
            (0x0, 0x9) => self.ora(AddressMode::Immediate),
            (0x0, 0xA) => self.asl(AddressMode::Accumulator),
            (0x0, 0xD) => self.ora(AddressMode::Absolute),
            (0x0, 0xE) => self.asl(AddressMode::Absolute),
            (0x1, 0x0) => self.bpl(),
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
            (0x2, 0x8) => self.plp(),
            (0x2, 0x9) => self.and(AddressMode::Immediate),
            (0x2, 0xA) => self.rol(AddressMode::Accumulator),
            (0x2, 0xC) => self.bit(AddressMode::Absolute),
            (0x2, 0xD) => self.and(AddressMode::Absolute),
            (0x2, 0xE) => self.rol(AddressMode::Absolute),
            (0x3, 0x0) => self.bmi(),
            (0x3, 0x1) => self.and(AddressMode::IndirectY),
            (0x3, 0x5) => self.and(AddressMode::ZeropageX),
            (0x3, 0x6) => self.rol(AddressMode::ZeropageX),
            (0x3, 0x8) => self.sec(AddressMode::Implied),
            (0x3, 0x9) => self.and(AddressMode::AbsoluteY),
            (0x3, 0xD) => self.and(AddressMode::AbsoluteX),
            (0x3, 0xE) => self.rol(AddressMode::AbsoluteX),
            (0x4, 0x0) => self.rti(),
            (0x4, 0x1) => self.eor(AddressMode::XIndirect),
            (0x4, 0x5) => self.eor(AddressMode::Zeropage),
            (0x4, 0x6) => self.lsr(AddressMode::Zeropage),
            (0x4, 0x8) => self.pha(),
            (0x4, 0x9) => self.eor(AddressMode::Immediate),
            (0x4, 0xA) => self.lsr(AddressMode::Accumulator),
            (0x4, 0xC) => self.jmp(AddressMode::Absolute),
            (0x4, 0xE) => self.lsr(AddressMode::Absolute),
            (0x4, 0xD) => self.eor(AddressMode::Absolute),
            (0x5, 0x0) => self.bvc(),
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
            (0x6, 0x8) => self.pla(),
            (0x6, 0x9) => self.adc(AddressMode::Immediate),
            (0x6, 0xC) => self.jmp(AddressMode::Indirect),
            (0x6, 0xA) => self.ror(AddressMode::Accumulator),
            (0x6, 0xD) => self.adc(AddressMode::Absolute),
            (0x6, 0xE) => self.ror(AddressMode::Absolute),
            (0x7, 0x0) => self.bvs(),
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
            (0x8, 0xA) => self.txa(),
            (0x8, 0xD) => self.sta(AddressMode::Absolute),
            (0x8, 0xE) => self.stx(AddressMode::Absolute),
            (0x9, 0x0) => self.bcc(),
            (0x9, 0x1) => self.sta(AddressMode::IndirectY),
            (0x9, 0x4) => self.sty(AddressMode::ZeropageX),
            (0x9, 0x5) => self.sta(AddressMode::ZeropageX),
            (0x9, 0x6) => self.stx(AddressMode::ZeropageY),
            (0x9, 0x8) => self.tya(),
            (0x9, 0x9) => self.sta(AddressMode::AbsoluteY),
            (0x9, 0xA) => self.txs(),
            (0x9, 0xD) => self.sta(AddressMode::AbsoluteX),
            (0xA, 0x0) => self.ldy(AddressMode::Immediate),
            (0xA, 0x1) => self.lda(AddressMode::XIndirect),
            (0xA, 0x2) => self.ldx(AddressMode::Immediate),
            (0xA, 0x4) => self.ldy(AddressMode::Zeropage),
            (0xA, 0x5) => self.lda(AddressMode::Zeropage),
            (0xA, 0x6) => self.ldx(AddressMode::Zeropage),
            (0xA, 0x8) => self.tay(),
            (0xA, 0x9) => self.lda(AddressMode::Immediate),
            (0xA, 0xA) => self.tax(),
            (0xA, 0xC) => self.ldy(AddressMode::Absolute),
            (0xA, 0xD) => self.lda(AddressMode::Absolute),
            (0xA, 0xE) => self.ldx(AddressMode::Absolute),
            (0xB, 0x0) => self.bcs(),
            (0xB, 0x1) => self.lda(AddressMode::IndirectY),
            (0xB, 0x4) => self.ldy(AddressMode::ZeropageX),
            (0xB, 0x5) => self.lda(AddressMode::ZeropageX),
            (0xB, 0x6) => self.ldx(AddressMode::ZeropageY),
            (0xB, 0x8) => self.clv(AddressMode::Implied),
            (0xB, 0x9) => self.lda(AddressMode::AbsoluteY),
            (0xB, 0xA) => self.tsx(),
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
            (0xD, 0x0) => self.bne(),
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
            (0xF, 0x0) => self.beq(),
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
        self.set_zero_neg_flags(self.acc);
    }

    // load X
    fn ldx(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("ldx");
        }
        let address = self.get_address(addr);
        self.rx = self.read_byte(address);
        self.set_zero_neg_flags(self.rx);
    }

    // load Y
    fn ldy(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("ldy");
        }
        let address = self.get_address(addr);
        self.ry = self.read_byte(address);
        self.set_zero_neg_flags(self.ry);
    }

    // store accumulator
    fn sta(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("sta");
        }
        let address = self.get_address(addr);
        self.write_byte(self.acc, address);
    }

    // store X
    fn stx(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("stx");
        }
        let address = self.get_address(addr);
        self.write_byte(self.rx, address);
    }

    // store Y
    fn sty(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("sty");
        }
        let address = self.get_address(addr);
        self.write_byte(self.ry, address);
    }

    // transfer accumulator to X
    fn tax(&mut self) {
        if DEBUGLOG {
            println!("tax");
        }
        self.rx = self.acc;
    }

    // transfer accumulator to Y
    fn tay(&mut self) {
        if DEBUGLOG {
            println!("tay");
        }
        self.ry = self.acc;
    }

    // transfer stack pointer to X
    fn tsx(&mut self) {
        if DEBUGLOG {
            println!("tsx");
        }
        self.rx = self.sp;
    }

    // transfer X to accumulator
    fn txa(&mut self) {
        if DEBUGLOG {
            println!("txa");
        }
        self.acc = self.rx;
    }

    // transfer X to stack pointer
    fn txs(&mut self) {
        if DEBUGLOG {
            println!("txs");
        }
        self.sp = self.rx;
    }

    // transfer Y to accumulator
    fn tya(&mut self) {
        if DEBUGLOG {
            println!("tya");
        }
        self.acc = self.ry;
    }

    /// ======================
    /// STACK INSTRUCTIONS
    /// ======================

    // push accumulator
    fn pha(&mut self) {
        if DEBUGLOG {
            println!("pha");
        }
        self.push_stack(self.acc);
    }

    // push processor status (SR)
    fn php(&mut self) {
        if DEBUGLOG {
            println!("php");
        }
        self.push_stack(self.f);
    }

    // pull accumulator
    fn pla(&mut self) {
        if DEBUGLOG {
            println!("pla");
        }
        self.acc = self.pop_stack();
        self.set_zero_neg_flags(self.acc);
    }

    // pull processor status (SR)
    fn plp(&mut self) {
        if DEBUGLOG {
            println!("plp");
        }
        self.f = self.pop_stack();
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
        let byte = self.read_byte(address);
        let ( res, _ ) = byte.overflowing_sub(1);
        self.write_byte(res, address);
        self.set_zero_neg_flags(res);
    }

    // decrement X
    fn dex(&mut self) {
        if DEBUGLOG {
            println!("dex");
        }
        ( self.rx, _ ) = self.rx.overflowing_sub(1);
        self.set_zero_neg_flags(self.rx);
    }

    // decrement Y
    fn dey(&mut self) {
        if DEBUGLOG {
            println!("dey");
        }
        ( self.ry, _ ) = self.ry.overflowing_sub(1);
        self.set_zero_neg_flags(self.ry);
    }

    // increment
    fn inc(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("inc");
        }
        let address = self.get_address(addr);
        let byte = self.read_byte(address);
        let ( res, _ ) = byte.overflowing_add(1);
        self.write_byte(res, address);
        self.set_zero_neg_flags(res);
    }

    // increment X
    fn inx(&mut self) {
        if DEBUGLOG {
            println!("inx");
        }
        ( self.rx, _ ) = self.rx.overflowing_add(1);
        self.set_zero_neg_flags(self.rx);
    }

    // increment Y
    fn iny(&mut self) {
        if DEBUGLOG {
            println!("iny");
        }
        ( self.ry, _ ) = self.ry.overflowing_add(1);
        self.set_zero_neg_flags(self.ry);
    }

    /// ======================
    /// ARITHMETIC OPERATIONS
    /// ======================

    // add with carry
    // TODO: Some of the hardest functions
    fn adc(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("adc");
        }
        todo!("adc");
    }

    // subtract with carry
    // TODO: Some of the hardest functions
    fn sbc(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("sbc");
        }
        todo!("sbc");
    }

    /// ======================
    /// LOGICAL OPERATIONS
    /// ======================

    // and (with accumulator)
    fn and(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("and")
        }
        let address = self.get_address(addr);
        let and = self.read_byte(address);

        self.acc &= and;
        self.set_zero_neg_flags(self.acc);
    }

    // exclusive or (with accumulator)
    fn eor(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("eor")
        }
        let address = self.get_address(addr);
        let eor = self.read_byte(address);

        self.acc ^= eor;
        self.set_zero_neg_flags(self.acc);
    }

    // or with accumulator
    fn ora(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("ora");
        }
        let address = self.get_address(addr);
        let ora = self.read_byte(address);

        self.acc |= ora;
        self.set_zero_neg_flags(self.acc);
    }

    /// ======================
    /// SHIFT & ROTATE INSTRUCTIONS
    /// ======================

    // arithmetic shift left
    fn asl(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("asl");
        }
        let address = self.get_address(addr);
        let byte = self.read_byte(address);
        match addr {
            AddressMode::Accumulator => {
                self.acc <<= 1;
                self.set_zero_neg_flags(self.acc);
            }
            _ => {
                let res = byte << 1;
                self.write_byte(res, address);
                self.set_zero_neg_flags(res);
            }
        }
    }

    // logical shift right
    fn lsr(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("lsr");
        }
        let address = self.get_address(addr);
        let byte = self.read_byte(address);
        match addr {
            AddressMode::Accumulator => {
                self.acc >>= 1;
                self.set_zero_neg_flags(self.acc);
            }
            _ => {
                let res = byte >> 1;
                self.write_byte(res, address);
                self.set_zero_neg_flags(res);
            }
        }
    }

    // rotate left
    fn rol(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("rol");
        }
        let address = self.get_address(addr);
        let byte = self.read_byte(address);
        match addr {
            AddressMode::Accumulator => {
                self.acc = self.acc.rotate_left(1);
                self.set_zero_neg_flags(self.acc);
            }
            _ => {
                let res = byte.rotate_left(1);
                self.write_byte(res, address);
                self.set_zero_neg_flags(res);
            }
        }
    }

    // rotate right
    fn ror(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("ror");
        }
        let address = self.get_address(addr);
        let byte = self.read_byte(address);
        match addr {
            AddressMode::Accumulator => {
                self.acc = self.acc.rotate_right(1);
                self.set_zero_neg_flags(self.acc);
            }
            _ => {
                let res = byte.rotate_right(1);
                self.write_byte(res, address);
                self.set_zero_neg_flags(res);
            }
        }
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

    // set decimal flag
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
        let address = self.get_address(addr);
        let (res, _) = self.acc.overflowing_sub(self.read_byte(address));
        self.set_zero_neg_flags(res);
        if self.acc >= res {
            self.f |= C;
        }
    }

    // compare with X
    fn cpx(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("cpx");
        }
        let addresse = self.get_address(addr);
        let (res, _ ) = self.rx.overflowing_sub(self.read_byte(addresse));
        self.set_zero_neg_flags(res);
        if self.rx >= res {
            self.f |= C;
        }
    }

    // compare with Y
    fn cpy(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("cpy");
        }
        let addresse = self.get_address(addr);
        let (res, _ ) = self.ry.overflowing_sub(self.read_byte(addresse));
        self.set_zero_neg_flags(res);
        if self.ry >= res {
            self.f |= C;
        }
    }

    /// ======================
    /// CONDITIONAL BRANCH INSTRUCTION
    /// ======================

    // branch on carry clear
    fn bcc(&mut self) {
        if DEBUGLOG {
            println!("bcc");
        }
        todo!("bcc");
    }

    // branch on carry set
    fn bcs(&mut self) {
        if DEBUGLOG {
            println!("bcs");
        }
        todo!("bcs");
    }

    // branch on equal (zero set)
    fn beq(&mut self) {
        if DEBUGLOG {
            println!("beq");
        }
        todo!("beq");
    }

    // branch on minus (negative set)
    fn bmi(&mut self) {
        if DEBUGLOG {
            println!("bmi");
        }
        todo!("bmi");
    }

    // branch on not equal (zero clear)
    fn bne(&mut self) {
        if DEBUGLOG {
            println!("bne");
        }
        let offset = self.fetch_byte();
        let zf = self.f & Z;
        if zf != Z {
            self.pc += offset as u16;
        }
    }

    // branch on plus (negative clear)
    fn bpl(&mut self) {
        if DEBUGLOG {
            println!("bpl");
        }
        todo!("bpl");
    }

    // branch on overflow clear
    fn bvc(&mut self) {
        if DEBUGLOG {
            println!("bvc");
        }
        todo!("bvc");
    }

    // branch on overflow set
    fn bvs(&mut self) {
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
        self.push_stack(hh);
        self.push_stack(ll);

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
        let ll = self.pop_stack();
        let hh = self.pop_stack();
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
        let (ll, hh) = self.word_to_bytes(self.pc);
        self.push_stack(hh);
        self.push_stack(ll);
        self.f = B;
        self.push_stack(self.f);
        self.pc = self.read_word(0xFFFE);
    }

    // return from interrupt
    fn rti(&mut self) {
        if DEBUGLOG {
            println!("rti");
        }
        let ll = self.pop_stack();
        let hh = self.pop_stack();
        self.pop_stack();
        self.pc = self.bytes_to_word(ll, hh);
        self.f = self.pop_stack() ^ B;
    }

    /// ======================
    /// OTHER
    /// ======================

    // bit test
    fn bit(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("bit");
        }
        let address = self.get_address(addr);
        let byte = self.read_byte(address);
        let op = byte | (N & V);
        self.f = op & self.acc;
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
        // We could also do a wait here...
        // I will see in the future!
    }
}

fn main() {
    // println!("Hello, world!");

    let mut c = Chip::new();

    c.load_exe("bin/6502_functional_test.bin".to_string())
        .unwrap();
    c.startup(0x200);

    loop {
        c.execute_cycle();
    }
}


/// ==========================
/// TRANSFER INSTRUCTIONS TEST
/// ==========================

#[cfg(test)]
mod load_accumulator {
    use crate::*;

    #[test]
    fn immediate_addressing() {
        let mut c = Chip::new();

        // Code:
        // LDA #$1
        let prog: Vec<u8> = [0xA9, 0x01].to_vec();
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(0x01, c.acc);
    }

    #[test]
    fn zeropage_addressing() {
        let mut c = Chip::new();

        // Code:
        // LDA $01
        let prog: Vec<u8> = [0xA5, 0x01].to_vec();
        c.memory[0x01] = 0x12;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(0x12, c.acc);
    }

    #[test]
    fn zeropage_x_addressing() {
        let mut c = Chip::new();

        // Code:
        // LDA $01,X
        let prog: Vec<u8> = [0xB5, 0x01].to_vec();
        c.rx = 0x01;
        c.memory[0x02] = 0x12;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(0x12, c.acc);
    }
    
    #[test]
    fn absolute_addressing() {
        let mut c = Chip::new();

        // Code:
        // LDA $3010
        let prog: Vec<u8> = [0xAD, 0x10, 0x30].to_vec();
        c.memory[0x3010] = 0x12;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(0x12, c.acc);
    }

    #[test]
    fn absolute_x_addressing() {
        let mut c = Chip::new();

        // Code:
        // LDA $3120,X
        let prog: Vec<u8> = [0xBD, 0x20, 0x31].to_vec();
        c.rx = 0x12;
        c.memory[0x3132] = 0x12;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(0x12, c.acc);
    }

    #[test]
    fn absolute_y_addressing() {
        let mut c = Chip::new();

        // Code:
        // LDA $3120,Y
        let prog: Vec<u8> = [0xB9, 0x20, 0x31].to_vec();
        c.ry = 0x12;
        c.memory[0x3132] = 0x12;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(0x12, c.acc);
    }

    #[test]
    fn indirect_x_addressing() {
        let mut c = Chip::new();

        // Code:
        // LDA ($70,X)
        let prog: Vec<u8> = [0xA1, 0x70].to_vec();
        c.rx = 0x05;
        c.memory[0x75] = 0x32;
        c.memory[0x76] = 0x30;
        c.memory[0x3032] = 0xA5;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(0xA5, c.acc);
    }

    #[test]
    fn indirect_y_addressing() {
        let mut c = Chip::new();

        // Code:
        // LDA ($70),Y
        let prog: Vec<u8> = [0xB1, 0x70].to_vec();
        c.ry = 0x10;
        c.memory[0x70] = 0x43;
        c.memory[0x71] = 0x35;
        c.memory[0x3553] = 0x12;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(0x12, c.acc);
    }

    #[test]
    fn flags() {
        let mut c = Chip::new();

        // Code:
        // LDA #$01
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
    fn immediate_addressing() {
        let mut c = Chip::new();

        // Code:
        // LDX #$01
        let prog: Vec<u8> = [0xA2, 0x01].to_vec();
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(0x01, c.rx);
    }

    #[test]
    fn zeropage_addressing() {
        let mut c = Chip::new();

        // Code:
        // LDX #01
        let prog: Vec<u8> = [0xA6, 0x01].to_vec();
        c.memory[0x01] = 0x12;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(0x12, c.rx);
    }

    #[test]
    fn zeropage_y_addressing() {
        let mut c = Chip::new();

        // Code:
        // LDX $01,Y
        let prog: Vec<u8> = [0xB6, 0x01].to_vec();
        c.ry = 0x4;
        c.memory[0x05] = 0x12;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(0x12, c.rx);
    }

    #[test]
    fn absolute_addressing() {
        let mut c = Chip::new();

        // Code:
        // LDX $3120
        let prog: Vec<u8> = [0xAE, 0x20, 0x31].to_vec();
        c.memory[0x3120] = 0x12;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(0x12, c.rx);
    }

    #[test]
    fn absolute_y_addressing() {
        let mut c = Chip::new();

        // Code:
        // LDX $3120,Y
        let prog: Vec<u8> = [0xBE, 0x20, 0x31].to_vec();
        c.ry = 0x12;
        c.memory[0x3132] = 0x12;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(0x12, c.rx);
    }

    #[test]
    fn flags() {
        let mut c = Chip::new();

        // Code:
        // LDX #$01
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
    fn immediate_addressing() {
        let mut c = Chip::new();

        // Code:
        // LDY #$01
        let prog: Vec<u8> = [0xA0, 0x01].to_vec();
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(0x01, c.ry);
    }

    #[test]
    fn zeropage_addressing() {
        let mut c = Chip::new();

        // Code:
        // LDY $01
        let prog: Vec<u8> = [0xA4, 0x01].to_vec();
        c.memory[0x01] = 0x12;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(0x12, c.ry);
    }

    #[test]
    fn zeropage_x_addressing() {
        let mut c = Chip::new();

        // Code:
        // LDY $01,Y
        let prog: Vec<u8> = [0xB4, 0x01].to_vec();
        c.rx = 0x0A;
        c.memory[0x0B] = 0x12;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(0x12, c.ry);
    }

    #[test]
    fn absolute_addressing() {
        let mut c = Chip::new();

        // Code:
        // LDY $FFF0
        let prog: Vec<u8> = [0xAC, 0xF0, 0xFF].to_vec();
        c.memory[0xFFF0] = 0x12;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(0x12, c.ry);
    }

    #[test]
    fn absolute_x_addressing() {
        let mut c = Chip::new();

        // Code:
        // LDY $3120,X
        let prog: Vec<u8> = [0xBC, 0x20, 0x31].to_vec();
        c.rx = 0x12;
        c.memory[0x3132] = 0x12;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(0x12, c.ry);
    }

    #[test]
    fn flags() {
        let mut c = Chip::new();

        // Code:
        // LDY #$01
        let prog: Vec<u8> = [0xA0, 0x01].to_vec();
        c.load_program(prog);

        c.execute_cycle();
        assert_ne!(Z, c.f);
    }
}

#[cfg(test)]
mod store_accumulator {
    use crate::*;

    #[test]
    fn zeropage_addressing() {
        let mut c = Chip::new();

        // Code:
        // STA $01
        let prog: Vec<u8> = [0x85, 0x01].to_vec();
        c.acc = 0x01;
        c.load_program(prog);

        c.execute_cycle();
        c.execute_cycle();
        assert_eq!(c.memory[0x01], 0x01);
    }

    #[test]
    fn zeropage_x_addressing() {
        let mut c = Chip::new();

        // Code:
        // STA $01,X
        let prog: Vec<u8> = [0x95, 0x01].to_vec();
        c.acc = 0x01;
        c.rx = 0x01;
        c.load_program(prog);

        c.execute_cycle();
        c.execute_cycle();
        assert_eq!(c.memory[0x02], 0x01);
    }

    #[test]
    fn absolute_addressing() {
        let mut c = Chip::new();

        // Code:
        // STA $FFF0
        let prog: Vec<u8> = [0x8D, 0xF0, 0xFF].to_vec();
        c.acc = 0x01;
        c.load_program(prog);

        c.execute_cycle();
        c.execute_cycle();
        assert_eq!(c.memory[0xFFF0], 0x01);
    }

    #[test]
    fn absolute_x_addressing() {
        let mut c = Chip::new();

        // Code:
        // STA $3120,X
        let prog: Vec<u8> = [0x9D, 0x20, 0x31].to_vec();
        c.acc = 0x01;
        c.rx = 0x12;
        c.load_program(prog);

        c.execute_cycle();
        c.execute_cycle();
        assert_eq!(c.memory[0x3132], 0x01);
    }

    #[test]
    fn absolute_y_addressing() {
        let mut c = Chip::new();

        // Code:
        // STA $3120,Y
        let prog: Vec<u8> = [0x99, 0x20, 0x31].to_vec();
        c.acc = 0x01;
        c.ry = 0x12;
        c.load_program(prog);

        c.execute_cycle();
        c.execute_cycle();
        assert_eq!(c.memory[0x3132], 0x01);
    }

    #[test]
    fn indirect_x_addressing() {
        let mut c = Chip::new();

        // Code:
        // STA ($70,X)
        let prog: Vec<u8> = [ 0x81, 0x70].to_vec();
        c.acc = 0x01;
        c.rx = 0x05;
        c.memory[0x75] = 0x32;
        c.memory[0x76] = 0x30;
        c.load_program(prog);

        c.execute_cycle();
        c.execute_cycle();
        assert_eq!(c.memory[0x3032], 0x01);
    }

    #[test]
    fn indirect_y_addressing() {
        let mut c = Chip::new();

        // Code:
        // STA ($70),Y
        let prog: Vec<u8> = [0x91, 0x70].to_vec();
        c.acc = 0x01;
        c.ry = 0x10;
        c.memory[0x70] = 0x43;
        c.memory[0x71] = 0x35;
        c.load_program(prog);

        c.execute_cycle();
        c.execute_cycle();
        assert_eq!(c.memory[0x3553], 0x01);
    }
}

#[cfg(test)]
mod store_x {
    use crate::*;

    #[test]
    fn zeropage_addressing() {
        let mut c = Chip::new();

        // Code:
        // STX $01
        let prog: Vec<u8> = [0x86, 0x01].to_vec();
        c.rx = 0x01;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(c.memory[0x01], 0x01);
    }

    #[test]
    fn zeropage_y_addressing() {
        let mut c = Chip::new();

        // Code:
        // STX $01,Y
        let prog: Vec<u8> = [0x96, 0x01].to_vec();
        c.rx = 0x01;
        c.ry = 0x01;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(c.memory[0x02], 0x01);
    }

    #[test]
    fn absolute_addressing() {
        let mut c = Chip::new();

        // Code:
        // STX $FFF0
        let prog: Vec<u8> = [0x8E, 0xF0, 0xFF].to_vec();
        c.rx = 0x01;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(c.memory[0xFFF0], 0x01);
    }
}

#[cfg(test)]
mod store_y {
    use crate::*;

    #[test]
    fn zeropage_addressing() {
        let mut c = Chip::new();

        // Code:
        // STA $01
        let prog: Vec<u8> = [0x84, 0x01].to_vec();
        c.ry = 0x01;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(c.memory[0x01], 0x01);
    }

    #[test]
    fn zeropage_x_addressing() {
        let mut c = Chip::new();

        // Code:
        // STA $01,X
        let prog: Vec<u8> = [0x94, 0x01].to_vec();
        c.rx = 0x01;
        c.ry = 0x01;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(c.memory[0x02], 0x01);
    }

    #[test]
    fn absolute_addressing() {
        let mut c = Chip::new();

        // Code:
        // STA $FFF0
        let prog: Vec<u8> = [0x8C, 0xF0, 0xFF].to_vec();
        c.ry = 0x01;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(c.memory[0xFFF0], 0x01);
    }
}

#[cfg(test)]
mod transfer_accumulator_to_x {
    use crate::*;

    #[test]
    fn implied_addressing() {
        let mut c = Chip::new();

        // Code:
        // TAX
        let prog: Vec<u8> = [0xAA].to_vec();
        c.acc = 0x01;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(c.rx, 0x01);
    }
}

#[cfg(test)]
mod transfer_accumulator_to_y {
    use crate::*;

    #[test]
    fn implied_addressing() {
        let mut c = Chip::new();

        // Code:
        // TAY
        let prog: Vec<u8> = [0xA8].to_vec();
        c.acc = 0x01;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(c.ry, 0x01);
    }
}

#[cfg(test)]
mod transfer_stack_pointer_to_x {
    use crate::*;

    #[test]
    fn implied_addressing() {
        let mut c = Chip::new();

        // Code:
        // TSX
        let prog: Vec<u8> = [0xBA].to_vec();
        c.sp = 0x01;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(c.rx, 0x01);
    }
}

#[cfg(test)]
mod transfer_x_to_accumulator {
    use crate::*;

    #[test]
    fn implied_addressing() {
        let mut c = Chip::new();

        // Code:
        // TXA
        let prog: Vec<u8> = [0x8A].to_vec();
        c.rx = 0x01;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(c.acc, 0x01);
    }
}

#[cfg(test)]
mod transfer_x_to_stack_pointer {
    use crate::*;

    #[test]
    fn implied_addressing() {
        let mut c = Chip::new();

        // Code:
        // TXS
        let prog: Vec<u8> = [0x9A].to_vec();
        c.rx = 0x01;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(c.sp, 0x01);
    }
}

#[cfg(test)]
mod transfer_y_to_accumulator {
    use crate::*;

    #[test]
    fn implied_addressing() {
        let mut c = Chip::new();

        // Code:
        // TYA
        let prog: Vec<u8> = [0x98].to_vec();
        c.ry = 0x01;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(c.acc, 0x01);
    }
}

/// ==============================
/// STACK INSTRUCTIONS TEST
/// ==============================

#[cfg(test)]
mod push_accumulator {

}

#[cfg(test)]
mod push_processor_status_register {
    
}

#[cfg(test)]
mod pull_accumulator {
    
}

#[cfg(test)]
mod pull_processor_status_register {
    
}

/// ==============================
/// DECREMENNT & INCREMENT TESTS
/// ==============================

#[cfg(test)]
mod decrement {
    use crate::*;

    #[test]
    fn zeropage_addressing() {
        let mut c = Chip::new();

        // Code:
        // DEC $01
        let prog: Vec<u8> = [0xC6, 0x01].to_vec();
        c.memory[0x01] = 0x12;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(c.memory[0x01], 0x11);
    }

    #[test]
    fn zeropage_x_addressing() {
        let mut c = Chip::new();

        // Code:
        // DEC $01,X
        let prog: Vec<u8> = [0xD6, 0x01].to_vec();
        c.rx = 0x01;
        c.memory[0x02] = 0x13;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(c.memory[0x02], 0x12);
    }

    #[test]
    fn absolute_addressing() {
        let mut c = Chip::new();

        // Code:
        // DEC $FFF0
        let prog: Vec<u8> = [0xCE, 0xF0, 0xFF].to_vec();
        c.memory[0xFFF0] = 0xAA;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(c.memory[0xFFF0], 0xA9);
    }

    #[test]
    fn absolute_x_addressing() {
        let mut c = Chip::new();

        // Code:
        // DEC $3120,X
        let prog: Vec<u8> = [0xDE, 0x20, 0x31].to_vec();
        c.rx = 0x12;
        c.memory[0x3132] = 0xAA;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(c.memory[0x3132], 0xA9);
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
    fn implied_addressing() {
        let mut c = Chip::new();

        // Code:
        // DEX
        let prog: Vec<u8> = [0xCA].to_vec();
        c.rx = 0x01;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(c.rx, 0x00);
    }

    #[test]
    fn flag() {
        let mut c = Chip::new();

        // Code:
        // DEX
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
    fn implied_addressing() {
        let mut c = Chip::new();

        // Code:
        // DEY
        let prog: Vec<u8> = [0x88].to_vec();
        c.ry = 0x01;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(c.rx, 0x00);
    }

    #[test]
    fn flag() {
        let mut c = Chip::new();

        // Code:
        // DEY
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
    fn zeropage_addressing() {
        let mut c = Chip::new();

        // Code:
        // INC $01
        let prog: Vec<u8> = [0xE6, 0x01].to_vec();
        c.memory[0x01] = 0x12;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(c.memory[0x01], 0x13);
    }

    #[test]
    fn zeropage_x_addressing() {
        let mut c = Chip::new();

        // Code:
        // INC $01,X
        let prog: Vec<u8> = [0xF6, 0x01].to_vec();
        c.rx = 0x01;
        c.memory[0x02] = 0x14;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(c.memory[0x02], 0x15);
    }

    #[test]
    fn absolute_addressing() {
        let mut c = Chip::new();

        // Code:
        // INC $3010
        let prog: Vec<u8> = [0xEE, 0x10, 0x30].to_vec();
        c.memory[0x3010] = 0xAA;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(c.memory[0x3010], 0xAB);
    }

    #[test]
    fn absolute_x_addressing() {
        let mut c = Chip::new();

        // Code:
        // INC $3120,X
        let prog: Vec<u8> = [0xFE, 0x20, 0x31].to_vec();
        c.rx = 0x12;
        c.memory[0x3132] = 0xAA;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(c.memory[0x3132], 0xAB);
    }

    #[test]
    fn flag() {
        let mut c = Chip::new();

        // Code:
        // INC $01
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
    fn implied_addressing() {
        let mut c = Chip::new();

        // Code:
        // INX
        let prog: Vec<u8> = [0xE8].to_vec();
        c.rx = 0x11;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(c.rx, 0x12);
    }

    #[test]
    fn flag() {
        let mut c = Chip::new();

        // Code:
        // INX
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
    fn implied_addressing() {
        let mut c = Chip::new();

        // Code:
        // INY
        let prog: Vec<u8> = [0xC8].to_vec();
        c.ry = 0x02;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(c.ry, 0x03);
    }

    #[test]
    fn flag() {
        let mut c = Chip::new();

        // Code:
        // INY
        let prog: Vec<u8> = [0xC8].to_vec();
        c.ry = 0x01;
        c.load_program(prog);

        c.execute_cycle();
        assert_ne!(Z, c.f);
    }
}

/// ==========================
/// ARITHMETIC OPERATION TESTS
/// ==========================

#[cfg(test)]
mod add_with_carry {
    
}

#[cfg(test)]
mod subtract_with_carry {
    
}

/// ==========================
/// LOGICAL TESTS
/// ==========================

#[cfg(test)]
mod and_with_accumulator {
    
}
#[cfg(test)]
mod exclusive_or_with_accumulator {
    
}

#[cfg(test)]
mod or_with_accumulator {
    
}

/// ==========================
/// SHIFT & ROTATE TESTS
/// ==========================

#[cfg(test)]
mod arithmtic_shift_left {

}

#[cfg(test)]
mod logical_shift_right {
    
}

#[cfg(test)]
mod rotate_left {
    
}

#[cfg(test)]
mod rotate_right {
    
}

/// ==========================
/// FLAG TESTS
/// ==========================

#[cfg(test)]
mod clear_carry {

}

#[cfg(test)]
mod clear_decimal {
    
}

#[cfg(test)]
mod clear_interrupt_disable {
    
}

#[cfg(test)]
mod clear_overflow {
    
}

#[cfg(test)]
mod set_carry {
    
}

#[cfg(test)]
mod set_decimal {
    
}

#[cfg(test)]
mod set_interrupt_diable {
    
}

/// ==========================
/// COMPARISON TESTS
/// ==========================

#[cfg(test)]
mod compare_with_accumulator {
    use crate::*;

    #[test]
    fn immediate_addressing() {
        let mut c = Chip::new();

        // Code:
        // CMP
        let prog: Vec<u8> = [0xC9, 0x01].to_vec();
        c.acc = 0x01;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(c.f, C | Z);
    }

    #[test]
    fn zeropage_addressing() {
        let mut c = Chip::new();

        // Code:
        // CMP $80
        let prog: Vec<u8> = [0xC5, 0x80].to_vec();
        c.acc = 0x01;
        c.memory[0x80] = 0x01;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(c.f, C | Z);
    }

    #[test]
    fn zeropage_x_addressing() {
        let mut c = Chip::new();

        // Code:
        // CMP $80,X
        let prog: Vec<u8> = [0xD5, 0x80].to_vec();
        c.acc = 0x01;
        c.rx = 0x02;
        c.memory[0x82] = 0x01;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(c.f, C | Z);
    }

    #[test]
    fn absolute_addressing() {
        let mut c = Chip::new();

        // Code:
        // CMP $3010
        let prog: Vec<u8> = [0xCD].to_vec();
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(c.f, C | Z);
    }

    #[test]
    fn absolute_x_addressing() {
        let mut c = Chip::new();

        // Code:
        // CMP $3120,X
        let prog: Vec<u8> = [0xDD, 0x20, 0x31].to_vec();
        c.acc = 0x01;
        c.rx = 0x12;
        c.memory[0x3132] = 0x1;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(c.f, C | Z);
    }

    #[test]
    fn absolute_y_addressing() {
        let mut c = Chip::new();

        // Code:
        // CMP $3120,Y
        let prog: Vec<u8> = [0xD9, 0x20, 0x31].to_vec();
        c.acc = 0x01;
        c.ry = 0x12;
        c.memory[0x3132] = 0x1;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(c.f, C | Z);
    }

    #[test]
    fn indirect_x_addressing() {
        let mut c = Chip::new();

        // Code:
        // CMP ($70,X)
        let prog: Vec<u8> = [0xC1, 0x70].to_vec();
        c.acc = 0x01;
        c.rx = 0x05;
        c.memory[0x75] = 0x32;
        c.memory[0x76] = 0x30;
        c.memory[0x3032] = 0x01;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(c.f, C | Z);
    }

    #[test]
    fn indirect_y_addressing() {
        let mut c = Chip::new();

        // Code:
        // CMP ($70),Y
        let prog: Vec<u8> = [0xD1, 0x70].to_vec();
        c.acc = 0x01;
        c.ry = 0x10;
        c.memory[0x70] = 0x43;
        c.memory[0x71] = 0x35;
        c.memory[0x3553] = 0x01;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(c.f, C | Z);
    }
}

#[cfg(test)]
mod compare_with_x {
    use crate::*;

    #[test]
    fn immediate_addressing() {
        let mut c = Chip::new();

        // Code:
        // CPX #$01
        let prog: Vec<u8> = [0xE0, 0x01].to_vec();
        c.rx = 0x01;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(c.f, C | Z);
    }

    #[test]
    fn zeropage_addressing() {
        let mut c = Chip::new();

        // Code:
        // CPX $80
        let prog: Vec<u8> = [0xE4, 0x80].to_vec();
        c.memory[0x80] = 0x01;
        c.rx = 0x01;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(c.f, C | Z);
    }

    #[test]
    fn absolute_addressing() {
        let mut c = Chip::new();

        // Code:
        // CPX $3010
        let prog: Vec<u8> = [0xEC, 0x10, 0x30].to_vec();
        c.memory[0x3010] = 0x01;
        c.rx = 0x01;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(c.f, C | Z);
    }
}

#[cfg(test)]
mod compare_with_y {
    use crate::*;

    #[test]
    fn immediate_addressing() {
        let mut c = Chip::new();

        // Code:
        // CPY #$01
        let prog: Vec<u8> = [0xC0, 0x01].to_vec();
        c.ry = 0x01;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(c.f, C | Z);
    }

    #[test]
    fn zeropage_addressing() {
        let mut c = Chip::new();

        // Code:
        // CPY $80
        let prog: Vec<u8> = [0xC4, 0x80].to_vec();
        c.memory[0x80] = 0x01;
        c.ry = 0x01;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(c.f, C | Z);
    }

    #[test]
    fn absolute_addressing() {
        let mut c = Chip::new();

        // Code:
        // CPY $3010
        let prog: Vec<u8> = [0xCC, 0x10, 0x30].to_vec();
        c.memory[0x3010] = 0x01;
        c.ry = 0x01;
        c.load_program(prog);

        c.execute_cycle();
        assert_eq!(c.f, C | Z);
    }
}

/// ==========================
/// CONDITIONAL BRANCH TESTS
/// ==========================

#[cfg(test)]
mod branch_on_carry_clear {
    
}

#[cfg(test)]
mod branch_on_carry_set {
    
}

#[cfg(test)]
mod branch_on_equal {
    
}

#[cfg(test)]
mod branch_on_minus {
    
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
    
}

#[cfg(test)]
mod branch_on_overflow_clear {
    
}

#[cfg(test)]
mod branch_on_overflow_carry {
    
}

/// ==========================
/// JUMP & SUBROUTINE TESTS
/// ==========================

#[cfg(test)]
mod jump {
    use crate::*;

    #[test]
    fn absolute_addressing() {
        let mut c = Chip::new();

        // Code:
        // JMP $3010
        // LDA #$FF
        let prog: Vec<u8> = [0x4C, 0x10, 0x30].to_vec();
        c.load_program(prog);
        c.memory[0x3010] = 0xA9;
        c.memory[0x3011] = 0xFF;

        c.execute_cycle();
        c.execute_cycle();
        assert_eq!(0xFF, c.acc);
    }

    #[test]
    fn indirect_addressing() {
        let mut c = Chip::new();

        // Code:
        // JMP ($4FF82)
        // LDA #$FF
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
}

#[cfg(test)]
mod jump_soubroutine {
    use crate::*;

    #[test]
    fn absolute_addressing() {
        let mut c = Chip::new();

        // Code:
        // JSR $4240
        // LDA #$FF
        let prog: Vec<u8> = [0x20, 0x40, 0x42].to_vec();
        c.load_program(prog);
        c.memory[0x4240] = 0xA9;
        c.memory[0x4241] = 0xFF;

        c.execute_cycle();
        c.execute_cycle();
        assert_eq!(0xFF, c.acc);
        assert_eq!(c.memory[0x0100], 0x02);
        assert_eq!(c.memory[0x0101], 0x03);
    }

}

#[cfg(test)]
mod return_from_subroutine {
    use crate::*;

    #[test]
    fn implied_addressing() {
        let mut c = Chip::new();

        // Code:
        // JSR $4240
        // LDA #$FF
        // RTS
        // LDA #$F0
        let prog: Vec<u8> = [0x20, 0x40, 0x42, 0xA9, 0xF0].to_vec();
        c.load_program(prog);
        c.memory[0x4240] = 0xA9;
        c.memory[0x4241] = 0xFF;
        c.memory[0x4242] = 0x60;

        c.execute_cycle();
        c.execute_cycle();
        c.execute_cycle();
        assert_eq!(0xFF, c.acc);
        c.execute_cycle();
        assert_eq!(0xF0, c.acc);
    }
}

/// ==========================
/// INTERRUPT TESTS
/// ==========================

#[cfg(test)]
mod break_software_interrupt {

}

#[cfg(test)]
mod return_from_interrupt {

}

/// ==========================
/// OTHER TESTS
/// ==========================

// TODO: UPDATE OTHER TESTS TO INCLUDE ONLY ONE OPCODE INSTRUCTION
#[cfg(test)]
mod other {
    use crate::*;

    #[test]
    fn no_operation() {
        let mut c = Chip::new();

        // Code:
        // NOP
        // NOP
        // NOP
        // NOP
        // NOP
        let prog: Vec<u8> = [0xEA, 0xEA, 0xEA, 0xEA, 0xEA].to_vec();
        c.load_program(prog);

        c.execute_cycle();
        c.execute_cycle();
        c.execute_cycle();
        c.execute_cycle();
        c.execute_cycle();
        assert_eq!(0x205, c.pc);
    }
}
