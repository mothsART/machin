use std::error::Error;
use image::io::Reader as ImageReader;

use crate::mmap::InputTo;

pub struct JPG2PNG<'a> {
    pub input_file: &'a str,
    pub output_file: &'a str
}

impl<'a> JPG2PNG<'a> {
    pub fn new(input_file: &'a str, output_file: &'a str) -> JPG2PNG<'a> {
        JPG2PNG {
            input_file,
            output_file
        }
    }
}

impl<'a> InputTo<'a> for JPG2PNG<'a> {
    fn convert(&self) -> Result<String, Box<dyn Error + 'a>> {
        let img = ImageReader::open(&self.input_file)?.decode()?;
        img.save(&self.output_file)?;
        Ok(format!(
            "convert jpg to png : {} -> {}",
            self.input_file,
            self.output_file
        ))
    }
}
