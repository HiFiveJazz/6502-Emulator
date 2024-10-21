
// Data Sheet on the 6502
// https://web.archive.org/web/20210909190432/http://www.obelisk.me.uk/6502/
struct BitField {
    data: u8,
    size: usize,
}

impl BitField {
    fn new(size: usize) -> Self {
        assert!(size <=8, "Size must be 8 bits or less!");
        BitField { data: 0, size}
    }

    fn set_bit(&mut self, index: usize, value: bool) {
        if value {
            self.data |= 1 <<index;
        } else {
            self.data &= !(1 << index);
        }
    }

    fn get_bit(&self, index: usize) -> bool {
        (self.data & (1<<index)) != 0
    }
}


type Byte = u8;
type Word = u16;


struct CPU {
    PC: Word, //Program Counter
    SP: Word, //Stack Pointer

    //Register
    A: Byte, //Accumulator Register
    X: Byte, //Register X
    Y: Byte, //Register Y




};

fn main () {

    return 0;
}
