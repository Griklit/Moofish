use std::io::{stdout, Write};

use rand::SeedableRng;
use rand_xorshift::XorShiftRng;

use moofish::{Printer, tar};

fn main() {
    let mut x = tar::Extract::new(XorShiftRng::from_entropy(), 4096);
    x.colorful(true);
    let mut io = stdout();
    for i in x {
        print!("{}", i);
        io.flush().unwrap();
    }
}