use anyhow::Result;
use noodles::vcf::{self, reader::Reader};
use std::fs::File;

pub fn parse_vcf(path: &str) -> Result<()> {
    let file = File::open(path)?;
    let mut reader = Reader::new(file);

    let header = reader.read_header()?;
    println!("VCF Header: {:?}", header.file_format());

    for (i, result) in reader.records(&header).enumerate().take(5) {
        let record = result?;
        println!(
            "Variant {}: {}:{} {:?}->{:?}",
            i + 1,
            record.chromosome(),
            record.position(),
            record.reference_bases(),
            record.alternate_bases()
        );
    }

    Ok(())
}
