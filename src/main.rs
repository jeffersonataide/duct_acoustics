use clap::Parser;

/// Calculates the acoustic transmission of duct elements
#[derive(Parser, Debug)]
struct Cli {
    /// The low frequency limit
    #[arg(long)]
    lower_frequency: i32,
    /// The high frequency limit
    #[arg(long)]
    high_frequency: i32,
}

fn main() {
    println!("Hello, world!");
    println!("Sin: {}", (std::f32::consts::PI / 2f32).sin());
    let args = Cli::parse();

    println!(
        "Calculating from {} hz to {} hz",
        args.lower_frequency, args.high_frequency
    )
}
