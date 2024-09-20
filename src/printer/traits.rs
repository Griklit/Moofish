pub trait ShellOutput: Iterator<Item=String> {
    fn colorful(&mut self, enable: bool) -> &mut Self;
}
