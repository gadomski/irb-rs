//! Read InfraTec .irb thermal image files.

#![deny(missing_docs,
        missing_debug_implementations, missing_copy_implementations,
        trivial_casts, trivial_numeric_casts,
        unstable_features,
        unused_import_braces, unused_qualifications)]

#[cfg(test)]
#[macro_use]
extern crate approx;
#[cfg(feature = "irbacs-sys")]
extern crate irbacs_sys;
#[macro_use]
extern crate quick_error;

#[cfg(feature = "irbacs-sys")]
pub mod acs;
pub mod text;

#[cfg(feature = "irbacs-sys")]
pub use acs::Irb;

quick_error! {
    /// Our custom error enum.
    #[derive(Debug)]
    pub enum Error {
        /// Wrapper around `std::ffi::NulError`.
        FfiNul(err: std::ffi::NulError) {
            cause(err)
            description(err.description())
            display("FFI nul error: {}", err)
            from()
        }
        /// Wrapper around `std::ffi::IntoStringError`.
        FfiIntoString(err: std::ffi::IntoStringError) {
            cause(err)
            description(err.description())
            display("FFI into string error: {}", err)
            from()
        }
        /// Wrapper around `std::io::Error`.
        Io(err: std::io::Error) {
            cause(err)
            description(err.description())
            display("IO error: {}", err)
            from()
        }
        /// Error in the irbasc library.
        ///
        /// The library doesn't have great error messages so these can be pretty opaque.
        #[cfg(feature = "irbacs-sys")]
        IrbacsSys(cause: String) {
            description("error in the irbasc library")
            display("Irbasc-sys error: {}", cause)
        }
        /// Wraper around `std::num::ParseFloat`.
        ParseFloat(err: std::num::ParseFloatError) {
            cause(err)
            description(err.description())
            display("Parse float error: {}", err)
            from()
        }
        /// Wraper around `std::num::ParseIntError`.
        ParseInt(err: std::num::ParseIntError) {
            cause(err)
            description(err.description())
            display("Parse int error: {}", err)
            from()
        }
        /// Error while parsing text image data.
        TextDataParse(message: String) {
            description("error parsing text irb file data")
            display("Could not parse text irb file data: {}", message)
        }
        /// Error while parsing a text image header.
        TextHeaderParse(line: String) {
            description("error parsing a text irb file header")
            display("Could not parse text irb file header line: {}", line)
        }
    }
}

/// Our custom result type.
pub type Result<T> = std::result::Result<T, Error>;
