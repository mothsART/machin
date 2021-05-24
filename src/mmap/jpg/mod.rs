pub mod jpg2png;

use crate::mmap::{InputTo, JPGInputFile, HashMap};
use crate::mmap::jpg::jpg2png::JPG2PNG;

impl<'a> JPGInputFile<'a> {
    pub fn new(input_file: &'a str, output_file: &'a str) -> JPGInputFile<'a> {
        let png = JPG2PNG::new(input_file, output_file);

        let mut map: HashMap<&'a str, Box<dyn InputTo<'a> +'a>> = HashMap::new();
        map.insert("image/png", Box::new(png));
        JPGInputFile {
            input_file: input_file,
            output_file: output_file,
            map: map
        }
    }
}
