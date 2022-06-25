use image::io::Reader as ImageReader;
use std::error::Error;

use crate::mmap::InputTo;

pub struct PNGToAVIF<'a> {
    pub input_file: &'a str,
    pub output_file: &'a str,
}

impl<'a> PNGToAVIF<'a> {
    pub fn new(input_file: &'a str, output_file: &'a str) -> PNGToAVIF<'a> {
        PNGToAVIF {
            input_file,
            output_file,
        }
    }
}

impl<'a> InputTo<'a> for PNGToAVIF<'a> {
    fn convert(&self) -> Result<String, Box<dyn Error + 'a>> {
        let img = ImageReader::open(&self.input_file)?.decode()?;
        println!("{:?}", img.color());
        img.save(&self.output_file)?;
        Ok(format!(
            "convert png to avif : {} -> {}",
            self.input_file, self.output_file
        ))
    }
}
