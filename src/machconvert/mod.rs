use std::error::Error;

use crate::errors::InputFileUnsupportedError;

pub mod errors;
pub mod image_input;

use crate::machconvert::errors::ArgConvertError;
use crate::machconvert::image_input::InputTo;
use image_input::ImageInputFile;

pub struct InputsFiles<'a> {
    pub input_file: &'a str,
    pub output_file: &'a str,
    pub image_mime_type: Vec<&'a str>,
}

impl<'a> InputsFiles<'a> {
    pub fn new(input_file: &'a str, output_file: &'a str) -> InputsFiles<'a> {
        InputsFiles {
            input_file,
            output_file,
            image_mime_type: vec!["image/png", "image/jpeg"],
        }
    }

    pub fn convert(&self, rotate_value: Option<&str>) -> Result<String, Box<dyn Error + 'a>> {
        let input_mime = mime_guess::from_path(self.input_file);

        let input_e = InputFileUnsupportedError {
            input_file: self.input_file,
        };

        let arg_e = ArgConvertError {};

        if let Some(r) = rotate_value {
            match &input_mime.first_raw() {
                Some(i_mime) => {
                    if self.image_mime_type.contains(i_mime) {
                        return self.image_convert(r);
                    }
                    return Err(Box::new(input_e));
                }
                None => {
                    return Err(Box::new(input_e));
                }
            };
        }
        Err(Box::new(arg_e))
    }

    pub fn image_convert(&self, rotate_value: &str) -> Result<String, Box<dyn Error + 'a>> {
        let image = ImageInputFile::new(self.input_file, self.output_file);
        image.convert(rotate_value)
    }
}
