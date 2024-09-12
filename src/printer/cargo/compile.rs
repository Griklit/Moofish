use std::thread::sleep;
use std::time::Duration;
use std::cmp;

use rand::Rng;
use rand::seq::SliceRandom;

use crate::printer::Printer;
use super::data::{Crate, CRATES};

pub struct Compile<R: Rng + ?Sized> {
    pub colorful: bool,
    crates: Vec<Crate>,
    current: Crate,
    compiling: [Option<Crate>; 2],
    finished: Vec<Crate>,
    total_one: u16,
    progress_one: f32,
    rng: R,
}

impl<R: Rng + Sized> Compile<R> {
    pub fn new(mut rng: R, count: usize) -> Self {
        let mut crates = CRATES.choose_multiple(&mut rng, count).cloned().collect::<Vec<Crate>>();
        let current = crates.pop().unwrap();
        Compile {
            colorful: false,
            crates,
            current,
            compiling: [None, None],
            finished: Vec::new(),
            total_one: rng.gen_range(1..100),
            progress_one: 0.0,
            rng,
        }
    }
    fn current_version(&self) -> String {
        format!("v{}.{}.{}", self.current.version.0, self.current.version.1, self.current.version.2)
    }

    fn progress_bar(&self) -> String {
        let mut ret = String::with_capacity(22);
        ret.push_str("[");
        let count = cmp::min((20.0 * self.progress_one).round() as u8, 20);
        for _ in 0..count {
            ret.push_str("=");
        }
        ret.push_str(">");
        for _ in 0..(20 - count) {
            ret.push_str(" ");
        }
        ret.push_str("]");
        ret
    }

    fn progress_ratio(&self) -> String {
        format!("{}/{}", (self.total_one as f32 * self.progress_one).round() as u16, self.total_one)
    }

    fn compiling(&self) -> String {
        format!("\r\x1b[2K   Compiling {} {}\n", self.current.name, self.current_version())
    }

    fn building(&self) -> String {
        let mut line = String::new();
        line.push_str("\r\x1b[2K    Building ");
        line.push_str(&self.progress_bar().as_str());
        line.push(' ');
        line.push_str(&self.progress_ratio().as_str());
        line.push_str(": ");
        if let Some(c) = self.compiling[0] {
            line.push_str(&c.name);
        }
        if let Some(c) = self.compiling[1] {
            line.push_str(", ");
            line.push_str(&c.name);
        }
        line
    }
}

impl<R: Rng + Sized> Iterator for Compile<R> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        return if self.progress_one == 0.0 {
            self.progress_one += 0.01;
            Some(self.building())
        } else if self.progress_one >= 1.0 {
            let ret = Some(self.compiling());
            self.finished.push(self.current.clone());
            self.current = self.crates.pop()?;
            self.total_one = self.rng.gen_range(1..100);
            let finished = self.finished.choose_multiple(&mut self.rng, 2).cloned().collect::<Vec<Crate>>();
            match finished.len() {
                0 => {
                    self.compiling = [None, None];
                }
                1 => {
                    self.compiling = [Some(finished[0].clone()), None];
                }
                _ => {
                    self.compiling = [Some(finished[0].clone()), Some(finished[1].clone())];
                }
            }
            self.progress_one = 0.0;
            ret
        } else {
            let add = self.rng.gen_range(0.01..0.5);
            let duration = Duration::from_millis(self.rng.gen_range(100..500));
            sleep(duration);
            self.progress_one += add;
            Some(self.building())
        };
    }
}

impl<R: Rng + Sized> Printer for Compile<R> {
    fn colorful(&mut self, enable: bool) -> &mut Self {
        self.colorful = enable;
        self
    }
}


#[test]
fn test() {
    let x = Compile::default();
    for i in x {
        println!("{}", i);
    }
}