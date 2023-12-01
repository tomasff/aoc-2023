use std::fs;
mod day_one;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    day: u8,

    #[arg(short, long)]
    input: String,
}

fn main() {
    let args = Args::parse();

    let contents = fs::read_to_string(args.input)
        .expect("Failed to find the input file in the path provided.");

    let parts = match args.day {
        1 => day_one::solve(&contents),
        _ => (None, None),
    };

    println!("Day {}: {:?}", args.day, parts);
}
