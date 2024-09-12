use std::error::Error;

use image::ImageReader;
use tempfile::tempdir;

use crate::machmap::InputTo;

pub struct SVGToAVIF<'a> {
    pub input_file: &'a str,
    pub output_file: &'a str,
}

impl<'a> SVGToAVIF<'a> {
    pub fn new(input_file: &'a str, output_file: &'a str) -> SVGToAVIF<'a> {
        SVGToAVIF {
            input_file,
            output_file,
        }
    }
}

impl<'a> InputTo<'a> for SVGToAVIF<'a> {
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

        img.save(self.output_file)?;

        tmp_dir.close()?;

        Ok(format!(
            "convert svg to avif : {} -> {}",
            self.input_file, self.output_file
        ))
    }
}
