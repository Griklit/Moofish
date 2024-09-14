/// tar -zxvf
use std::path::PathBuf;
use std::thread::sleep;
use std::time::Duration;

use rand::{Rng, SeedableRng};
use rand::seq::SliceRandom;

use crate::data::words::{EXTENSIONS, WORDS};
use crate::Printer;

pub struct Extract<R: Rng> {
    rng: R,
    path: PathBuf,
    filename: Option<(String, String)>,
    limit: usize,
}

impl<R: Rng> Extract<R> {
    pub fn new(rng: R, count: usize) -> Self {
        Extract {
            rng,
            path: PathBuf::new(),
            filename: None,
            limit: count,
        }
    }
    fn random_gen_stem(&mut self) -> String {
        const SEPARATORS: [char; 2] = ['-', '_'];
        let separator = SEPARATORS.choose(&mut self.rng).unwrap();
        let count = self.rng.gen_range(1..=6);
        let words = WORDS.choose_multiple(&mut self.rng, count).map(|x| x.to_string()).collect::<Vec<String>>();
        words.join(separator.to_string().as_str())
    }

    /// 随机改变文件名，1/8概率改变扩展名
    fn random_filename(&mut self) -> &mut Self {
        let stem = self.random_gen_stem();
        if let Some((_, ext)) = &self.filename {
            if self.rng.gen_range(u8::MIN..=u8::MAX) < 32 {
                let ext = EXTENSIONS.choose(&mut self.rng).unwrap().to_string();
                self.filename = Some((stem, ext));
            } else {
                self.filename = Some((stem, ext.to_string()));
            }
        } else {
            let ext = EXTENSIONS.choose(&mut self.rng).unwrap().to_string();
            self.filename = Some((stem, ext));
        }
        self
    }

    /// 随机改变路径
    fn random_path(&mut self) -> &mut Self {
        if self.rng.gen_range(0..100) > self.path.components().count() * 5 {
            for _ in 0..=self.rng.gen_range(1..=4) {
                let word = WORDS.choose(&mut self.rng).unwrap().to_string();
                self.path.push(word);
            }
            self.filename = None;
        } else {
            for _ in 0..=self.rng.gen_range(1..=4) {
                self.path.pop();
            }
            self.random_filename();
        }
        self
    }

    fn random(&mut self) -> &mut Self {
        if self.rng.gen_range(0..16) > 12 && self.filename.is_some() {
            self.random_path();
        } else {
            self.random_filename();
        }
        self
    }

    fn line(&self) -> String {
        let mut ret: String;
        if let Some((name, ext)) = &self.filename {
            let mut path = self.path.clone();
            path.push(format!("{}.{}", name, ext));
            ret = path.to_string_lossy().to_string()
        } else {
            ret = self.path.to_string_lossy().to_string()
        }
        ret.push_str("\n");
        ret
    }
}

impl<R: Rng> Iterator for Extract<R> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.limit == 0 {
            return None;
        }
        let ret = self.line();
        self.random();
        self.limit -= 1;
        let file_size_level = self.rng.gen_range(u8::MIN..=u8::MAX);
        let duration_range;
        if file_size_level > 8 {
            duration_range = 5..=10;
        } else if file_size_level > 1 {
            duration_range = 100..=200;
        } else {
            duration_range = 500..=1000;
        }
        sleep(Duration::from_millis(self.rng.gen_range(duration_range)));
        Some(ret)
    }
}

impl<R: Rng> Printer for Extract<R> {
    /// 设置是否启用彩色输出，此处不支持
    fn colorful(&mut self, _enable: bool) -> &mut Self {
        self
    }
}

impl Default for Extract<rand_xorshift::XorShiftRng> {
    fn default() -> Self {
        Extract::new(rand_xorshift::XorShiftRng::from_entropy(), 2usize.pow(12))
    }
}