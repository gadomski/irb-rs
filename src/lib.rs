//! Read InfraTec .irb thermal image files.
//!
//! This is a reverse-engineered library, based on [riri](https://sourceforge.net/projects/riri/).
//! Riri wasn't correct anymore, so we've done some legwork to make the library work for the data
//! that we have. YMMV.
//!
//! I'm working to communicate with InfraTec to get a format definition, so I can make this library
//! actually correct rather than guesswork.

#![deny(missing_docs,
        missing_debug_implementations, missing_copy_implementations,
        trivial_casts, trivial_numeric_casts,
        unsafe_code,
        unstable_features,
        unused_import_braces, unused_qualifications)]

extern crate byteorder;

pub mod file;
mod image;
mod utils;

pub use file::File;
pub use image::Image;

/// Our custom error enum.
#[derive(Debug)]
pub enum Error {
    /// A fixed-length string has an interior nul byte.
    InteriorNulByte(Vec<u8>),
    /// The file magic number is invalid.
    InvalidHeader([u8; 5]),
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
