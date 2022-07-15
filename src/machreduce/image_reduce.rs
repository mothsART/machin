use std::error::Error;
use std::path::Path;

use colored::*;
use image::io::Reader as ImageReader;
use image::{image_dimensions, GenericImage, ImageBuffer, Rgba};

use crate::machreduce::ImageOutputFile;
use crate::machreduce::InputTo;

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
    fn reduce(&self) -> Result<String, Box<dyn Error + 'a>> {
        let lines = std::io::stdin().lines();
        let mut x_size: u32 = 0;
        let mut y_size: u32 = 0;
        let mut _files = Vec::new();

        for line in lines {
            match line {
                Ok(_l) => {
                    if !Path::new(&_l).exists() {
                        eprintln!(
                            "{}",
                            format!("Input file \"{}\" doesn't exist", _l)
                                .black()
                                .on_red()
                        );
                        continue;
                    }
                    if let Ok(dimensions) = image_dimensions(&_l) {
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
                Err(_) => {
                    continue;
                }
            }
        }

        let mut img_buf = <ImageBuffer<Rgba<u8>, _>>::new(x_size, y_size);
        let mut before_pos = 0;
        for _file in _files.iter() {
            let new_img = ImageReader::open(&_file.path)?.decode()?;
            if let Err(_e) = img_buf.copy_from(&new_img, 0, before_pos) {
                continue;
            }
            before_pos = _file.pos;
        }
        img_buf.save(self.output_file)?;
        Ok(format!("images reduce to {}", self.output_file))
    }
}
