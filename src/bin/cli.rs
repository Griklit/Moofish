use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Colorize the output
    #[arg(short, long)]
    color: Option<bool>,
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args.color)
}