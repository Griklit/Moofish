use std::str::FromStr;
use rand_xorshift::XorShiftRng;

use crate::ShellOutput;

use super::{cargo, tar};

#[derive(Debug, Clone)]
pub enum ShellAdapter {
    Cargo(cargo::Cargo),
    // Pip,
    Tar(tar::Extract<XorShiftRng>), // TODO
}


impl FromStr for ShellAdapter {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "cargo" => Ok(ShellAdapter::Cargo(cargo::Cargo::default())),
            // "pip" => Ok(Printers::Pip),
            "tar" => Ok(ShellAdapter::Tar(tar::Extract::default())),
            _ => Err("Invalid printer".to_string())
        }
    }
}

impl Iterator for ShellAdapter {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            ShellAdapter::Cargo(c) => c.next(),
            // PrinterTypes::Pip => unimplemented!(),
            ShellAdapter::Tar(t) => t.next(),
        }
    }
}

impl ShellOutput for ShellAdapter {
    fn colorful(&mut self, enable: bool) -> &mut Self {
        match self {
            ShellAdapter::Cargo(c) => { c.colorful(enable); }
            // PrinterTypes::Pip => unimplemented!(),
            ShellAdapter::Tar(t) => { t.colorful(enable); }
        }
        self
    }
}