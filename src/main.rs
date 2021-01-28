type Byte = u8;
type Word = u16;

struct CPU {
    pc: Word, // Program counter
    sp: Word, // Stack pointer

    // Registers
    a: Byte,
    x: Byte,
    y: Byte,

    // Status flag
    c: bool,
    z: bool,
    i: bool,
    d: bool,
    b: bool,
    v: bool,
    n: bool,
}

impl CPU {
    fn new() -> Self {
        CPU {
            pc: 0,
            sp: 0,
            a: 0,
            x: 0,
            y: 0,
            c: false,
            z: false,
            i: false,
            d: false,
            b: false,
            v: false,
            n: false,
        }
    }

    fn reset(&mut self) {
        self.pc = 0xFFFC;
        self.sp = 0x0100;

        self.d = false;
        self.c = false;
        self.z = false;
        self.i = false;
        self.b = false;
        self.v = false;
        self.n = false;

        self.a = 0;
        self.x = 0;
        self.y = 0;
    }
}

fn main() {
    let mut cpu = CPU::new();
    cpu.reset();
}
