use image::io::Reader as ImageReader;
use std::error::Error;

use crate::machmap::InputTo;

pub struct PNGToJPG<'a> {
    pub input_file: &'a str,
    pub output_file: &'a str,
}

impl<'a> PNGToJPG<'a> {
    pub fn new(input_file: &'a str, output_file: &'a str) -> PNGToJPG<'a> {
        PNGToJPG {
            input_file,
            output_file,
        }
    }
}

impl<'a> InputTo<'a> for PNGToJPG<'a> {
    fn convert(&self) -> Result<String, Box<dyn Error + 'a>> {
        let img = ImageReader::open(&self.input_file)?.decode()?;
        //println!("dimensions {:?}", img.dimensions());
        println!("{:?}", img.color());
        img.save(&self.output_file)?;
        Ok(format!(
            "convert png to jpg : {} -> {}",
            self.input_file, self.output_file
        ))
    }
}
