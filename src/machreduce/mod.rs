use std::error::Error;
use std::path::Path;

use colored::*;
use image::io::Reader as ImageReader;
use image::{image_dimensions, GenericImage, ImageBuffer, Rgba};

impl<'a> ImageInputFile<'a> {
    pub fn new(output_file: &'a str) -> ImageInputFile<'a> {
        ImageInputFile { output_file }
    }
}

struct ImagePath {
    pos: u32,
    path: String,
}

impl<'a> InputTo<'a> for ImageInputFile<'a> {
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
        Ok(format!("images reduce to {}", self.output_file,))
    }
}

#[macro_export]
macro_rules! create_reduce_input {
    ($struct_name:ident) => {
        pub struct $struct_name<'a> {
            output_file: &'a str,
        }
    };
}

pub trait InputTo<'a> {
    fn reduce(&self) -> Result<String, Box<dyn Error + 'a>>;
}

create_reduce_input!(ImageInputFile);

pub struct InputsFiles<'a> {
    pub output_file: &'a str,
    pub image_mime_type: Vec<&'a str>,
}

impl<'a> InputsFiles<'a> {
    pub fn new(output_file: &'a str) -> InputsFiles<'a> {
        InputsFiles {
            output_file,
            image_mime_type: vec!["image/png", "image/jpeg"],
        }
    }

    pub fn reduce(&mut self) -> Result<String, Box<dyn Error + 'a>> {
        let input = ImageInputFile::new(self.output_file);
        input.reduce()
    }
}
