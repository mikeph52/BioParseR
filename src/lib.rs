use anyhow::Result;
use noodles::{sam, bam, vcf};

/// Parse a SAM file and print basic info
pub fn parse_sam(path: &str) -> Result<()> {
    let mut reader = sam::Reader::from_path(path)?;
    let header = reader.read_header()?;
    println!("Reference sequences: {:?}", header.reference_sequences().len());

    for (i, record) in reader.records().enumerate().take(5) {
        let record = record?;
        println!("Read {}: {:?}", i + 1, record.name());
    }
    Ok(())
}

/// Parse a BAM file and print basic info
pub fn parse_bam(path: &str) -> Result<()> {
    let mut reader = bam::Reader::from_path(path)?;
    let header = reader.read_header()?;
    println!("Reference sequences: {:?}", header.reference_sequences().len());

    for (i, result) in reader.records().enumerate().take(5) {
        let record = result?;
        println!("Read {}: {:?}", i + 1, record.name());
    }
    Ok(())
}

/// Parse a VCF file and print variant data
pub fn parse_vcf(path: &str) -> Result<()> {
    let mut reader = vcf::Reader::from_path(path)?;
    let header = reader.read_header()?;
    println!("Header: {:?}", header.file_format());

    for (i, result) in reader.records().enumerate().take(5) {
        let record = result?;
        println!("Variant {}: {}:{} {:?}->{:?}",
                 i + 1,
                 record.chromosome(),
                 record.position(),
                 record.reference_bases(),
                 record.alternate_bases());
    }
    Ok(())
}
