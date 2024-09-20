use rand::{Rng, SeedableRng};
use rand_xorshift::XorShiftRng;

use crate::ShellOutput;

use super::compile::Compile;

#[derive(Clone, Debug)]
enum CargoComponent {
    Compile(Compile<XorShiftRng>)
}

impl Iterator for CargoComponent {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            CargoComponent::Compile(c) => c.next(),
        }
    }
}

impl ShellOutput for CargoComponent {
    fn colorful(&mut self, enable: bool) -> &mut Self {
        match self {
            CargoComponent::Compile(c) => { c.colorful(enable); }
        }
        self
    }
}

#[derive(Clone, Debug)]
pub struct Cargo {
    pub colorful: bool,
    printer: CargoComponent,
}

impl Cargo {
    fn random_printer() -> CargoComponent {
        let mut rng = XorShiftRng::from_entropy();
        let count = rng.gen_range(8..=512);
        CargoComponent::Compile(Compile::new(rng, count))
    }
}


impl Iterator for Cargo {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let line = self.printer.next();
        if line.is_some() {
            line
        } else {
            self.printer = Cargo::random_printer();
            self.next()
        }
    }
}

impl ShellOutput for Cargo {
    fn colorful(&mut self, enable: bool) -> &mut Self {
        self.colorful = enable;
        self
    }
}


impl Default for Cargo {
    fn default() -> Self {
        Cargo {
            colorful: false,
            printer: Cargo::random_printer(),
        }
    }
}
