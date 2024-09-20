use std::cmp;
use std::thread::sleep;
use std::time::Duration;

use rand::{Rng, SeedableRng};
use rand::seq::SliceRandom;

use crate::data::crates::{Crate, CRATES};
use crate::printer::ShellOutput;

#[derive(Clone, Debug)]
pub struct Compile<R: Rng> {
    pub colorful: bool,
    rng: R,
    crates: Vec<(Crate, Vec<Crate>)>,
    current: Crate,  // 当前包
    dependencies: Vec<Crate>,  // 当前包的依赖
    dependency: Option<Crate>,  // 当前包正在编译的依赖，若为None则表示正在编译当前包
    completed: usize,
    total: usize,
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
            completed: 0,
            total,
            rng,
        }
    }

    fn progress_bar(&self) -> String {
        let mut ret = String::with_capacity(22);
        ret.push_str("[");
        let count = cmp::min(20 * self.completed / self.total, 20);
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
        let mut line = "\r\x1b[2K".to_string();
        if self.colorful {
            line.push_str("   \x1b[1;32mCompiling\x1b[0m ");
        } else {
            line.push_str("   Compiling ");
        }
        if let Some(dependency) = self.dependency {
            line.push_str(format!("{} {}", dependency.name, dependency.version).as_str());
        } else {
            line.push_str(format!("{} {}", self.current.name, self.current.version).as_str());
        }
        if cfg!(target_os = "windows") {
            line.push_str("\r\n");
        } else {
            line.push_str("\n");
        }
        line
    }

    fn building(&self) -> String {
        let mut line = String::new();
        // line.push_str("\r\x1b[2K");
        if self.colorful {
            line.push_str("    \x1b[1;34mBuilding\x1b[0m ");
        } else {
            line.push_str("    Building ");
        }
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
        let ret = Some(self.compiling() + &self.building());
        if self.completed != 0 {
            let sleep_range = match self.rng.gen_range(0..=255u8) {
                0..=15 => 500..=4000,
                16..=63 => 200..=500,
                _ => 50..=200,
            };
            sleep(Duration::from_millis(self.rng.gen_range(sleep_range)))
        }
        if self.dependency.is_none() {
            let (current, dependencies) = self.crates.pop()?;
            self.current = current;
            self.dependencies = dependencies;
        }
        self.dependency = self.dependencies.pop();
        self.completed += 1;
        ret
    }
}

impl<R: Rng> ShellOutput for Compile<R> {
    fn colorful(&mut self, enable: bool) -> &mut Self {
        self.colorful = enable;
        self
    }
}

impl Default for Compile<rand_xorshift::XorShiftRng> {
    fn default() -> Self {
        Compile::new(rand_xorshift::XorShiftRng::from_entropy(), 32)
    }
}