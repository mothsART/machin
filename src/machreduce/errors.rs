use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct OutputFileUnsupportedError<'a> {
    pub output_file: &'a str,
}

impl<'a> fmt::Display for OutputFileUnsupportedError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Output file extension \"{}\" not supported",
            self.output_file
        )
    }
}

impl<'a> Error for OutputFileUnsupportedError<'a> {}

#[derive(Debug)]
pub struct InputFilesToPdfUnsupportedError {}

impl fmt::Display for InputFilesToPdfUnsupportedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Some inputs files extension not supported")
    }
}

impl Error for InputFilesToPdfUnsupportedError {}
