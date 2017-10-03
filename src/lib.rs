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
