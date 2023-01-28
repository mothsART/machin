use std::error::Error;

use crate::machmap::InputTo;

pub struct JpgToOdt<'a> {
    pub input_file: &'a str,
    pub output_file: &'a str,
}

impl<'a> JpgToOdt<'a> {
    pub fn new(input_file: &'a str, output_file: &'a str) -> JpgToOdt<'a> {
        JpgToOdt {
            input_file,
            output_file,
        }
    }
}

impl<'a> InputTo<'a> for JpgToOdt<'a> {
    fn convert(&self) -> Result<String, Box<dyn Error + 'a>> {
        use std::fs::File;
        use std::io::{Read, Write};
        use std::path::Path;

        use image::image_dimensions;
        use tera::{Context, Tera};

        let path = std::path::Path::new(&self.output_file);
        let file = std::fs::File::create(path).unwrap();
        let mut zip = zip::ZipWriter::new(file);
        let options =
            zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Stored);

        let tera = Tera::new("templates/odt/**/*")?;
        let mut context = Context::new();
        let file_name = Path::new(self.input_file).file_name();
        if let Some(f_name_os_str) = file_name {
            if let Some(f_name) = f_name_os_str.to_str() {
                let dimensions = image_dimensions(self.input_file)?;
                let width = dimensions.0 as f64;
                let height = dimensions.1 as f64;

                context.insert("width", &width);
                context.insert("height", &height);
                context.insert("image", &f_name);

                let mut buffer = Vec::new();
                zip.start_file(format!("Pictures/{f_name}"), options)?;
                let mut pic_f = File::open(self.input_file)?;
                pic_f.read_to_end(&mut buffer)?;
                zip.write_all(&buffer)?;

                let manifest_rendered = tera.render("META-INF/manifest.xml", &context)?;
                zip.start_file("META-INF/manifest.xml", options)?;
                zip.write_all(manifest_rendered.as_bytes())?;

                let content_rendered = tera.render("content.xml", &context)?;
                zip.start_file("content.xml", options)?;
                zip.write_all(content_rendered.as_bytes())?;

                zip.finish()?;
            }
        }

        Ok(format!(
            "convert jpg to odt : {} -> {}",
            self.input_file, self.output_file
        ))
    }
}
