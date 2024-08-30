use std::error::Error;
use std::fmt;
use std::path::Path;

use colored::*;
use image::{image_dimensions, GenericImage};
use image::{DynamicImage, ImageFormat, ImageReader, RgbaImage};

use crate::machreduce::{Direction, InputTo};

pub struct ImageOutputFile<'a> {
    pub input_lines: &'a Vec<String>,
    pub output_file: &'a str,
    pub input_mime_type: Vec<&'a str>,
    pub output_mime_type: Vec<&'a str>,
}

impl<'a> ImageOutputFile<'a> {
    pub fn new(input_lines: &'a Vec<String>, output_file: &'a str) -> ImageOutputFile<'a> {
        ImageOutputFile {
            input_lines,
            output_file,
            input_mime_type: vec!["image/png", "image/jpeg"],
            output_mime_type: vec!["image/png", "image/jpeg"],
        }
    }
}

struct ImagePath {
    pos: u32,
    path: String,
}

impl<'a> InputTo<'a> for ImageOutputFile<'a> {
    fn reduce(&self, direction: &Direction) -> Result<String, Box<dyn Error + 'a>> {
        let mut x_size: u32 = 0;
        let mut y_size: u32 = 0;
        let mut _files = Vec::new();

        for line in self.input_lines {
            if !Path::new(&line).exists() {
                colored_err!(format!("Input file \"{line}\" doesn't exist"));
                continue;
            }
            if let Some(o_mime) = mime_guess::from_path(line).first_raw() {
                if !self.input_mime_type.contains(&o_mime) {
                    colored_err!(format!("Input file \"{line}\" not supported"));
                    continue;
                }
                if let Ok(dimensions) = image_dimensions(line) {
                    if *direction == Direction::Horizontal {
                        _files.push(ImagePath {
                            pos: dimensions.0,
                            path: line.to_string(),
                        });
                        if dimensions.1 >= y_size {
                            y_size = dimensions.1;
                        }
                        x_size += dimensions.0;
                    } else {
                        _files.push(ImagePath {
                            pos: dimensions.1,
                            path: line.to_string(),
                        });
                        if dimensions.0 >= x_size {
                            x_size = dimensions.0;
                        }
                        y_size += dimensions.1;
                    }
                }
            }
        }
        if _files.is_empty() {
            return Err(Box::new(NoPicturesFoundError {}));
        }

        let rgba = RgbaImage::new(x_size, y_size);
        let mut d_img = DynamicImage::ImageRgba8(rgba);
        let mut before_pos = 0;
        let mut has_alpha = false;
        for _file in _files.iter() {
            let new_img = ImageReader::open(&_file.path)?.decode()?;
            if new_img.color().has_alpha() {
                has_alpha = true;
            }
            if *direction == Direction::Horizontal {
                if let Err(_e) = d_img.copy_from(&new_img, before_pos, 0) {
                    continue;
                }
            } else if let Err(_e) = d_img.copy_from(&new_img, 0, before_pos) {
                continue;
            }
            before_pos += _file.pos;
        }
        let format = ImageFormat::from_path(self.output_file)?;
        if format == ImageFormat::Jpeg {
            if has_alpha {
                colored_warn!(format!(
                    "Warning : some input files have an alpha channel : is not supported for en jpeg file. The output file \"{}\" will no longer have an alpha channel.",
                    self.output_file,
                ));
            }
            d_img.to_rgb8().save(self.output_file)?;
        } else {
            d_img.save(self.output_file)?;
        }

        Ok(format!("images reduce to {}", self.output_file))
    }
}

#[derive(Debug)]
pub struct NoPicturesFoundError {}

impl fmt::Display for NoPicturesFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "No input pictures found",)
    }
}

impl Error for NoPicturesFoundError {}
