pub mod yamltojson;

use crate::machmap::yaml::yamltojson::YamlToJson;
use crate::machmap::{HashMap, InputTo, YamlInputFile};

impl<'a> YamlInputFile<'a> {
    pub fn new(input_file: &'a str, output_file: &'a str) -> YamlInputFile<'a> {
        let json = YamlToJson::new(input_file, output_file);

        let mut map: HashMap<&'a str, Box<dyn InputTo<'a> + 'a>> = HashMap::new();
        map.insert("application/json", Box::new(json));
        YamlInputFile {
            input_file,
            output_file,
            map,
        }
    }
}
