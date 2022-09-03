pub mod jsontoyaml;

use crate::machmap::json::jsontoyaml::JsonToYaml;
use crate::machmap::{HashMap, InputTo, JsonInputFile};

impl<'a> JsonInputFile<'a> {
    pub fn new(input_file: &'a str, output_file: &'a str) -> JsonInputFile<'a> {
        let yaml = JsonToYaml::new(input_file, output_file);

        let mut map: HashMap<&'a str, Box<dyn InputTo<'a> + 'a>> = HashMap::new();
        map.insert("text/x-yaml", Box::new(yaml));
        JsonInputFile {
            input_file,
            output_file,
            map,
        }
    }
}
