//! Read InfraTec .irb thermal image files.
//!
//! For now, we can't actually read .irb files themselves. It's a closed format and I can't find a
//! reference anywhere on the internet. I'm hoping to get in touch with InfraTec and get a format
//! definition from them.
//!
//! Until then, we *can* read text files exportd by InfraTec software, an example of which is in
//! `data/image.txt`.

#![deny(missing_docs,
        missing_debug_implementations, missing_copy_implementations,
        trivial_casts, trivial_numeric_casts,
        unsafe_code,
        unstable_features,
        unused_import_braces, unused_qualifications)]

pub mod text;
mod image;

pub use image::Image;

/// Our custom error enum.
#[derive(Debug)]
pub enum Error {
    /// The image height does not match the header.
    ImageHeight(usize, usize),
    /// The data in the image vector does not match width * height.
    ImageLength(usize, usize),
    /// The image width does not match the header.
    ImageWidth(usize, usize),
    /// Wrapper around `std::io::Error`.
    Io(std::io::Error),
    /// Wrapper around `std::num::ParseFloatError`.
    ParseFloat(std::num::ParseFloatError),
    /// Wrapper around `std::num::ParseIntError`.
    ParseInt(std::num::ParseIntError),
    /// Wrapper around `irb::text::HeaderError`.
    TextHeader(text::HeaderError),
}

/// Our custom result type.
pub type Result<T> = std::result::Result<T, Error>;

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::ImageHeight(_, _) => "image height does not match header",
            Error::ImageLength(_, _) => "image length does not match header",
            Error::ImageWidth(_, _) => "image width does not match header",
            Error::Io(ref err) => err.description(),
            Error::ParseFloat(ref err) => err.description(),
            Error::ParseInt(ref err) => err.description(),
            Error::TextHeader(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&std::error::Error> {
        match *self {
            Error::Io(ref err) => Some(err),
            Error::ParseFloat(ref err) => Some(err),
            Error::ParseInt(ref err) => Some(err),
            Error::TextHeader(ref err) => Some(err),
            Error::ImageHeight(_, _) |
            Error::ImageLength(_, _) |
            Error::ImageWidth(_, _) => None,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::ImageHeight(expected, actual) => {
                write!(f, "expected {} rows, got {}", expected, actual)
            }
            Error::ImageLength(expected, actual) => {
                write!(f, "expected {} values, got {}", expected, actual)
            }
            Error::ImageWidth(expected, actual) => {
                write!(f, "expected {} columns, got {}", expected, actual)
            }
            Error::Io(ref err) => write!(f, "io error: {}", err),
            Error::ParseFloat(ref err) => write!(f, "float parse error: {}", err),
            Error::ParseInt(ref err) => write!(f, "int parse error: {}", err),
            Error::TextHeader(ref err) => write!(f, "irb text header error: {}", err),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<std::num::ParseFloatError> for Error {
    fn from(err: std::num::ParseFloatError) -> Error {
        Error::ParseFloat(err)
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(err: std::num::ParseIntError) -> Error {
        Error::ParseInt(err)
    }
}

impl From<text::HeaderError> for Error {
    fn from(err: text::HeaderError) -> Error {
        Error::TextHeader(err)
    }
}
