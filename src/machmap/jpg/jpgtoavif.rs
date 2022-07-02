use image::io::Reader as ImageReader;
use std::error::Error;

use crate::machmap::InputTo;

pub struct JPGTOAVIF<'a> {
    pub input_file: &'a str,
    pub output_file: &'a str,
}

impl<'a> JPGTOAVIF<'a> {
    pub fn new(input_file: &'a str, output_file: &'a str) -> JPGTOAVIF<'a> {
        JPGTOAVIF {
            input_file,
            output_file,
        }
    }
}

impl<'a> InputTo<'a> for JPGTOAVIF<'a> {
    fn convert(&self) -> Result<String, Box<dyn Error + 'a>> {
        let img = ImageReader::open(&self.input_file)?.decode()?;
        img.save(&self.output_file)?;
        Ok(format!(
            "convert jpg to avif : {} -> {}",
            self.input_file, self.output_file
        ))
    }
}
