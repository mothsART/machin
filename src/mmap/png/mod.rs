pub mod pngtojpg;
pub mod pngtoavif;

use crate::mmap::png::pngtojpg::PNGToJPG;
use crate::mmap::png::pngtoavif::PNGToAVIF;
use crate::mmap::{HashMap, InputTo, PNGInputFile};

impl<'a> PNGInputFile<'a> {
    pub fn new(input_file: &'a str, output_file: &'a str) -> PNGInputFile<'a> {
        let jpg = PNGToJPG::new(input_file, output_file);
        let avif = PNGToAVIF::new(input_file, output_file);

        let mut map: HashMap<&'a str, Box<dyn InputTo<'a> + 'a>> = HashMap::new();
        map.insert("image/jpeg", Box::new(jpg));
        map.insert("image/avif", Box::new(avif));
        PNGInputFile {
            input_file,
            output_file,
            map,
        }
    }
}
