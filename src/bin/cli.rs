use std::time::Duration;

use clap::Parser;

use moofish::PrinterTypes;

fn runtime_parser(s: &str) -> Result<Duration, String> {
    let chars = s.chars().collect::<Vec<char>>();
    let last = chars[chars.len() - 1];
    let number;
    let unit;
    if last.is_digit(10) {
        number = s.parse::<u64>().map_err(|e| e.to_string())?;
        unit = 's';
    } else {
        number = chars[0..chars.len() - 1].into_iter().collect::<String>().parse::<u64>().map_err(|e| e.to_string())?;
        unit = last;
    }
    match last {
        's' => Ok(Duration::from_secs(number)),
        'm' => Ok(Duration::from_secs(number * 60)),
        'h' => Ok(Duration::from_secs(number * 60 * 60)),
        'd' => Ok(Duration::from_secs(number * 60 * 60 * 24)),
        _ => Err("Invalid unit, must be 's', 'm', 'h' or 'd'".to_string())
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
    #[arg(short = 't', long, value_parser = runtime_parser, default_value = "5s")]
    runtime: Duration,

    /// The printer to use
    #[arg(required = true)]
    printers: Vec<PrinterTypes>,
}

fn main() {
    let mut args = Args::parse();
    println!("{:?}", args);
    let start_time = std::time::Instant::now();
    while std::time::Instant::now().duration_since(start_time) < args.runtime {
        let line = args.printers[0].next();
        match line {
            Some(line) => { println!("{}", line); }
            None => { break; }
        }
    }
}