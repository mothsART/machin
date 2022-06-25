pub mod markdowntohtml;

use crate::mmap::markdown::markdowntohtml::MarkdownToHTML;
use crate::mmap::{HashMap, InputTo, MarkdownInputFile};

impl<'a> MarkdownInputFile<'a> {
    pub fn new(input_file: &'a str, output_file: &'a str) -> MarkdownInputFile<'a> {
        let html = MarkdownToHTML::new(input_file, output_file);

        let mut map: HashMap<&'a str, Box<dyn InputTo<'a> + 'a>> = HashMap::new();
        map.insert("text/html", Box::new(html));
        MarkdownInputFile {
            input_file,
            output_file,
            map,
        }
    }
}
