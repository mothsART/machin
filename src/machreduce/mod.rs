use std::error::Error;

pub mod errors;
pub mod image_reduce;
pub mod pdf;
pub mod pdf_reduce;
pub mod zip_reduce;

use crate::machreduce::errors::OutputFileUnsupportedError;
use crate::machreduce::image_reduce::ImageOutputFile;
use crate::machreduce::pdf_reduce::PdfOutputFile;
use crate::machreduce::zip_reduce::ZipOutputFile;

#[derive(Eq, PartialEq)]
pub enum Direction {
    Vertical,
    Horizontal,
}

pub trait InputTo<'a> {
    fn reduce(&self, direction: &Direction) -> Result<String, Box<dyn Error + 'a>>;
}

pub struct InputsFiles<'a> {
    pub input_lines: &'a Vec<String>,
    pub output_file: &'a str,
    pub direction: Direction,
}

impl<'a> InputsFiles<'a> {
    pub fn new(
        input_lines: &'a Vec<String>,
        output_file: &'a str,
        direction: Direction,
    ) -> InputsFiles<'a> {
        InputsFiles {
            input_lines,
            output_file,
            direction,
        }
    }

    pub fn reduce(&mut self) -> Result<String, Box<dyn Error + 'a>> {
        let output_mime = mime_guess::from_path(self.output_file);

        let output_e = OutputFileUnsupportedError {
            output_file: self.output_file,
        };

        let image_output = ImageOutputFile::new(self.input_lines, self.output_file);
        let pdf_output = PdfOutputFile::new(self.input_lines, self.output_file);
        let zip_output = ZipOutputFile::new(self.input_lines, self.output_file);

        match &output_mime.first_raw() {
            Some(o_mime) => {
                if image_output.output_mime_type.contains(o_mime) {
                    return image_output.reduce(&self.direction);
                }
                if pdf_output.output_mime_type == *o_mime {
                    return pdf_output.reduce(&self.direction);
                }
                if zip_output.output_mime_type.contains(o_mime) {
                    return zip_output.reduce(&self.direction);
                }
                Err(Box::new(output_e))
            }
            None => Err(Box::new(output_e)),
        }
    }
}
