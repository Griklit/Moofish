use std::io::{self, Read};

fn main() {
    let mut buffer = [0; 1];
    let mut stdin = io::stdin();

    while let Ok(bytes_read) = stdin.read(&mut buffer) {
        if bytes_read == 0 {
            break;
        }
        // 打印十六进制值
        if buffer == [0x0a] {
            println!();
        } else {
            print!("{:02X} ", buffer[0]);
        }
    }
}
