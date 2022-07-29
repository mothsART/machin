use std::error::Error;
use std::io::BufRead;
use std::path::Path;

use colored::Colorize;

use crate::machreduce::errors::InputFilesToPdfUnsupportedError;
use crate::machreduce::pdf::imagestopdf::ImagesToPdf;
use crate::machreduce::pdf::pdftopdf::PdfToPdf;
use crate::machreduce::{Direction, InputTo};

pub struct PdfOutputFile<'a> {
    pub output_file: &'a str,
    pub output_mime_type: &'a str,
}

impl<'a> PdfOutputFile<'a> {
    pub fn new(output_file: &'a str) -> PdfOutputFile<'a> {
        PdfOutputFile {
            output_file,
            output_mime_type: "application/pdf",
        }
    }
}

impl<'a> InputTo<'a> for PdfOutputFile<'a> {
    fn reduce(&self, _direction: &Direction) -> Result<String, Box<dyn Error + 'a>> {
        let lines = std::io::stdin().lock().lines();
        let mut _files = Vec::new();
        let mut only_img = true;
        let mut only_pdf = true;

        let input_e = InputFilesToPdfUnsupportedError {};

        let image_to_pdf = ImagesToPdf::new();
        let pdf_to_pdf = PdfToPdf::new();

        for line in lines {
            match line {
                Ok(_l) => {
                    if !Path::new(&_l).exists() {
                        colored_err!(format!("Input file \"{}\" doesn't exist", _l));
                        continue;
                    }
                    if let Some(o_mime) = mime_guess::from_path(&_l).first_raw() {
                        if !image_to_pdf.input_mime_type.contains(&o_mime) {
                            only_img = false;
                        }
                        if pdf_to_pdf.input_mime_type != o_mime {
                            only_pdf = false;
                        }
                        _files.push(_l);
                    }
                }
                Err(_) => {
                    continue;
                }
            }
        }

        if only_img {
            return image_to_pdf.reduce(_files, self.output_file);
        }
        if only_pdf {
            return pdf_to_pdf.reduce(_files, self.output_file);
        }
        Err(Box::new(input_e))
    }
}
