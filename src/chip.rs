// Imports for reading a file
use std::fs::File;
use std::io;
use std::io::prelude::*;

// max number of an u16
pub const MEMORY: usize = 65536;

// The flag's for the flag in the Chip
// defined as u8 so to or them
pub const N: u8 = 0x80; // [1000 0000] negative
pub const V: u8 = 0x40; // [0100 0000] overflow
pub const R: u8 = 0x20; // [0010 0000] reserved
pub const B: u8 = 0x10; // [0001 0000] break
pub const D: u8 = 0x08; // [0000 1000] decimale
pub const I: u8 = 0x04; // [0000 0100] interupt disable
pub const Z: u8 = 0x02; // [0000 0010] zero
pub const C: u8 = 0x01; // [0000 0001] carry

#[derive(Debug, PartialEq, Copy, Clone)]
enum AddressMode {
    Accumulator,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Immediate,
    Indirect,
    XIndirect,
    IndirectY,
    Zeropage,
    ZeropageX,
    ZeropageY,
}

pub struct Chip {
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
    pub memory: [u8; MEMORY],
}

impl Chip {
    pub fn new() -> Chip {
        Chip {
            acc: 0,
            rx: 0,
            ry: 0,
            f: 0,
            sp: 0xFF,
            pc: 0x200,
            memory: [0; MEMORY],
        }
    }

    // ======================================
    // Starting and executing functions
    // ======================================

    pub fn startup(&mut self, address: u16) {
        self.pc = address;
    }

    pub fn load_program(&mut self, prog: Vec<u8>) {
        for i in 0..prog.len() {
            self.memory[0x200 + i] = prog[i];
        }
    }

    pub fn load_exe(&mut self, file_path: String, zeropage_start: usize) -> io::Result<()> {
        let mut f = File::open(file_path)?;
        let mut buffer = Vec::new();

        f.read_to_end(&mut buffer)?;
        // println!("buffer.len() = {}", buffer.len());
        // println!("{:?}", buffer);
        // println!("self.memory.len() = {}", self.memory.len());
        for i in zeropage_start..MEMORY {
            // println!("{:>08b}", self.memory[i]);
            self.memory[i] = buffer[i - zeropage_start];
        }
        Ok(())
    }

    // =====================
    // Helper functions
    // =====================

    /// pushes a byte to the stack
    /// and then decrements the stack pointer
    fn push_stack(&mut self, byte: u8) {
        self.memory[0x0100 + self.sp as usize] = byte;
        (self.sp, _) = self.sp.overflowing_sub(1);
    }

    /// Pops a value from the stack
    /// and then increments the stack pointer
    fn pop_stack(&mut self) -> u8 {
        (self.sp, _) = self.sp.overflowing_add(1);
        let data = self.memory[0x0100 + self.sp as usize];
        data
    }

    /// Reads a byte from the given address
    fn read_byte(&mut self, address: u16) -> u8 {
        self.memory[(address) as usize]
    }

    /// Fetches a byte from the programm counter
    fn fetch_byte(&mut self) -> u8 {
        let data = self.memory[(self.pc) as usize];
        self.pc += 1;
        data
    }

    /// Writes the given byte to the address
    fn write_byte(&mut self, byte: u8, address: u16) {
        self.memory[address as usize] = byte;
    }

    /// Reads a word from an address
    fn read_word(&mut self, address: u16) -> u16 {
        let b1 = self.read_byte(address);
        let b2 = self.read_byte(address + 1);
        self.bytes_to_word(b1, b2)
    }

    /// Fetches a word from the programm counter
    fn fetch_word(&mut self) -> u16 {
        let ll = self.fetch_byte();
        let hh = self.fetch_byte();
        self.bytes_to_word(ll, hh)
    }

    /// takes a u16 (word) number and returns
    /// two u8 (byte) numbers back
    fn word_to_bytes(&self, word: u16) -> (u8, u8) {
        (word as u8 & 0xFF, (word >> 8) as u8)
    }

    /// takes two u8 (byte) numbers and returns a
    /// u16 (word) number back
    fn bytes_to_word(&self, ll: u8, hh: u8) -> u16 {
        (ll as u16) + ((hh as u16) << 8)
    }

    /// Sets the zero and the negative flag:
    ///
    /// After most instructions that have a value result, this flag will either be set or cleared based on whether or not that value is equal to zero.
    ///
    /// Therefore for the Zero set:
    ///
    /// If the given value is 0, the zero flag is set.
    /// If the given value is not 0, the zero flag is cleared.
    ///
    /// The same aplies to the negative flag,
    /// that is only set if the seventh byte (starting at hexadecimal: `F0` or binary: `10000000`)
    /// and cleared if it is not above of that value.
    fn set_zero_neg_flags(&mut self, value: u8) {
        if value == 0x0 {
            self.set_flag(Z);
        } else {
            self.clear_flag(Z);
        }
        if value >= 0x80 {
            self.set_flag(N)
        } else {
            self.clear_flag(N);
        }
    }

    /// Function to set a flag
    fn set_flag(&mut self, flag: u8) {
        self.f |= flag;
    }

    /// Function to clear a flag
    fn clear_flag(&mut self, flag: u8) {
        if self.f & flag == flag {
            self.f ^= flag;
        }
    }

    /// The branching function ...
    /// Implemented here to not have repeating code ...
    fn branch(&mut self, offset: u8) {
        if offset < 0x80 {
            self.pc += offset as u16;
        } else {
            if offset > 0x80 {
                // This code here gets the positive form a into negative
                // parsed i8 as an u8
                self.pc -= (offset as i8 * -1i8) as u16;
            } else if offset == 0x80 {
                self.pc -= 128;
            }
        }
    }

    // Returns the address depending on the given AddressMode.
    fn get_address(&mut self, addr: AddressMode) -> u16 {
        match addr {
            AddressMode::Immediate => {
                // Need to increment the pc
                // Else it would not register that we have
                // "read" the Immediate Address
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
                let x = self.rx;
                let (address, _) = ll.overflowing_add(x);
                return address as u16;
            }
            AddressMode::ZeropageY => {
                let ll = self.fetch_byte();
                let y = self.ry;
                let (address, _) = ll.overflowing_add(y);
                return address as u16;
            }
            AddressMode::Indirect => {
                let address = self.fetch_word();
                let address2 = self.read_word(address);
                return address2;
            }
            AddressMode::XIndirect => {
                let ll = self.fetch_byte();
                let x = self.rx;
                let (address, _) = ll.overflowing_add(x);
                return self.read_word(address as u16);
            }
            AddressMode::IndirectY => {
                let ll = self.fetch_byte();
                let y = self.ry;
                let address = ll as u16;
                let address2 = self.read_word(address) + y as u16;
                return address2;
            }
            _ => {
                // This here needs to return nothing ...
                // because the ...
                // AddressMode::Accumulator is not handled ...
                // here and can also not be handled ...
                // because it is kind of special ...
                return 0;
            }
        }
    }

    /// This here is called execute_cycle but strictly speaking
    /// it only executes an OPCODE
    pub fn execute_cycle(&mut self) {
        let opcode: u8 = self.fetch_byte();
        self.process_opcode(opcode);
    }

    /// Processes an opcode and calls the correct function for the opcode
    fn process_opcode(&mut self, opcode: u8) {
        // opcode
        let op_1 = (opcode & 0xF0) >> 4;
        let op_2 = opcode & 0x0F;

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
            (0x1, 0x8) => self.clc(),
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
            (0x3, 0x8) => self.sec(),
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
            (0x5, 0x8) => self.cli(),
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
            (0x7, 0x8) => self.sei(),
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
            (0xB, 0x8) => self.clv(),
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
            (0xD, 0x8) => self.cld(),
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
            (0xF, 0x8) => self.sed(),
            (0xF, 0x9) => self.sbc(AddressMode::AbsoluteY),
            (0xF, 0xD) => self.sbc(AddressMode::AbsoluteX),
            (0xF, 0x6) => self.inc(AddressMode::ZeropageX),
            (0xF, 0xE) => self.inc(AddressMode::AbsoluteX),
            _ => {}
        }
    }

    // ======================
    // TRANSFER INSTRUCTIONS
    // ======================

    // load accumulator
    fn lda(&mut self, addr: AddressMode) {
        let address = self.get_address(addr);
        self.acc = self.read_byte(address);
        self.set_zero_neg_flags(self.acc);
    }

    // load X
    fn ldx(&mut self, addr: AddressMode) {
        let address = self.get_address(addr);
        self.rx = self.read_byte(address);
        self.set_zero_neg_flags(self.rx);
    }

    // load Y
    fn ldy(&mut self, addr: AddressMode) {
        let address = self.get_address(addr);
        self.ry = self.read_byte(address);
        self.set_zero_neg_flags(self.ry);
    }

    // store accumulator
    fn sta(&mut self, addr: AddressMode) {
        let address = self.get_address(addr);
        self.write_byte(self.acc, address);
    }

    // store X
    fn stx(&mut self, addr: AddressMode) {
        let address = self.get_address(addr);
        self.write_byte(self.rx, address);
    }

    // store Y
    fn sty(&mut self, addr: AddressMode) {
        let address = self.get_address(addr);
        self.write_byte(self.ry, address);
    }

    // transfer accumulator to X
    fn tax(&mut self) {
        self.rx = self.acc;
        self.set_zero_neg_flags(self.rx);
    }

    // transfer accumulator to Y
    fn tay(&mut self) {
        self.ry = self.acc;
        self.set_zero_neg_flags(self.ry);
    }

    // transfer stack pointer to X
    fn tsx(&mut self) {
        self.rx = self.sp;
        self.set_zero_neg_flags(self.rx);
    }

    // transfer X to accumulator
    fn txa(&mut self) {
        self.acc = self.rx;
        self.set_zero_neg_flags(self.acc);
    }

    // transfer X to stack pointer
    fn txs(&mut self) {
        self.sp = self.rx;
    }

    // transfer Y to accumulator
    fn tya(&mut self) {
        self.acc = self.ry;
        self.set_zero_neg_flags(self.acc);
    }

    // ======================
    // STACK INSTRUCTIONS
    // ======================

    // push accumulator
    fn pha(&mut self) {
        self.push_stack(self.acc);
    }

    // push processor status (SR)
    fn php(&mut self) {
        // Reference: https://www.nesdev.org/wiki/Status_flags#The_B_flag
        self.push_stack(self.f | B | 0x20);
    }

    // pull accumulator
    fn pla(&mut self) {
        self.acc = self.pop_stack();
        self.set_zero_neg_flags(self.acc);
    }

    // pull processor status (SR)
    fn plp(&mut self) {
        self.f = self.pop_stack();
    }

    // ======================
    // DECREMENTS & INCREMENTS
    // ======================

    // decrement
    fn dec(&mut self, addr: AddressMode) {
        let address = self.get_address(addr);
        let byte = self.read_byte(address);
        let (res, _) = byte.overflowing_sub(1);
        self.write_byte(res, address);
        self.set_zero_neg_flags(res);
    }

    // decrement X
    fn dex(&mut self) {
        (self.rx, _) = self.rx.overflowing_sub(1);
        self.set_zero_neg_flags(self.rx);
    }

    // decrement Y
    fn dey(&mut self) {
        (self.ry, _) = self.ry.overflowing_sub(1);
        self.set_zero_neg_flags(self.ry);
    }

    // increment
    fn inc(&mut self, addr: AddressMode) {
        let address = self.get_address(addr);
        let byte = self.read_byte(address);
        let (res, _) = byte.overflowing_add(1);
        self.write_byte(res, address);
        self.set_zero_neg_flags(res);
    }

    // increment X
    fn inx(&mut self) {
        (self.rx, _) = self.rx.overflowing_add(1);
        self.set_zero_neg_flags(self.rx);
    }

    // increment Y
    fn iny(&mut self) {
        (self.ry, _) = self.ry.overflowing_add(1);
        self.set_zero_neg_flags(self.ry);
    }

    // ======================
    // ARITHMETIC OPERATIONS
    // ======================

    // add with carry
    fn adc(&mut self, addr: AddressMode) {
        let address = self.get_address(addr);
        let byte = self.read_byte(address);
        let carry = if self.f & C == C { 1 } else { 0 };
        let m_7 = if self.acc & 0x80 == 0x80 { 1 } else { 0 };
        let n_7 = if byte & 0x80 == 0x80 { 1 } else { 0 };
        let (c, _) = (self.acc & 0x7F).overflowing_add((byte & 0x7F) + carry);
        let c_6 = if c & 0x80 == 0x80 { 1 } else { 0 };
        let temp_res = (self.acc as u16) + (byte as u16) + (carry as u16);
        self.acc = (temp_res & 0xFF) as u8;
        let of = temp_res & 0x100 == 0x100;
        if m_7 == 0 && n_7 == 0 && c_6 == 1 || m_7 == 1 && n_7 == 1 && c_6 == 0 {
            self.set_flag(V)
        } else {
            self.clear_flag(V);
        }
        if of == true {
            self.set_flag(C)
        } else {
            self.clear_flag(C)
        }
        self.set_zero_neg_flags(self.acc);
    }

    // subtract with carry
    fn sbc(&mut self, addr: AddressMode) {
        let address = self.get_address(addr);
        let byte = self.read_byte(address);
        let carry = if self.f & C == C { 1 } else { 0 };
        let m_7 = if self.acc & 0x80 == 0x80 { 1 } else { 0 };
        let n_7 = if byte & 0x80 == 0x80 { 1 } else { 0 };
        let (c, _) = (self.acc & 0x7F).overflowing_add(((255 - byte) & 0x7F) + carry);
        let c_6 = if c & 0x80 == 0x80 { 1 } else { 0 };
        let temp_res = (self.acc as u16) + (!byte as u16) + (carry as u16);
        self.acc = (temp_res & 0xFF) as u8;
        let of = temp_res & 0x100 == 0x100;
        if m_7 == 0 && n_7 == 1 && c_6 == 1 || m_7 == 1 && n_7 == 0 && c_6 == 0 {
            self.set_flag(V)
        } else {
            self.clear_flag(V);
        }
        if of == true {
            self.set_flag(C)
        } else {
            self.clear_flag(C)
        }
        self.set_zero_neg_flags(self.acc);
    }

    // ======================
    // LOGICAL OPERATIONS
    // ======================

    // and (with accumulator)
    fn and(&mut self, addr: AddressMode) {
        let address = self.get_address(addr);
        let and = self.read_byte(address);
        self.acc &= and;
        self.set_zero_neg_flags(self.acc);
    }

    // exclusive or (with accumulator)
    fn eor(&mut self, addr: AddressMode) {
        let address = self.get_address(addr);
        let eor = self.read_byte(address);
        self.acc ^= eor;
        self.set_zero_neg_flags(self.acc);
    }

    // or with accumulator
    fn ora(&mut self, addr: AddressMode) {
        let address = self.get_address(addr);
        let or = self.read_byte(address);
        self.acc |= or;
        self.set_zero_neg_flags(self.acc);
    }

    // ======================
    // SHIFT & ROTATE INSTRUCTIONS
    // ======================

    // arithmetic shift left
    fn asl(&mut self, addr: AddressMode) {
        let address = self.get_address(addr);
        let byte = self.read_byte(address);
        match addr {
            AddressMode::Accumulator => {
                if self.acc >> 7 == 1 {
                    self.set_flag(C);
                } else {
                    self.clear_flag(C);
                }
                self.acc <<= 1;
                self.set_zero_neg_flags(self.acc);
            }
            _ => {
                if byte >> 7 == 1 {
                    self.set_flag(C);
                } else {
                    self.clear_flag(C);
                }
                let res = byte << 1;
                self.write_byte(res, address);
                self.set_zero_neg_flags(res);
            }
        }
    }

    // logical shift right
    fn lsr(&mut self, addr: AddressMode) {
        let address = self.get_address(addr);
        let byte = self.read_byte(address);
        match addr {
            AddressMode::Accumulator => {
                if self.acc & 0x01 == 1 {
                    self.set_flag(C);
                } else {
                    self.clear_flag(C);
                }
                self.acc >>= 1;
                self.set_zero_neg_flags(self.acc);
            }
            _ => {
                if self.acc & 0x01 == 1 {
                    self.set_flag(C);
                } else {
                    self.clear_flag(C);
                }
                let res = byte >> 1;
                self.write_byte(res, address);
                self.set_zero_neg_flags(res);
            }
        }
    }

    // rotate left
    fn rol(&mut self, addr: AddressMode) {
        let address = self.get_address(addr);
        let byte = self.read_byte(address);
        match addr {
            AddressMode::Accumulator => {
                let oc = if self.f & C == C {
                    0b00000001
                } else {
                    0b00000000
                };
                if self.acc >> 7 == 1 {
                    self.set_flag(C);
                } else {
                    self.clear_flag(C);
                }
                self.acc = (self.acc << 1) + oc;
                self.set_zero_neg_flags(self.acc);
            }
            _ => {
                let oc = if self.f & C == C {
                    0b00000001
                } else {
                    0b00000000
                };
                if byte >> 7 == 1 {
                    self.set_flag(C);
                } else {
                    self.clear_flag(C);
                }
                let res = (byte << 1) + oc;
                self.write_byte(res, address);
                self.set_zero_neg_flags(res);
            }
        }
    }

    // rotate right
    fn ror(&mut self, addr: AddressMode) {
        let address = self.get_address(addr);
        let byte = self.read_byte(address);
        match addr {
            AddressMode::Accumulator => {
                let oc = if self.f & C == C {
                    0b10000000
                } else {
                    0b00000000
                };
                if (self.acc << 7) >> 7 == 1 {
                    self.set_flag(C);
                } else {
                    self.clear_flag(C);
                }
                self.acc = (self.acc >> 1) + oc;
                self.set_zero_neg_flags(self.acc);
            }
            _ => {
                let oc = if self.f & C == C {
                    0b10000000
                } else {
                    0b00000000
                };
                if (byte << 7) >> 7 == 1 {
                    self.set_flag(C);
                } else {
                    self.clear_flag(C);
                }
                let res = (byte >> 1) + oc;
                self.write_byte(res, address);
                self.set_zero_neg_flags(res);
            }
        }
    }

    // ======================
    // FLAG INSTRUCTIONS
    // ======================

    // clear carry
    fn clc(&mut self) {
        self.clear_flag(C);
    }

    // clear decimal
    fn cld(&mut self) {
        self.clear_flag(D);
    }

    // clear interrupt disable
    fn cli(&mut self) {
        self.clear_flag(I);
    }

    // clear overflow
    fn clv(&mut self) {
        self.clear_flag(V);
    }

    // set carry
    fn sec(&mut self) {
        self.set_flag(C);
    }

    // set decimal
    fn sed(&mut self) {
        self.set_flag(D);
    }

    // set interrupt disable
    fn sei(&mut self) {
        self.set_flag(I);
    }

    // ======================
    // COMPARISON
    // ======================

    // compare (with accumulator)
    // SOLUTION: ... clear the damn values ...
    fn cmp(&mut self, addr: AddressMode) {
        let address = self.get_address(addr);
        let byte = self.read_byte(address);
        let (res, _) = self.acc.overflowing_sub(byte);
        if self.acc >= byte {
            self.set_flag(C);
        } else {
            self.clear_flag(C);
        }
        if self.acc == byte {
            self.set_flag(Z);
        } else {
            self.clear_flag(Z);
        }
        if res >> 7 == 1 {
            self.f |= N;
        } else {
            self.clear_flag(N);
        }
    }

    // compare with X
    fn cpx(&mut self, addr: AddressMode) {
        let address = self.get_address(addr);
        let byte = self.read_byte(address);
        let (res, _) = self.rx.overflowing_sub(byte);
        if self.rx >= byte {
            self.set_flag(C);
        } else {
            self.clear_flag(C);
        }
        if self.rx == byte {
            self.set_flag(Z);
        } else {
            self.clear_flag(Z);
        }
        if res >> 7 == 1 {
            self.f |= N;
        } else {
            self.clear_flag(N);
        }
    }

    // compare with Y
    fn cpy(&mut self, addr: AddressMode) {
        let address = self.get_address(addr);
        let byte = self.read_byte(address);
        let (res, _) = self.ry.overflowing_sub(byte);
        if self.ry >= byte {
            self.set_flag(C);
        } else {
            self.clear_flag(C);
        }
        if self.ry == byte {
            self.set_flag(Z);
        } else {
            self.clear_flag(Z);
        }
        if res >> 7 == 1 {
            self.set_flag(N);
        } else {
            self.clear_flag(N);
        }
    }

    // ======================
    // CONDITIONAL BRANCH INSTRUCTION
    // ======================

    // branch on carry clear
    fn bcc(&mut self) {
        let offset = self.fetch_byte();
        if self.f & C != C {
            self.branch(offset);
        }
    }

    // branch on carry set
    fn bcs(&mut self) {
        let offset = self.fetch_byte();
        if self.f & C == C {
            self.branch(offset);
        }
    }

    // branch on equal (zero set)
    fn beq(&mut self) {
        let offset = self.fetch_byte();
        if self.f & Z == Z {
            self.branch(offset);
        }
    }

    // branch on minus (negative set)
    fn bmi(&mut self) {
        let offset = self.fetch_byte();
        if self.f & N == N {
            self.branch(offset);
        }
    }

    // branch on not equal (zero clear)
    fn bne(&mut self) {
        let offset = self.fetch_byte();
        if self.f & Z != Z {
            self.branch(offset);
        }
    }

    // branch on plus (negative clear)
    fn bpl(&mut self) {
        let offset = self.fetch_byte();
        if self.f & N != N {
            self.branch(offset);
        }
    }

    // branch on overflow clear
    fn bvc(&mut self) {
        let offset = self.fetch_byte();
        if self.f & V != V {
            self.branch(offset);
        }
    }

    // branch on overflow set
    fn bvs(&mut self) {
        let offset = self.fetch_byte();
        if self.f & V == V {
            self.branch(offset);
        }
    }

    // ======================
    // JUMP & SUBROUTINES
    // ======================

    // jump
    fn jmp(&mut self, addr: AddressMode) {
        self.pc = self.get_address(addr);
    }

    // jump subroutine
    fn jsr(&mut self) {
        let subaddr = self.fetch_word();
        // The write_word function is more or less the same as the
        // Used push_stack functions...
        // I just prefer the push stack a little bit more
        // Else it does exactly the same job
        // self.write_word(self.pc, self.sp as u16);
        // self.sp += 2;
        let (ll, hh) = self.word_to_bytes(self.pc - 1);
        self.push_stack(hh);
        self.push_stack(ll);

        self.pc = subaddr;
    }

    // return from subroutine
    fn rts(&mut self) {
        // The read_word function is more or less the same as the
        // Used pop_stack functions...
        // I just prefer the pop stack a little bit more
        // Else it does exactly the same job
        // self.sp -= 2;
        // self.pc = self.read_word(self.sp as u16);
        let ll = self.pop_stack();
        let hh = self.pop_stack();

        self.pc = self.bytes_to_word(ll, hh) + 1;
    }

    // ======================
    // INTERRUPTS
    // ======================

    // break / interrupt
    // Force Break
    fn brk(&mut self) {
        let (ll, hh) = self.word_to_bytes(self.pc + 1);
        self.push_stack(hh);
        self.push_stack(ll);
        self.push_stack(self.f | B | R);
        self.set_flag(I);
        self.pc = self.read_word(0xFFFE);
    }

    // return from interrupt
    fn rti(&mut self) {
        self.f = self.pop_stack();
        let ll = self.pop_stack();
        let hh = self.pop_stack();
        self.pc = self.bytes_to_word(ll, hh);
    }

    // ======================
    // OTHER
    // ======================

    // bit test
    fn bit(&mut self, addr: AddressMode) {
        let address = self.get_address(addr);
        let byte = self.read_byte(address);
        if (self.acc & byte) == 0x0 {
            self.set_flag(Z);
        } else {
            self.clear_flag(Z);
        }
        self.f |= byte & (N | V);
        if byte & N == 0 {
            self.clear_flag(N);
        }
        if byte & V == 0 {
            self.clear_flag(V);
        }
    }

    // no operation
    fn nop(&mut self) {
        // A simple empty field
        // because what says truly more than
        // no operation than simply
        // not doing anything?
        // We could also do a wait here...
        // I will see in the future!
    }
}
