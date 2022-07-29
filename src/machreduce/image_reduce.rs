use std::error::Error;
use std::fmt;
use std::path::Path;

use colored::*;
use std::io::BufRead;
use image::io::Reader as ImageReader;
use image::{image_dimensions, GenericImage, ImageBuffer, Rgba};

use crate::machreduce::{Direction, InputTo};

pub struct ImageOutputFile<'a> {
    pub output_file: &'a str,
    pub input_mime_type: Vec<&'a str>,
    pub output_mime_type: Vec<&'a str>,
}

impl<'a> ImageOutputFile<'a> {
    pub fn new(output_file: &'a str) -> ImageOutputFile<'a> {
        ImageOutputFile {
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
        let lines = std::io::stdin().lock().lines();
        let mut x_size: u32 = 0;
        let mut y_size: u32 = 0;
        let mut _files = Vec::new();

        for line in lines {
            match line {
                Ok(_l) => {
                    if !Path::new(&_l).exists() {
                        colored_err!(format!("Input file \"{}\" doesn't exist", _l));
                        continue;
                    }
                    if let Some(o_mime) = mime_guess::from_path(&_l).first_raw() {
                        if !self.input_mime_type.contains(&o_mime) {
                            colored_err!(format!("Input file \"{}\" not supported", _l));
                            continue;
                        }
                        if let Ok(dimensions) = image_dimensions(&_l) {
                            if *direction == Direction::Horizontal {
                                _files.push(ImagePath {
                                    pos: dimensions.0,
                                    path: _l,
                                });
                                if dimensions.1 >= y_size {
                                    y_size = dimensions.1;
                                }
                                x_size += dimensions.0;
                            } else {
                                _files.push(ImagePath {
                                    pos: dimensions.1,
                                    path: _l,
                                });
                                if dimensions.0 >= x_size {
                                    x_size = dimensions.0;
                                }
                                y_size += dimensions.1;
                            }
                        }
                    }
                }
                Err(_) => {
                    continue;
                }
            }
        }
        if _files.is_empty() {
            return Err(Box::new(NoPicturesFoundError {}));
        }

        let mut img_buf = <ImageBuffer<Rgba<u8>, _>>::new(x_size, y_size);
        let mut before_pos = 0;
        for _file in _files.iter() {
            let new_img = ImageReader::open(&_file.path)?.decode()?;
            if *direction == Direction::Horizontal {
                if let Err(_e) = img_buf.copy_from(&new_img, before_pos, 0) {
                    continue;
                }
            } else if let Err(_e) = img_buf.copy_from(&new_img, 0, before_pos) {
                continue;
            }
            before_pos += _file.pos;
        }
        img_buf.save(self.output_file)?;
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
