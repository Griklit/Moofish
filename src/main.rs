use std::io::{Write, stdout};

use rand::rngs::ThreadRng;

use moofish::{Printer, cargo};

fn main() {
    let mut x = cargo::Compile::new(ThreadRng::default(), 512);
    x.colorful(true);
    let mut io = stdout();
    for i in x {
        print!("{}", i);
        io.flush().unwrap();
    }
}