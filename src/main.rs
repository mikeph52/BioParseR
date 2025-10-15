use clap::{Parser, Subcommand};
use anyhow::Result;
use bioparser::{parse_bam, parse_sam, parse_vcf};

#[derive(Parser)]
#[command(name = "BioParseR", about = "A fast Rust parser for BAM, SAM, and VCF files.")]
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
    let cli = Cli::parse();

    match cli.command {
        Commands::Sam { input } => parse_sam(&input)?,
        Commands::Bam { input } => parse_bam(&input)?,
        Commands::Vcf { input } => parse_vcf(&input)?,
    }

    Ok(())
}
