use std::io::{stdout, Write};

use moofish::{Printer, tar};

fn main() {
    let mut x = tar::Extract::default();
    x.colorful(true);
    let mut io = stdout();
    for i in x {
        print!("{}", i);
        io.flush().unwrap();
    }
}