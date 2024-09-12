pub trait Printer: Iterator<Item=String> {
    fn colorful(&mut self, enable: bool) -> &mut Self;
}
