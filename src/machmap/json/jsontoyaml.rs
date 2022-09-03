use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::fs;

use crate::machmap::InputTo;

pub struct JsonToYaml<'a> {
    pub input_file: &'a str,
    pub output_file: &'a str,
}

impl<'a> JsonToYaml<'a> {
    pub fn new(input_file: &'a str, output_file: &'a str) -> JsonToYaml<'a> {
        JsonToYaml {
            input_file,
            output_file,
        }
    }
}

impl<'a> InputTo<'a> for JsonToYaml<'a> {
    fn convert(&self) -> Result<String, Box<dyn Error + 'a>> {
        let content = fs::read_to_string(self.input_file)?;

        let yaml_value: serde_yaml::Value = serde_json::from_str(&content)?;
        let yaml_str = serde_yaml::to_string(&yaml_value)?;

        let mut file = File::create(self.output_file)?;
        file.write_all(yaml_str.as_bytes())?;
        Ok(format!(
            "convert json to yaml : {} -> {}",
            self.input_file, self.output_file
        ))
    }
}
