use clap::{Parser, Subcommand};
use anyhow::Result;
use bioparser::{parse_bam, parse_sam, parse_vcf};

#[derive(Parser)]
#[command(name = "BioParseR Pre-Release by mikeph_ 2025", about = "A fast parser for BAM, SAM, and VCF files, written in Rust.")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Sam { input: String },
    Bam { input: String },
    Vcf { input: String },
}

fn main() -> Result<()> {
    println!("BioParseR Pre-Release by mikeph_ 2025");
    println!("-------------------------------------");
    let cli = Cli::parse();

    match cli.command {
        Commands::Sam { input } => parse_sam(&input)?,
        Commands::Bam { input } => parse_bam(&input)?,
        Commands::Vcf { input } => parse_vcf(&input)?,
    }

    Ok(())
}
