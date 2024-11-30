use std::collections::BTreeMap;
use std::error::Error;

use lopdf::{Bookmark, Document, Object, ObjectId};

pub struct PdfToPdf<'a> {
    pub input_mime_type: &'a str,
}

impl Default for PdfToPdf<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> PdfToPdf<'a> {
    pub fn new() -> Self {
        PdfToPdf {
            input_mime_type: "application/pdf",
        }
    }

    pub fn reduce(
        &self,
        pdf_path: Vec<&String>,
        output_file: &str,
    ) -> Result<String, Box<dyn Error + 'a>> {
        let mut doc = Document::with_version("1.5");

        // Define a starting max_id (will be used as start index for object_ids)
        let mut max_id = 1;
        let mut pagenum = 1;

        // Collect all Documents Objects grouped by a map
        let mut documents_pages = BTreeMap::new();
        let mut documents_objects = BTreeMap::new();

        for d in pdf_path {
            let p = std::path::Path::new(&d);
            let mut _doc = Document::load(p).unwrap();
            let mut first = false;
            _doc.renumber_objects_with(max_id);

            max_id = doc.max_id + 1;

            documents_pages.extend(
                _doc.get_pages()
                    .into_values()
                    .map(|object_id| {
                        if !first {
                            let bookmark = Bookmark::new(
                                format!("Page_{pagenum}"),
                                [0.0, 0.0, 1.0],
                                0,
                                object_id,
                            );
                            doc.add_bookmark(bookmark, None);
                            first = true;
                            pagenum += 1;
                        }

                        (object_id, _doc.get_object(object_id).unwrap().to_owned())
                    })
                    .collect::<BTreeMap<ObjectId, Object>>(),
            );
            documents_objects.extend(_doc.objects);
        }

        // Catalog and Pages are mandatory
        let mut catalog_object: Option<(ObjectId, Object)> = None;
        let mut pages_object: Option<(ObjectId, Object)> = None;

        // Process all objects except "Page" type
        for (object_id, object) in documents_objects.iter() {
            // We have to ignore "Page" (as are processed later), "Outlines" and "Outline" objects
            // All other objects should be collected and inserted into the main Document
            match object.type_name().unwrap_or("") {
                "Catalog" => {
                    // Collect a first "Catalog" object and use it for the future "Pages"
                    catalog_object = Some((
                        if let Some((id, _)) = catalog_object {
                            id
                        } else {
                            *object_id
                        },
                        object.clone(),
                    ));
                }
                "Pages" => {
                    // Collect and update a first "Pages" object and use it for the future "Catalog"
                    // We have also to merge all dictionaries of the old and the new "Pages" object
                    if let Ok(dictionary) = object.as_dict() {
                        let mut dictionary = dictionary.clone();
                        if let Some((_, ref object)) = pages_object {
                            if let Ok(old_dictionary) = object.as_dict() {
                                dictionary.extend(old_dictionary);
                            }
                        }

                        pages_object = Some((
                            if let Some((id, _)) = pages_object {
                                id
                            } else {
                                *object_id
                            },
                            Object::Dictionary(dictionary),
                        ));
                    }
                }
                "Page" => {}     // Ignored, processed later and separately
                "Outlines" => {} // Ignored, not supported yet
                "Outline" => {}  // Ignored, not supported yet
                _ => {
                    doc.objects.insert(*object_id, object.clone());
                }
            }
        }

        // If no "Pages" found abort
        if pages_object.is_none() {
            println!("Pages root not found.");
        }

        // Iter over all "Page" and collect with the parent "Pages" created before
        for (object_id, object) in documents_pages.iter() {
            if let Ok(dictionary) = object.as_dict() {
                let mut dictionary = dictionary.clone();
                dictionary.set("Parent", pages_object.as_ref().unwrap().0);

                doc.objects
                    .insert(*object_id, Object::Dictionary(dictionary));
            }
        }

        // If no "Catalog" found abort
        if catalog_object.is_none() {
            println!("Catalog root not found.");
        }

        let catalog_object = catalog_object.unwrap();
        let pages_object = pages_object.unwrap();

        // Build a new "Pages" with updated fields
        if let Ok(dictionary) = pages_object.1.as_dict() {
            let mut dictionary = dictionary.clone();

            // Set new pages count
            dictionary.set("Count", documents_pages.len() as u32);

            // Set new "Kids" list (collected from documents pages) for "Pages"
            dictionary.set(
                "Kids",
                documents_pages
                    .into_keys()
                    .map(Object::Reference)
                    .collect::<Vec<_>>(),
            );

            doc.objects
                .insert(pages_object.0, Object::Dictionary(dictionary));
        }

        // Build a new "Catalog" with updated fields
        if let Ok(dictionary) = catalog_object.1.as_dict() {
            let mut dictionary = dictionary.clone();
            dictionary.set("Pages", pages_object.0);
            dictionary.remove(b"Outlines"); // Outlines not supported in merged PDFs

            doc.objects
                .insert(catalog_object.0, Object::Dictionary(dictionary));
        }

        doc.trailer.set("Root", catalog_object.0);

        // Update the max internal ID as wasn't updated before due to direct objects insertion
        doc.max_id = doc.objects.len() as u32;

        // Reorder all new Document objects
        doc.renumber_objects();

        //Set any Bookmarks to the First child if they are not set to a page
        doc.adjust_zero_pages();

        //Set all bookmarks to the PDF Object tree then set the Outlines to the Bookmark content map.
        if let Some(n) = doc.build_outline() {
            if let Ok(Object::Dictionary(ref mut dict)) = doc.get_object_mut(catalog_object.0) {
                dict.set("Outlines", Object::Reference(n));
            }
        }

        doc.compress();

        // Save the merged PDF
        doc.save(output_file)?;
        Ok(format!("pdf reduce to {output_file}"))
    }
}
