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
    /// Wrapper around `std::io::Error`.
    Io(std::io::Error),
}

/// Our custom result type.
pub type Result<T> = std::result::Result<T, Error>;

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::Io(err)
    }
}
