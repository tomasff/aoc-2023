use std::fs;

mod day_one;
mod day_six;
mod day_two;

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
        2 => day_two::solve(&contents),
        6 => day_six::solve(&contents),
        _ => (None, None),
    };

    println!("Day {}: {:?}", args.day, parts);
}
