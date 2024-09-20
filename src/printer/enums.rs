use std::str::FromStr;
use rand_xorshift::XorShiftRng;

use crate::Printer;

use super::{cargo, tar};

#[derive(Debug, Clone)]
pub enum PrinterTypes {
    Cargo(cargo::Cargo),
    // Pip,
    Tar(tar::Extract<XorShiftRng>), // TODO
}


impl FromStr for PrinterTypes {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "cargo" => Ok(PrinterTypes::Cargo(cargo::Cargo::default())),
            // "pip" => Ok(Printers::Pip),
            "tar" => Ok(PrinterTypes::Tar(tar::Extract::default())),
            _ => Err("Invalid printer".to_string())
        }
    }
}


impl Iterator for PrinterTypes {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            PrinterTypes::Cargo(c) => c.next(),
            // PrinterTypes::Pip => unimplemented!(),
            PrinterTypes::Tar(t) => t.next(),
        }
    }
}

impl Printer for PrinterTypes {
    fn colorful(&mut self, enable: bool) -> &mut Self {
        match self {
            PrinterTypes::Cargo(c) => { c.colorful(enable); }
            // PrinterTypes::Pip => unimplemented!(),
            PrinterTypes::Tar(t) => { t.colorful(enable); }
        }
        self
    }
}