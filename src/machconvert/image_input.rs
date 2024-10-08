use image::DynamicImage::{ImageLuma8, ImageRgba8};
use image::{imageops, ImageFormat, ImageReader};
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

        if args.color.is_none() && args.flip.is_none() && args.rotate.is_none() {
            return Err(Box::new(arg_e));
        }

        let mut step = 1;
        let mut img = ImageReader::open(self.input_file)?.decode()?;
        let has_alpha = img.color().has_alpha();

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

        let format = ImageFormat::from_path(self.output_file)?;
        if format == ImageFormat::Jpeg {
            if has_alpha {
                colored_warn!(format!(
                    "Warning : input file \"{}\" have an alpha channel : is not supported for en jpeg file. The output file \"{}\" will no longer have an alpha channel.",
                    self.input_file,
                    self.output_file,
                ));
            }
            img.to_rgb8().save(self.output_file)?;
        } else {
            img.save(self.output_file)?;
        }
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
