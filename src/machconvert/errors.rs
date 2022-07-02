use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct UnsupportedRotateError {}

impl Error for UnsupportedRotateError {}

impl fmt::Display for UnsupportedRotateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "rotate value must be 90, 180 or 270 (degree value).")
    }
}

#[derive(Debug)]
pub struct ArgConvertError {}

impl Error for ArgConvertError {}

impl fmt::Display for ArgConvertError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "no conversion argument matches this file type")
    }
}
