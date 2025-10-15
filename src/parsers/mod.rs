pub mod sam;
pub mod bam;
pub mod vcf;

pub use sam::parse_sam;
pub use bam::parse_bam;
pub use vcf::parse_vcf;
