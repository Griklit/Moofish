use std::thread::sleep;
use std::time::Duration;
use std::cmp;

use rand::Rng;
use rand::seq::SliceRandom;

use crate::printer::Printer;
use super::data::{Crate, CRATES};

pub struct Compile<R: Rng> {
    pub colorful: bool,
    rng: R,
    crates: Vec<(Crate, Vec<Crate>)>,
    current: Crate,
    dependencies: Vec<Crate>,
    dependency: Option<Crate>,
    total: usize,
    progress_one: f32,
}

impl<R: Rng> Compile<R> {
    pub fn new(mut rng: R, count: usize) -> Self {
        let mut crates = CRATES.choose_multiple(&mut rng, count).cloned().collect::<Vec<Crate>>();
        let total = crates.len();
        let mut crate_with_depends = Vec::new();
        while let Some(current) = crates.pop() {
            let mut dependencies = Vec::new();
            let d_count = rng.gen_range(0..16);
            for _ in 0..d_count {
                let d = crates.pop();
                if let Some(d) = d {
                    dependencies.push(d);
                } else {
                    break;
                }
            }
            crate_with_depends.push((current, dependencies));
        }
        let (current, mut dependencies) = crate_with_depends.pop().unwrap();
        let dependency = dependencies.pop();

        Compile {
            colorful: false,
            crates: crate_with_depends,
            current,
            dependencies,
            dependency,
            total,
            progress_one: 0.0,
            rng,
        }
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
        let current = self.total - (
            self.crates.len()
                + self.crates.iter().map(|(_, d)| d.len()).sum::<usize>()
                + self.dependencies.len()
                + self.dependency.is_some() as usize
        );
        format!("{}/{}", current, self.total)
    }

    fn compiling(&self) -> String {
        if let Some(dependency) = self.dependency {
            format!("\r\x1b[2K   Compiling {} {}\n", dependency.name, dependency.version)
        } else {
            format!("\r\x1b[2K   Compiling {} {}\n", self.current.name, self.current.version)
        }
    }

    fn building(&self) -> String {
        let mut line = String::new();
        line.push_str("\r\x1b[2K    Building ");
        line.push_str(&self.progress_bar().as_str());
        line.push(' ');
        line.push_str(&self.progress_ratio().as_str());
        line.push_str(": ");
        line.push_str(self.current.name);
        if let Some(dependency) = &self.dependency {
            line.push_str(", ");
            line.push_str(dependency.name);
        }
        line
    }
}

impl<R: Rng> Iterator for Compile<R> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        return if self.progress_one == 0.0 {
            self.progress_one += 0.01;
            Some(self.building())
        } else if self.progress_one >= 1.0 {
            let ret = Some(self.compiling());
            if self.dependency.is_none() {
                let (current, dependencies) = self.crates.pop()?;
                self.current = current;
                self.dependencies = dependencies;
                self.dependency = self.dependencies.pop();
                self.progress_one = 0.0;
            } else {
                self.dependency = self.dependencies.pop();
                self.progress_one = 0.0;
            }
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

impl<R: Rng> Printer for Compile<R> {
    fn colorful(&mut self, enable: bool) -> &mut Self {
        self.colorful = enable;
        self
    }
}