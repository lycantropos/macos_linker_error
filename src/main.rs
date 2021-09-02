use rithm::BigInt;
use std::convert::TryFrom;

fn main() {
    type Digit = u16;
    const SHIFT: usize = (Digit::BITS - 1) as usize;
    println!("Hello {}", BigInt::<Digit, SHIFT>::try_from("0").unwrap());
}
