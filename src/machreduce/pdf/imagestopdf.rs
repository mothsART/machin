use std::error::Error;

use colored::Colorize;
use image::image_dimensions;
use image::ImageReader;
use lopdf::content::Content;
use lopdf::xobject;
use lopdf::{dictionary, Document, Object, Stream};
use tempfile::tempdir;

pub struct ImagesToPdf<'a> {
    pub input_mime_type: Vec<&'a str>,
}

impl Default for ImagesToPdf<'_> {
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
        images_path: Vec<&String>,
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
            let img = ImageReader::open(f)?.decode()?;
            let new_path = format!("{}-{}.jpg", tmp_dir.path().to_str().unwrap_or(""), i);
            if img.color().has_alpha() {
                colored_warn!(format!(
                    "Warning : input file \"{}\" have an alpha channel : is not supported for en jpeg file. The output file \"{}\" will no longer have an alpha channel.",
                    f,
                    output_file,
                ));
            }
            img.to_rgb8().save(&new_path)?;

            if let Ok(img) = xobject::image(new_path) {
                if let Ok(dimensions) = image_dimensions(f) {
                    // magical values ? TODO : ref to pdf documentation
                    let max_width = 590.;
                    let max_height = 770.;

                    let mut padding_left = 10.;
                    let mut padding_right = 10.;
                    let mut width = dimensions.0 as f32;
                    let mut height = dimensions.1 as f32;
                    let ratio = height / width;

                    if width > max_width || height > max_height {
                        if ratio > 1. {
                            height = max_height;
                            width = height / ratio;
                        } else {
                            width = max_width;
                            height = width * ratio;
                        }
                    }
                    if max_width > width {
                        padding_left = (max_width - width) / 2.;
                    }
                    if max_height > height {
                        padding_right = (max_height - height) / 2.;
                    }
                    let insert_result = doc.insert_image(
                        page_id,
                        img,
                        (padding_left, padding_right),
                        (width, height),
                    );
                    if let Some(insert_error) = insert_result.err() {
                        colored_err!(format!("Couln't insert images: {insert_error}"));
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
        Ok(format!("images reduce to {output_file}"))
    }
}
