use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct UnSupportedError<'a> {
    pub input_file: &'a str,
    pub output_ext: &'a str,
}

impl fmt::Display for UnSupportedError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.input_file, self.output_ext)
    }
}

impl Error for UnSupportedError<'_> {}

#[derive(Debug)]
pub struct InputFileUnsupportedError<'a> {
    pub input_file: &'a str,
}

impl fmt::Display for InputFileUnsupportedError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Input file \"{}\" not supported", self.input_file)
    }
}

impl Error for InputFileUnsupportedError<'_> {}

#[derive(Debug)]
pub struct InputFileExtUnReconizeError<'a> {
    pub input_file: &'a str,
}

impl fmt::Display for InputFileExtUnReconizeError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Input file extension \"{}\" doesn't been reconize",
            self.input_file
        )
    }
}

impl Error for InputFileExtUnReconizeError<'_> {}
