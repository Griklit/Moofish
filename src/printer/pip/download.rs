use crate::ShellOutput;

pub struct Download {
    pub colorful: bool,
}

impl Iterator for Download {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

impl ShellOutput for Download {
    fn colorful(&mut self, enable: bool) -> &mut Self {
        self.colorful = enable;
        self
    }
}