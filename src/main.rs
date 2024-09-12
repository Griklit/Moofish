use std::io::{stdout, Write};

use rand::SeedableRng;
use rand_xorshift::XorShiftRng;

use moofish::{cargo, Printer};

fn main() {
    let mut x = cargo::Compile::new(XorShiftRng::from_entropy(), 512);
    x.colorful(true);
    let mut io = stdout();
    for i in x {
        print!("{}", i);
        io.flush().unwrap();
    }
}