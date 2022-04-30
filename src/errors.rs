use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct UnSupportedError<'a> {
    pub input_file: &'a str,
    pub output_ext: &'a str,
}

impl<'a> fmt::Display for UnSupportedError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.input_file, self.output_ext)
    }
}

impl<'a> Error for UnSupportedError<'a> {}

#[derive(Debug)]
pub struct InputFileUnsupportedError<'a> {
    pub input_file: &'a str,
}

impl<'a> fmt::Display for InputFileUnsupportedError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Input file \"{}\" not supported", self.input_file)
    }
}

impl<'a> Error for InputFileUnsupportedError<'a> {}

#[derive(Debug)]
pub struct InputFileExtUnReconizeError<'a> {
    pub input_file: &'a str,
}

impl<'a> fmt::Display for InputFileExtUnReconizeError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Input file extension \"{}\" doesn't been reconize",
            self.input_file
        )
    }
}

impl<'a> Error for InputFileExtUnReconizeError<'a> {}
