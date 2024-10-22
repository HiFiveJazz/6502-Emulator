
// Data Sheet on the 6502
// https://web.archive.org/web/20210909190432/http://www.obelisk.me.uk/6502/

// struct BitField {
//     data: u8,
//     size: usize,
// }
//
// impl BitField {
//     fn new(size: usize) -> Self {
//         assert!(size <=8, "Size must be 8 bits or less!");
//         BitField { data: 0, size}
//     }
//
//     fn set_bit(&mut self, index: usize, value: u8) {
//         if index >= self.size {
//             panic!("Index out of bounds");
//         }
//         if value == 1 {
//             self.data |= 1 <<index; // Set the bit at 'index'
//         } else if value == 0 {
//             self.data &= !(1 << index); // Clear the bit at 'index'
//         } else {
//             panic!("Value must be 0 or 1");
//         }
//     }
//
//     fn get_bit(&self, index: usize) -> u8 {
//         if (self.data & (1 << index)) !=0 {
//             1
//         } else {
//             0
//         }
//     }
//
//     fn data(&self) -> u8 {
//         self.data
//     }
// } 

// Example Usage
//     let mut bit_field = BitField::new(4);
//     bit_field.set_bit(0,1);
//     bit_field.set_bit(1,0);
//     bit_field.set_bit(2,1);
//     bit_field.set_bit(3,1);
//
//     for i in 0..4 {
//         println!("Bit {} is: {}", i, bit_field.get_bit(i));
//     }
// in CPP
// struct Mem
// {
//         static constexpr u32 MAX_MEM = 1024 * 64;
//         Byte Data[MAX_MEM];
//
const MAX_MEM: usize = 1024 * 64;

struct Mem {
    data: [u8; MAX_MEM], // Array to hold memory data
}

impl Mem {
    fn intialization() -> Self {
        Mem {
            data: [0; MAX_MEM]
        }
    }

    fn read_byte(&self, address: usize) -> u8 {
        assert!(address < MAX_MEM, "Address is out of bounds");
        self.data[address] // Return the value from memory
    }
}

struct CPU {
    pc: u16, // Program Counter
    sp: u16, // Stack Pointer

    // Registers
    a: u8, // Accumulator Register
    x: u8, // Register X
    y: u8, // Register Y

    // Status Flags (all are 1 bit) 
    c: bool, // Carry Flag
    z: bool, // Zero Flag
    i: bool, // Interrupt Disable
    d: bool, // Decimal Mode
    b: bool, // Break Command
    v: bool, // Overflow Flag
    n: bool, // Negative Flag
}

impl CPU {
    fn reset(&mut self, _memory: &mut Mem) {
        // Reset Pointers
        self.pc = 0xFFFC;
        self.sp = 0x0100;

        // Clearing Registers
        self.a = 0;
        self.x = 0;
        self.y = 0;

        // Clearing Status Flags
        self.c = false;
        self.z = false;
        self.i = false;
        self.d = false;
        self.b = false;
        self.v = false;
        self.n = false;
    }

    fn fetch_byte(&mut self, memory: &mut Mem) -> u8 {
        let data = memory.read_byte(self.pc as usize);
        self.pc += 1;
        data // Return the fetched byte
    }

    fn execute(&mut self, mut cycles: u32, memory: &mut Mem) {
        while cycles > 0 {
            let instruction = self.fetch_byte(memory); // Fetch the next instruction in memory
            println!("Fetched instruction: {}", instruction);
            cycles -= 1; // Adjust cycles as needed
        }
    }

    fn print_contents(&mut self) {
        println!(
            "----------CPU Contents----------\n\
            Program Counter (PC):     {:04X}\n\
            Stack Pointer (SP):       {:04X}\n\
            Accumulator Register (A): {}\n\
            X Register (X):           {}\n\
            Y Register (Y):           {}\n\
            Carry Flag (C):           {}\n\
            Zero Flag (Z):            {}\n\
            Interrupt Disable (I):    {}\n\
            Decimal Mode (D):         {}\n\
            Break Command (B):        {}\n\
            Overflow Flag (V):        {}\n\
            Negative Flag (N):        {}\n",
            self.pc, self.sp, self.a, self.x,
            self.y, self.c, self.z, self.i,
            self.d, self.b, self.v, self.n
        );
    }
}

fn main() {
    let mut mem = Mem::intialization();
    let mut cpu = CPU {
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
    };
    cpu.reset(&mut mem);
    cpu.execute(2, &mut mem);
    cpu.print_contents();
}
// }
