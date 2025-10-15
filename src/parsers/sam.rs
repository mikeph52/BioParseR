use anyhow::Result;
use noodles_sam as sam;
use std::{fs::File, io::BufReader};

pub fn parse_sam(path: &str) -> Result<()> {
    let file = File::open(path)?;
    let buf_reader = BufReader::new(file);
    let mut reader = sam::Reader::new(buf_reader);

    let header = reader.read_header()?;
    println!(
        "SAM Header: {} reference sequences",
        header.reference_sequences().len()
    );

    for (i, result) in reader.records(&header).enumerate().take(5) {
        let record = result?;
        println!(
            "Read {}: {:?}, Position: {:?}",
            i + 1,
            record.read_name(),
            record.alignment_start()
        );
    }

    Ok(())
}
