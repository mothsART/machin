use std::error::Error;

use crate::machmap::InputTo;
use crate::machreduce::pdf::imagestopdf::ImagesToPdf;

pub struct JpgToPdf<'a> {
    pub input_file: &'a str,
    pub output_file: &'a str,
}

impl<'a> JpgToPdf<'a> {
    pub fn new(input_file: &'a str, output_file: &'a str) -> JpgToPdf<'a> {
        JpgToPdf {
            input_file,
            output_file,
        }
    }
}

impl<'a> InputTo<'a> for JpgToPdf<'a> {
    fn convert(&self) -> Result<String, Box<dyn Error + 'a>> {
        let img_to_pdf = ImagesToPdf::new();
        img_to_pdf.reduce(vec![self.input_file.to_string()], self.output_file)
    }
}
