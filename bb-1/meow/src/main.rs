use clap::Parser;
use std::env;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    config: String,
    #[arg(short, long)]
    verbose: bool,
    input: String,
}

fn main() {
    // Command-line arguments
    let args = Args::parse();
    println!("Configuration file: {}", args.config);
    println!("Verbose: {}", args.verbose);
    println!("Input file: {}", args.input);

    // Environment variables
    let home = env::var_os("HOME");
    println!("Home env variable: {:?}", home);
}
