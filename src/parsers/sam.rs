use anyhow::Result;
use noodles::sam::{self, reader::Reader};
use std::fs::File;

pub fn parse_sam(path: &str) -> Result<()> {
    let file = File::open(path)?;
    let mut reader = Reader::new(file);

    let header = reader.read_header()?;
    println!("SAM Header: {} reference sequences", header.reference_sequences().len());

    for (i, result) in reader.records(&header).enumerate().take(5) {
        let record = result?;
        println!(
            "Read {}: {:?}, Position: {:?}",
            i + 1,
            record.name(),
            record.alignment_start()
        );
    }

    Ok(())
}
