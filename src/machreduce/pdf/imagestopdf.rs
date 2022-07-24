use std::error::Error;

use colored::Colorize;
use image::image_dimensions;
use image::io::Reader as ImageReader;
use lopdf::content::Content;
use lopdf::xobject;
use lopdf::{dictionary, Document, Object, Stream};
use tempfile::tempdir;

pub struct ImagesToPdf<'a> {
    pub input_mime_type: Vec<&'a str>,
}

impl<'a> Default for ImagesToPdf<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> ImagesToPdf<'a> {
    pub fn new() -> Self {
        ImagesToPdf {
            input_mime_type: vec!["image/png", "image/jpeg"],
        }
    }

    pub fn reduce(
        &self,
        images_path: Vec<String>,
        output_file: &str,
    ) -> Result<String, Box<dyn Error + 'a>> {
        let mut doc = Document::with_version("1.5");
        let pages_id = doc.new_object_id();
        let content = Content { operations: vec![] };

        let mut pdf_kids = Vec::new();
        let tmp_dir = tempdir()?;

        for (i, f) in images_path.iter().enumerate() {
            let content_id = doc.add_object(Stream::new(dictionary! {}, content.encode()?));
            let page_id = doc.add_object(dictionary! {
                "Type" => "Page",
                "Parent" => pages_id,
                "Contents" => content_id,
            });
            let new_path;
            let input_mime = mime_guess::from_path(&f);
            let mut img_path = f;
            if let Some(img_mime) = &input_mime.first_raw() {
                if !img_mime.contains("image/jpeg") {
                    let img = ImageReader::open(&img_path)?.decode()?;
                    new_path = format!("{}-{}.jpg", tmp_dir.path().to_str().unwrap_or(""), i);
                    img_path = &new_path;
                    img.save(&img_path)?;
                }
            }
            if let Ok(img) = xobject::image(img_path) {
                if let Ok(dimensions) = image_dimensions(&img_path) {
                    let insert_result = doc.insert_image(
                        page_id,
                        img,
                        (10., dimensions.1 as f64),
                        (dimensions.0 as f64, dimensions.1 as f64),
                    );
                    if let Some(insert_error) = insert_result.err() {
                        eprintln!(
                            "{}",
                            format!("Couln't insert images: {}", insert_error)
                                .white()
                                .on_red()
                        );
                        continue;
                    }
                    pdf_kids.push(page_id.into());
                }
            }
        }

        let count = pdf_kids.len() as i32;
        let pages = dictionary! {
            "Type" => "Pages",
            "Kids" => pdf_kids,
            "Count" => count,
        };
        doc.objects.insert(pages_id, Object::Dictionary(pages));

        let catalog_id = doc.add_object(dictionary! {
            "Type" => "Catalog",
            "Pages" => pages_id,
        });
        doc.trailer.set("Root", catalog_id);
        doc.compress();
        doc.save(output_file)?;
        Ok(format!("images reduce to {}", output_file))
    }
}
