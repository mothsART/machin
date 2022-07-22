use std::error::Error;
use std::path::Path;

use colored::*;
use lopdf::content::{Content};
use lopdf::{Document, Object, Stream, dictionary};
use lopdf::xobject;
use image::image_dimensions;

use crate::machreduce::{InputTo, Direction};

pub struct PdfOutputFile<'a> {
    pub output_file: &'a str,
    pub input_mime_type: Vec<&'a str>,
    pub output_mime_type: &'a str,
}

impl<'a> PdfOutputFile<'a> {
    pub fn new(output_file: &'a str) -> PdfOutputFile<'a> {
        PdfOutputFile {
            output_file,
            input_mime_type: vec!["image/jpeg"],
            output_mime_type: "application/pdf",
        }
    }
}

impl<'a> InputTo<'a> for PdfOutputFile<'a> {
    fn reduce(&self, _direction: &Direction) -> Result<String, Box<dyn Error + 'a>> {
        let lines = std::io::stdin().lines();

        let mut doc = Document::with_version("1.5");
        let pages_id = doc.new_object_id();
        let content = Content {
            operations: vec![],
        };
        
        let mut pdf_kids = Vec::new();
        let mut _files = Vec::new();

        for line in lines {
            match line {
                Ok(_l) => {
                    if !Path::new(&_l).exists() {
                        eprintln!(
                            "{}",
                            format!("Input file \"{}\" doesn't exist", _l)
                                .white()
                                .on_red()
                        );
                        continue;
                    }
                    if let Some(o_mime) = mime_guess::from_path(&_l).first_raw() {
                        if !self.input_mime_type.contains(&o_mime) {
                            eprintln!(
                                "{}",
                                format!("Input file \"{}\" not supported", _l)
                                    .white()
                                    .on_red()
                            );
                            continue;
                        }
                        _files.push(_l);
                    }
                }
                Err(_) => {
                    continue;
                }
            }
        }

        for f in &_files {
            let content_id = doc.add_object(Stream::new(dictionary! {}, content.encode()?));
            let page_id = doc.add_object(dictionary! {
                "Type" => "Page",
                "Parent" => pages_id,
                "Contents" => content_id,
            });
            if let Ok(img) = xobject::image(f) {
                if let Ok(dimensions) = image_dimensions(&f) {
                    let insert_result = doc.insert_image(
                        page_id,
                        img,
                        (10., dimensions.1 as f64),
                        (dimensions.0 as f64, dimensions.1 as f64)
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
        doc.save(self.output_file)?;
        Ok(format!("images reduce to {}", self.output_file))
    }
}
