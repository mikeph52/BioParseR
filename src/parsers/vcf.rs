use anyhow::Result;
use noodles_vcf as vcf;
use std::{fs::File, io::BufReader};

pub fn parse_vcf(path: &str) -> Result<()> {
    let file = File::open(path)?;
    let buf_reader = BufReader::new(file);
    let mut reader = vcf::Reader::new(buf_reader);

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
