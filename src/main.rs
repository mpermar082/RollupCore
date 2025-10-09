// src/main.rs
/*
 * Main executable for RollupCore
 */

use clap::Parser;
use rollupcore::{Result, run};

/// CLI arguments for RollupCore
#[derive(Parser)]
#[command(version, about = "RollupCore - A Rust implementation")]
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
    // Parse CLI arguments
    let args = Cli::parse();

    // Run RollupCore with parsed arguments
    run(args.verbose, args.input, args.output)
}