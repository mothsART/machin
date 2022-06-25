pub mod jpgtopng;
pub mod jpgtoavif;

use crate::mmap::jpg::jpgtopng::JPGTOPNG;
use crate::mmap::jpg::jpgtoavif::JPGTOAVIF;

use crate::mmap::{HashMap, InputTo, JPGInputFile};

impl<'a> JPGInputFile<'a> {
    pub fn new(input_file: &'a str, output_file: &'a str) -> JPGInputFile<'a> {
        let png = JPGTOPNG::new(input_file, output_file);
        let avif = JPGTOAVIF::new(input_file, output_file);

        let mut map: HashMap<&'a str, Box<dyn InputTo<'a> + 'a>> = HashMap::new();
        map.insert("image/png", Box::new(png));
        map.insert("image/avif", Box::new(avif));
        JPGInputFile {
            input_file,
            output_file,
            map,
        }
    }
}
