use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::fs;

use crate::machmap::InputTo;

pub struct YamlToJson<'a> {
    pub input_file: &'a str,
    pub output_file: &'a str,
}

impl<'a> YamlToJson<'a> {
    pub fn new(input_file: &'a str, output_file: &'a str) -> YamlToJson<'a> {
        YamlToJson {
            input_file,
            output_file,
        }
    }
}

impl<'a> InputTo<'a> for YamlToJson<'a> {
    fn convert(&self) -> Result<String, Box<dyn Error + 'a>> {
        // TODO : if there is detection of comments, alert that there is a loss of data at the converter
        let content = fs::read_to_string(self.input_file)?;

        let json_value: serde_json::Value = serde_yaml::from_str(&content)?;
        let json_str = serde_json::to_string_pretty(&json_value)?;

        let mut file = File::create(self.output_file)?;
        file.write_all(json_str.as_bytes())?;
        Ok(format!(
            "convert yaml to json : {} -> {}",
            self.input_file, self.output_file
        ))
    }
}
