use image::imageops;
use image::io::Reader as ImageReader;
use std::error::Error;

use crate::machconvert::errors::UnsupportedRotateError;

impl<'a> ImageInputFile<'a> {
    pub fn new(input_file: &'a str, output_file: &'a str) -> ImageInputFile<'a> {
        ImageInputFile {
            input_file,
            output_file,
        }
    }
}

impl<'a> InputTo<'a> for ImageInputFile<'a> {
    fn convert(&self, rotate_value: &str) -> Result<String, Box<dyn Error + 'a>> {
        let e = UnsupportedRotateError {};
        match rotate_value.parse::<i16>() {
            Ok(r) => {
                if r == 90 || r == 180 || r == 270 {
                    let img = ImageReader::open(&self.input_file)?.decode()?;
                    let new_img;
                    if r == 90 {
                        new_img = imageops::rotate90(&img);
                    } else if r == 180 {
                        new_img = imageops::rotate180(&img);
                    } else {
                        new_img = imageops::rotate270(&img);
                    }
                    new_img.save(self.output_file)?;
                    Ok(format!(
                        "apply a {} degree rotation of {} to {}",
                        rotate_value, self.input_file, self.output_file,
                    ))
                } else {
                    Err(Box::new(e))
                }
            }
            Err(_e) => Err(Box::new(e)),
        }
    }
}

#[macro_export]
macro_rules! create_convert_input {
    ($struct_name:ident) => {
        pub struct $struct_name<'a> {
            input_file: &'a str,
            output_file: &'a str,
        }
    };
}

pub trait InputTo<'a> {
    fn convert(&self, rotate_value: &str) -> Result<String, Box<dyn Error + 'a>>;
}

create_convert_input!(ImageInputFile);
