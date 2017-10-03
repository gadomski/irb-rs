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

use std::path::Path;

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

/// An .irb file.
#[derive(Debug)]
pub struct File {
    height: u16,
    software_version: String,
    version: Version,
    width: u16,
}

/// The version of an .irb file.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Version(pub u32, pub u32);

struct FixedLengthString {
    bytes: Vec<u8>,
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::Io(err)
    }
}

impl File {
    /// Opens an .irb file for a given path.
    ///
    /// # Examples
    ///
    /// ```
    /// use irb::File;
    /// let file = File::open("data/image.irb").unwrap();
    /// ```
    pub fn open<P: AsRef<Path>>(path: P) -> Result<File> {
        use std::io::{Read, Seek, SeekFrom};
        use byteorder::{LittleEndian, ReadBytesExt};

        let mut file = std::fs::File::open(path)?;

        let mut header = [0; 5];
        file.read_exact(&mut header)?;
        if header != [0xff, 0x49, 0x52, 0x42, 0] {
            return Err(Error::InvalidHeader(header));
        }

        let mut software_version = FixedLengthString::new(15);
        file.read_exact(software_version.as_mut())?;

        let version = Version(file.read_u32::<LittleEndian>()?,
                              file.read_u32::<LittleEndian>()?);

        file.read_u32::<LittleEndian>()?; // totalindex
        file.seek(SeekFrom::Start(6059))?;
        let width = file.read_u16::<LittleEndian>()?;
        let height = file.read_u16::<LittleEndian>()?;
        file.seek(SeekFrom::Start(6119))?;

        Ok(File {
               height: height,
               software_version: software_version.to_string()?,
               version: version,
               width: width,
           })
    }

    /// Returns the software version of this file.
    ///
    /// Because this is from a reverse-engineer, I'm not sure what this really means.
    ///
    /// # Examples
    ///
    /// ```
    /// use irb::File;
    /// let file = File::open("data/image.irb").unwrap();
    /// let version = file.software_version();
    /// ```
    pub fn software_version(&self) -> &str {
        &self.software_version
    }

    /// Returns the version of this file.
    ///
    /// # Examples
    ///
    /// ```
    /// use irb::File;
    /// let file = File::open("data/image.irb").unwrap();
    /// let version = file.version();
    /// ```
    pub fn version(&self) -> Version {
        self.version
    }

    /// Returns this image's height.
    ///
    /// # Examples
    ///
    /// ```
    /// use irb::File;
    /// let file = File::open("data/image.irb").unwrap();
    /// let height = file.height();
    /// ```
    pub fn height(&self) -> u16 {
        self.height
    }

    /// Returns this image's width.
    ///
    /// # Examples
    ///
    /// ```
    /// use irb::File;
    /// let file = File::open("data/image.irb").unwrap();
    /// let width = file.width();
    /// ```
    pub fn width(&self) -> u16 {
        self.width
    }
}

impl FixedLengthString {
    fn new(length: usize) -> FixedLengthString {
        FixedLengthString { bytes: vec![0; length] }
    }

    fn to_string(&self) -> Result<String> {
        let mut string = String::new();
        let mut done = false;
        for &byte in &self.bytes {
            if byte == 0 {
                if done {
                    continue;
                } else {
                    done = true;
                }
            } else {
                if done {
                    return Err(Error::InteriorNulByte(self.bytes.clone()));
                } else {
                    string.push(byte as char);
                }
            }
        }
        Ok(string)
    }
}

impl AsMut<[u8]> for FixedLengthString {
    fn as_mut(&mut self) -> &mut [u8] {
        self.bytes.as_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read() {
        let file = File::open("data/image.irb").unwrap();
        assert_eq!("IRBACS", file.software_version());
        assert_eq!(Version(25600, 16384), file.version());
        assert_eq!(1024, file.width());
        assert_eq!(768, file.height());
    }
}
