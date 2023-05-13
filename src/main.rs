use core::panic;
// Imports for reading a file
use std::fs::File;
use std::io;
use std::io::prelude::*;

const DEBUGLOG: bool = true;

const MEMORY: usize = 65536;

// The flag for the flag in the Chip
enum Flag {
    N = 0x80, // [1000 0000] negative
    V = 0x40, // [0100 0000] overflow
              // [0010 0000] Reserved
    B = 0x10, // [0001 0000] break
    D = 0x08, // [0000 1000] decimale
    I = 0x04, // [0000 0100] interrpt disable
    Z = 0x02, // [0000 0010] zero
    C = 0x00, // [0000 0001] carry
}

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
    pub f: Flag,
    // Stack Pointer:
    pub sp: u8,
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
            acc: 0,
            rx: 0,
            ry: 0,
            f: Flag::Z,
            sp: 0,
            pc: 0x0200, // 256 + 256
            memory: [0; MEMORY],
        }
    }

    /// Helper functions

    fn push_stack(&mut self, address: u8) {
        self.memory[0x0100 + self.sp as usize] = address;
        // self.sp += 1;
    }

    fn pop_stack(&mut self) {
        self.memory[0x0100 + self.sp as usize] = 0;
        self.sp -= 1;
    }

    fn get_address(&mut self, addr: AddressMode) -> u8 {
        match addr {
            AddressMode::Immediate => {
                return self.memory[(self.pc) as usize];
            }
            AddressMode::Absolute => {
                self.pc += 2;
                let ll = self.memory[(self.pc - 1) as usize];
                let hh = self.memory[(self.pc) as usize];
                let address = (ll as u16) + ((hh as u16) << 8);
                return self.memory[(address) as usize];
            }
            AddressMode::Zeropage => {
                let ll = self.memory[(self.pc - 1) as usize];
                let address = ll as u16;
                return self.memory[(address) as usize];
            }
            AddressMode::AbsoluteX => {
                self.pc += 2;
                let ll = self.memory[(self.pc - 1) as usize];
                let hh = self.memory[(self.pc) as usize];
                let address = (ll as u16) + ((hh as u16) << 8);
                let x = self.rx;
                return self.memory[(address) as usize] + x;
            }
            AddressMode::AbsoluteY => {
                self.pc += 2;
                let ll = self.memory[(self.pc - 1) as usize];
                let hh = self.memory[(self.pc) as usize];
                let address = (ll as u16) + ((hh as u16) << 8);
                let y = self.ry;
                return self.memory[(address) as usize] + y;
            }
            AddressMode::ZeropageX => {
                let ll = self.memory[(self.pc - 1) as usize];
                let address = ll as u16;
                let x = self.rx;
                return self.memory[(address) as usize] + x;
            }
            AddressMode::ZeropageY => {
                let ll = self.memory[(self.pc - 1) as usize];
                let address = ll as u16;
                let y = self.ry;
                return self.memory[(address) as usize] + y;
            }
            AddressMode::Indirect => {
                self.pc += 2;
                let ll = self.memory[(self.pc - 1) as usize];
                let hh = self.memory[(self.pc) as usize];
                let address = (ll as u16) + ((hh as u16) << 8);
                let ll_2 = self.memory[(address) as usize];
                let hh_2 = self.memory[(address + 1) as usize];
                let address_2 = (ll_2 as u16) + ((hh_2 as u16) << 8);
                return self.memory[address_2 as usize];
            }
            AddressMode::XIndirect => {
                let ll = self.memory[(self.pc - 1) as usize];
                let x = self.rx;
                let address = (ll + x) as u16;
                let ll_2 = self.memory[(address) as usize];
                let hh_2 = self.memory[(address + 1) as usize];
                let address_2 = (ll_2 as u16) + ((hh_2 as u16) << 8);
                return self.memory[(address_2) as usize];
            }
            AddressMode::IndirectY => {
                let ll = self.memory[(self.pc - 1) as usize];
                let y = self.ry;
                let address = ll as u16;
                let ll_2 = self.memory[(address) as usize];
                let hh_2 = self.memory[(address + 1) as usize];
                let address_2 = (ll_2 as u16) + ((hh_2 as u16) << 8) + y as u16;
                return self.memory[(address_2) as usize];
            }
            AddressMode::Relative => {
                let off = self.memory[self.pc as usize];
                return off;
            }
            _ => { return 0; }
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
        let opcode: u8 = read_word(self.memory, self.pc);
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
            self.pc += 1;
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
            (0x2, 0x0) => self.jsr(AddressMode::Absolute),
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
            (0x8, 0x8) => self.dey(AddressMode::Implied),
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
            (0xC, 0x8) => self.iny(AddressMode::Implied),
            (0xC, 0x9) => self.cmp(AddressMode::Immediate),
            (0xC, 0xC) => self.cpy(AddressMode::Absolute),
            (0xC, 0xD) => self.cmp(AddressMode::Absolute),
            (0xC, 0xA) => self.dex(AddressMode::Implied),
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
            (0xE, 0x8) => self.inx(AddressMode::Implied),
            (0xE, 0x9) => self.sbc(AddressMode::Immediate),
            (0xE, 0xA) => self.nop(AddressMode::Implied),
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
    }

    // load X
    fn ldx(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("ldx");
        }
    }

    // load Y
    fn ldy(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("ldy");
        }
    }

    // store accumulator
    fn sta(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("sta");
        }
    }

    // store X
    fn stx(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("stx");
        }
    }

    // store Y
    fn sty(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("sty");
        }
    }

    // transfer accumulator to X
    fn tax(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("tax");
        }
    }

    // transfer accumulator to Y
    fn tay(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("tay");
        }
    }

    // transfer stack pointer to X
    fn tsx(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("tsx");
        }
    }

    // transfer X to accumulator
    fn txa(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("txa");
        }
    }

    // transfer X to stack pointer
    fn txs(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("txs");
        }
    }

    // transfer Y to accumulator
    fn tya(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("tya");
        }
    }

    /// ======================
    /// STACK INSTRUCTIONS
    /// ======================

    // push accumulator
    fn pha(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("pha");
        }
    }

    // push processor status (SR)
    fn php(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("php");
        }
    }

    // pull accumulator
    fn pla(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("pla");
        }
    }

    // pull processor status (SR)
    fn plp(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("plp");
        };
    }

    /// ======================
    /// DECREMENTS & INCREMENTS
    /// ======================

    // decrement
    fn dec(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("dec");
        };
    }

    // decrement X
    fn dex(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("dex");
        };
    }

    // decrement Y
    fn dey(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("dey");
        };
    }

    // increment
    fn inc(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("inc");
        };
    }

    // increment X
    fn inx(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("inx");
        };
    }

    // increment Y
    fn iny(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("iny");
        };
    }

    /// ======================
    /// ARITHMETIC OPERATIONS
    /// ======================

    // add with carry
    fn adc(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("adc");
        };
    }

    // subtract with carry
    fn sbc(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("sbc");
        };
    }

    /// ======================
    /// ALOGICAL OPERATIONS
    /// ======================

    // and (with accumulator)
    fn and(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("and")
        }
        let a = self.get_address(addr);
        self.acc &= a;
    }

    // exclusive or (with accumulator)
    fn eor(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("eor")
        }
    }

    // or with accumulator
    fn ora(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("ora");
        };
    }

    /// ======================
    /// SHIFT & ROTATE INSTRUCTIONS
    /// ======================

    // arithmetic shift left
    fn asl(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("asl");
        };
    }

    // logical shift right
    fn lsr(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("lsr");
        };
    }

    // rotate left
    fn rol(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("rol");
        };
    }

    // rotate right
    fn ror(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("ror");
        };
    }

    /// ======================
    /// FLAG INSTRUCTIONS
    /// ======================

    // clear carry
    fn clc(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("clc");
        };
    }

    // clear decimal
    fn cld(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("cld");
        };
    }

    // clear interrupt disable
    fn cli(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("cli");
        };
    }

    // clear overflow
    fn clv(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("clv");
        };
    }

    fn sec(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("sec");
        };
    }

    // set decimal
    fn sed(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("sed");
        };
    }

    // set interrupt disable
    fn sei(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("sei");
        };
    }

    /// ======================
    /// COMPARISON
    /// ======================

    // compare (with accumulator)
    fn cmp(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("cmp");
        };
    }

    // compare with X
    fn cpx(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("cpx");
        };
    }

    // compare with Y
    fn cpy(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("cpy");
        };
    }

    /// ======================
    /// CONDITIONAL BRANCH INSTRUCTION
    /// ======================

    // branch on carry clear
    fn bcc(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("bcc");
        };
    }

    // branch on carry set
    fn bcs(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("bcs");
        };
    }

    // branch on equal (zero set)
    fn beq(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("beq");
        };
    }

    // branch on minus (negative set)
    fn bmi(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("bmi");
        };
    }

    // branch on not equal (zero clear)
    fn bne(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("bne");
        };
    }

    // branch on plus (negative clear)
    fn bpl(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("bpl");
        };
    }

    // branch on overflow clear
    fn bvc(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("bvc");
        };
    }

    // branch on overflow set
    fn bvs(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("bvs");
        };;
    }

    /// ======================
    /// JUMP & SUBROUTINES
    /// ======================

    // jump
    fn jmp(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("jmp");
        };
    }

    // jump subroutine
    fn jsr(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("jsr");
        };
    }

    // return from subroutine
    fn rts(&mut self) {
        if DEBUGLOG {
            println!("rts");
        }
        self.pc += 1;
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
        self.f = Flag::B;
    }

    // return from interrupt
    fn rti(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("rti");
        };;
    }

    /// ======================
    /// OTHER
    /// ======================

    // bit test
    fn bit(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("bit");
        };
    }

    // no operation
    fn nop(&mut self, addr: AddressMode) {
        if DEBUGLOG {
            println!("nop");
        };
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


#[cfg(test)]
mod tests {
    use crate::Chip;

    #[test]
    fn test_working() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn load_accumulator_immediate_mode() {
        let mut c = Chip::new();

        c.memory[512] = 0x49;
        c.memory[513] = 0x01;
    }
}