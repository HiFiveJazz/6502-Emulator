
// Data Sheet on the 6502
// https://web.archive.org/web/20210909190432/http://www.obelisk.me.uk/6502/


//     let mut bit_field = BitField::new(4);
//     bit_field.set_bit(0,1);
//     bit_field.set_bit(1,0);
//     bit_field.set_bit(2,1);
//     bit_field.set_bit(3,1);
//
//     for i in 0..4 {
//         println!("Bit {} is: {}", i, bit_field.get_bit(i));
//     }



struct BitField {
    data: u8,
    size: usize,
}

impl BitField {
    fn new(size: usize) -> Self {
        assert!(size <=8, "Size must be 8 bits or less!");
        BitField { data: 0, size}
    }

    fn set_bit(&mut self, index: usize, value: u8) {
        if index >= self.size {
            panic!("Index out of bounds");
        }
        if value == 1 {
            self.data |= 1 <<index; // Set the bit at 'index'
        } else if value == 0 {
            self.data &= !(1 << index); // Clear the bit at 'index'
        } else {
            panic!("Value must be 0 or 1");
        }
    }

    fn get_bit(&self, index: usize) -> u8 {
        if (self.data & (1 << index)) !=0 {
            1
        } else {
            0
        }
    }

    fn data(&self) -> u8 {
        self.data
    }
} 

type Byte = u8;
type Word = u16;

struct CPU {
    PC: Word, // Program Counter
    SP: Word, // Stack Pointer

    // Register
    A: Byte, // Accumulator Register
    X: Byte, // Register X
    Y: Byte, // Register Y

    // Status Flags (all are 1 bit) 
    C: bool, // Carry Flag
    Z: bool, // Zero Flag
    I: bool, // Interrupt Disable
    D: bool, // Decimal Mode
    B: bool, // Break Command
    V: bool, // Overflow Flag
    N: bool, // Negative Flag

    fn Reset() -> () {
        // Pointers
        let PC = 0xFFFC;
        let SP = 0x0100;

        // Clearing Registers
        let A = 0;
        let X = 0;
        let Y = 0;

        // Clearing Status Flags
        let C = false;
        let Z = false;
        let I = false;
        let D = false;
        let B = false;
        let V = false;
        let N = false;
    }
};

fn main () {

    let mut cpu = CPU;
    return 0;
}

//     let mut bit_field = BitField::new(4);
//     bit_field.set_bit(0,1);
//     bit_field.set_bit(1,0);
//     bit_field.set_bit(2,1);
//     bit_field.set_bit(3,1);
//
//     for i in 0..4 {
//         println!("Bit {} is: {}", i, bit_field.get_bit(i));
//     }
