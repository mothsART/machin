use std::error::Error;

use colored::Colorize;
use image::{ImageFormat, ImageReader};
use tempfile::tempdir;

use crate::machmap::InputTo;

pub struct SVGToJPG<'a> {
    pub input_file: &'a str,
    pub output_file: &'a str,
}

impl<'a> SVGToJPG<'a> {
    pub fn new(input_file: &'a str, output_file: &'a str) -> SVGToJPG<'a> {
        SVGToJPG {
            input_file,
            output_file,
        }
    }
}

impl<'a> InputTo<'a> for SVGToJPG<'a> {
    fn convert(&self) -> Result<String, Box<dyn Error + 'a>> {
        let opt = usvg::Options::default();

        let svg_data = std::fs::read(self.input_file)?;
        let tree = usvg::Tree::from_data(&svg_data, &opt)?;
        let pixmap_size = tree.size().to_int_size();
        let mut pixmap = tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height()).unwrap();
        resvg::render(&tree, tiny_skia::Transform::default(), &mut pixmap.as_mut());

        let tmp_dir = tempdir()?;
        let tmp_png = tmp_dir.path().join("tmp.png").display().to_string();
        pixmap.save_png(&tmp_png)?;
        let img = ImageReader::open(&tmp_png)?.decode()?;

        let format = ImageFormat::from_path(self.output_file)?;
        if format == ImageFormat::Jpeg {
            if img.color().has_alpha() {
                colored_warn!(format!(
                    "Warning : file \"{}\" have an alpha channel : is not supported for en jpeg file with 8 bits. The output file will no longer have an alpha channel.",
                    self.input_file
                ));
            }
            img.to_rgb8().save(self.output_file)?;
        } else {
            img.save(self.output_file)?;
        }

        tmp_dir.close()?;

        Ok(format!(
            "convert svg to jpg : {} -> {}",
            self.input_file, self.output_file
        ))
    }
}
