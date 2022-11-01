use image::imageops;
use image::io::Reader as ImageReader;
use image::DynamicImage::{ImageLuma8, ImageRgba8};
use std::error::Error;

use colored::Colorize;

use crate::machconvert::errors::ArgConvertError;
use crate::machconvert::{ConvertArgs, ConvertFlip};

impl<'a> ImageInputFile<'a> {
    pub fn new(input_file: &'a str, output_file: &'a str) -> ImageInputFile<'a> {
        ImageInputFile {
            input_file,
            output_file,
        }
    }
}

impl<'a> InputTo<'a> for ImageInputFile<'a> {
    fn convert(&self, args: &ConvertArgs) -> Result<String, Box<dyn Error + 'a>> {
        let arg_e = ArgConvertError {};

        if None == args.color && None == args.flip && None == args.rotate {
            return Err(Box::new(arg_e));
        }

        let mut step = 1;
        let mut img = ImageReader::open(&self.input_file)?.decode()?;

        if args.color.is_some() {
            img = ImageLuma8(imageops::grayscale(&img));
            colored_success!(format!(
                "Step {} : apply a grayscale of {} to {}",
                step, self.input_file, self.output_file,
            ));
            step += 1;
        }

        if let Some(flip_value) = &args.flip {
            match flip_value {
                ConvertFlip::Horizontal => {
                    img = ImageRgba8(imageops::flip_horizontal(&img));
                    colored_success!(format!(
                        "Step {} : apply an horizontal flip of {} to {}",
                        step, self.input_file, self.output_file,
                    ));
                }
                ConvertFlip::Vertical => {
                    img = ImageRgba8(imageops::flip_vertical(&img));
                    colored_success!(format!(
                        "Step {} : apply a vertical flip of {} to {}",
                        step, self.input_file, self.output_file,
                    ));
                }
            }
            step += 1;
        }

        if let Some(r) = args.rotate {
            if r == 90 {
                img = ImageRgba8(imageops::rotate90(&img));
            } else if r == 180 {
                img = ImageRgba8(imageops::rotate180(&img));
            } else {
                img = ImageRgba8(imageops::rotate270(&img));
            }
            colored_success!(format!(
                "Step {} : apply a {} degree rotation of {} to {}",
                step, r, self.input_file, self.output_file,
            ));
        }

        img.save(self.output_file)?;
        Ok(String::new())
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
    fn convert(&self, args: &ConvertArgs) -> Result<String, Box<dyn Error + 'a>>;
}

create_convert_input!(ImageInputFile);
