// src/main.rs
/*
 * Main executable for QuantumSpectral
 */

use clap::Parser;
use quantumspectral::{Result, run};

#[derive(Parser)]
#[command(version, about = "QuantumSpectral - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
    
    /// Input file path
    #[arg(short, long)]
    input: Option<String>,
    
    /// Output file path
    #[arg(short, long)]
    output: Option<String>,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose, args.input, args.output)
}
