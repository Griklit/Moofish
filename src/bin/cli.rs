use std::collections::HashSet;
use std::time::Duration;

use clap::Parser;

use moofish::{ShellAdapter, ShellOutput};

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
    match unit {
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
    colorful: bool,

    /// Runtime
    #[arg(short = 't', long, value_parser = runtime_parser, default_value = "5s")]
    runtime: Duration,

    /// The printer to use [cargo | tar]
    #[arg(required = true)]
    adapters: Vec<ShellAdapter>,
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args);
    let Args { colorful, runtime, mut adapters } = args;
    let _ = adapters.iter_mut().map(|adapter| adapter.colorful(colorful));
    let adapters: HashSet<ShellAdapter> = adapters.into_iter().collect();
    let start_time = std::time::Instant::now();
    while std::time::Instant::now().duration_since(start_time) < runtime {
        let line = adapters[0].next();
        match line {
            Some(line) => { println!("{}", line); }
            None => { break; }
        }
    }
}