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
            pc:0,
            memory: [0; MEMORY]
        }
    }
}

fn main() {
    println!("Hello, world!");
}
