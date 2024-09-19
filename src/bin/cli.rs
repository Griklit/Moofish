use std::time::Duration;

use clap::Parser;

use moofish::PrinterTypes;

fn runtime_parser(s: &str) -> Result<Duration, String> {
    let chars = s.chars().collect::<Vec<char>>();
    let number = chars[0..chars.len() - 1].into_iter().collect::<String>();
    let number = number.parse::<u64>().map_err(|e| e.to_string())?;
    let unit = chars[chars.len() - 1];
    match unit {
        's' => Ok(Duration::from_secs(number)),
        'm' => Ok(Duration::from_secs(number * 60)),
        'h' => Ok(Duration::from_secs(number * 60 * 60)),
        'd' => Ok(Duration::from_secs(number * 60 * 60 * 24)),
        _ => Err("Invalid or missing unit".to_string())
    }
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Colorize the output
    #[arg(short, long)]
    color: Option<bool>,

    /// Runtime, e.g. 5s, 10m, 1h, 2d
    #[arg(short = 't', long, value_parser = runtime_parser)]
    runtime: Duration,

    /// The printer to use
    #[arg()]
    printer: Vec<PrinterTypes>,
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args);
    // let start_time = std::time::Instant::now();
    // while std::time::Instant::now().duration_since(start_time) < args.runtime {
    //     println!("Hello, world!");
    //     std::thread::sleep(Duration::from_secs(1));
    // }
}