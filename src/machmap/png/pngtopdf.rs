use std::error::Error;

use crate::machmap::InputTo;
use crate::machreduce::pdf::imagestopdf::ImagesToPdf;

pub struct PngToPdf<'a> {
    pub input_file: &'a str,
    pub output_file: &'a str,
}

impl<'a> PngToPdf<'a> {
    pub fn new(input_file: &'a str, output_file: &'a str) -> PngToPdf<'a> {
        PngToPdf {
            input_file,
            output_file,
        }
    }
}

impl<'a> InputTo<'a> for PngToPdf<'a> {
    fn convert(&self) -> Result<String, Box<dyn Error + 'a>> {
        let img_to_pdf = ImagesToPdf::new();
        img_to_pdf.reduce(
            vec![self.input_file.to_string()],
            self.output_file,
        )
    }
}
