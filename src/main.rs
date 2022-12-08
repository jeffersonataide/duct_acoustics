use clap::Parser;
use std::fs;
use std::io::prelude::*;

/// Calculates the acoustic transmission of duct elements
#[derive(Parser, Debug)]
struct Cli {
    /// The low frequency limit
    #[arg(long)]
    lower_frequency: i32,
    /// The high frequency limit
    #[arg(long)]
    high_frequency: i32,
    /// Output file
    #[arg(short, long)]
    output: String,
}

fn main() {
    println!("Sin: {}", (std::f32::consts::PI / 2f32).sin());
    let args = Cli::parse();

    println!("Args: {:?}", args);

    let mut file = fs::File::create(&args.output).expect("Could not create the output file.");

    for frequency in (args.lower_frequency..args.high_frequency).step_by(5) {
        let csv_line = format!("{}, placeholder_value;\n", frequency).into_bytes();
        file.write(&csv_line)
            .expect("Coult not write to the output file.");
    }

    println!(
        "Calculating from {} hz to {} hz",
        args.lower_frequency, args.high_frequency
    )
}
