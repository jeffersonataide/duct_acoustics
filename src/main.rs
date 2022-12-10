use clap::Parser;
use std::fs;
use std::io::prelude::*;

mod acoustic_elements;

/// Calculates the acoustic transmission of duct elements
#[derive(Parser, Debug)]
struct Cli {
    /// The low frequency limit
    #[arg(long)]
    lower_frequency: i32,
    /// The high frequency limit
    #[arg(long)]
    high_frequency: i32,

    /// Duct radius
    #[arg(long)]
    duct_radius: f32,
    /// Chamber radius
    #[arg(long)]
    chamber_radius: f32,
    /// Chamber length
    #[arg(long)]
    chamber_length: f32,

    /// Output file
    #[arg(short, long)]
    output: String,
}

fn main() {
    let args = Cli::parse();

    println!(
        "Calculating from {} hz to {} hz",
        args.lower_frequency, args.high_frequency
    );

    let expansion_chamber = acoustic_elements::ExpansionChamber::new(
        args.duct_radius,
        args.chamber_radius,
        args.chamber_length,
    );

    println!("{:?}", expansion_chamber);
    println!(
        "Transmission loss at 0 Hz: {:?}",
        expansion_chamber.transmission_loss(0_f32)
    );

    let mut file = fs::File::create(&args.output).expect("Could not create the output file.");

    for frequency in (args.lower_frequency..args.high_frequency).step_by(5) {
        let csv_line = format!(
            "{}, {};\n",
            frequency,
            expansion_chamber.transmission_loss(frequency as f32)
        )
        .into_bytes();
        file.write(&csv_line)
            .expect("Coult not write to the output file.");
    }
}
