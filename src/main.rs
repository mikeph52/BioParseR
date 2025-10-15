use clap::{Parser, Subcommand};
use anyhow::Result;

#[derive(Parser)]
#[command(name = "BioParseR", about = "A fast Rust parser for BAM, SAM, and VCF files, by mikeph_ 2025.")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Parse a SAM file
    Sam { input: String },
    /// Parse a BAM file
    Bam { input: String },
    /// Parse a VCF file
    Vcf { input: String },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Sam { input } => bioparser::parse_sam(&input)?,
        Commands::Bam { input } => bioparser::parse_bam(&input)?,
        Commands::Vcf { input } => bioparser::parse_vcf(&input)?,
    }

    Ok(())
}
