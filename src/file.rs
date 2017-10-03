//! InfraTec infrared image files.

use {Image, Result};
use std::fs;
use std::path::Path;

/// An .irb file.
#[derive(Debug)]
pub struct File {
    file: fs::File,
    height: u16,
    software_version: String,
    version: Version,
    width: u16,
}

/// The version of an .irb file.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Version(pub u32, pub u32);

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
        use utils::FixedLengthString;
        use Error;
        use std::io::{Read, Seek, SeekFrom};
        use byteorder::{LittleEndian, ReadBytesExt};

        let mut file = fs::File::open(path)?;

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
               file: file,
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

    /// Reads the image from the file.
    ///
    /// # Examples
    ///
    /// ```
    /// use irb::File;
    /// let file = File::open("data/image.irb").unwrap();
    /// let image = file.read_image().unwrap();
    /// ```
    pub fn read_image(&mut self) -> Result<Image> {
        use byteorder::{LittleEndian, ReadBytesExt};
        // FIXME should we be able to read an image twice? Probably not.
        let mut data = Vec::new();
        for _ in 0..(self.height as usize * self.width as usize) {
            data.push(self.file.read_f32::<LittleEndian>()?);
        }
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read() {
        let mut file = File::open("data/image.irb").unwrap();
        assert_eq!("IRBACS", file.software_version());
        assert_eq!(Version(25600, 16384), file.version());
        let image = file.read_image().unwrap();
        assert_eq!(1024, image.width);
        assert_eq!(768, image.height);
        assert!((0.64 - image[(0, 0)]).abs() < 1e-2);
        assert!((-37.49 - image[(767, 1023)]).abs() < 1e-2);
    }
}
