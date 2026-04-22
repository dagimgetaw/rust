fn main() {
    // there are two types of integer signed (include negative and posetive number) and unsigned (only posetive number)

    /*
        length                      signed       unsigned
        8-bit                         i8            u8
        16-bit                        i16           u16
        32-bit                        i32           u32
        64-bit                        i64           u64
        128-bit                       i128          u128
        architecture-dependent        isize         usize
     */

    // for signed from −(2^(n − 1)) to 2^(n − 1) − 1 inclusive
    // for unsigned from 0 to 2^(n) - 1

    // example 
    // 8-bit in signed from -(2^(8 - 1)) to 2^(8 - 1) -1 => -128 to 127
    // 8-bit in unsigned from 0 to 2^(8) - 1 => 0 to 255

    // rust lets you write the same number in different formats
    
    /*
        number literals	       example
        decimal	                98_222
        hex	                    0xff
        octal	                0o77
        binary	                0b1111_0000
        byte (u8 only)	        b'A'
     */

    // 98_222 is same as 98222 or 1_000 same as 1000 '_' it is just for readability
    // 0x → hex
    // 0o → octal
    // 0b → binary

    let x : i8 = 10;
    let y = 10i8;

    println!("the value of x = {} and the value of y = {}", x, y);

    // output -- the value of x = 10 and the value of y = 10
    // rust lets you write numbers in different formats, but they all become normal numbers when used.
}