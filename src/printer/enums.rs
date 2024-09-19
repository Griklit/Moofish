use std::str::FromStr;
use rand_xorshift::XorShiftRng;

use super::{cargo, tar};

#[derive(Debug, Clone)]
pub enum PrinterTypes {
    Cargo(cargo::Compile<XorShiftRng>),
    // Pip,
    Tar(tar::Extract<XorShiftRng>),
}


impl FromStr for PrinterTypes {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "cargo" => Ok(PrinterTypes::Cargo(cargo::Compile::default())),
            // "pip" => Ok(Printers::Pip),
            "tar" => Ok(PrinterTypes::Tar(tar::Extract::default())),
            _ => Err("Invalid printer".to_string())
        }
    }
}
