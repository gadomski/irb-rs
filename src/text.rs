//! Read text files exported from InfraTec software.

use {Error, Image, Result};
use std::error;
use std::fmt;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str::FromStr;

/// The text header is incorrect.
#[derive(Debug)]
pub enum HeaderError {
    /// The header is missing a height.
    MissingHeight,
    /// An assignment field does not have an equals sign.
    MissingEqualsSign(String),
    /// The header is missing a width.
    MissingWidth,
}

/// A text irb file.
#[derive(Debug)]
pub struct File {
    header: Header,
    reader: BufReader<fs::File>,
}

#[derive(Debug)]
struct Header {
    height: usize,
    width: usize,
}

impl error::Error for HeaderError {
    fn description(&self) -> &str {
        match *self {
            HeaderError::MissingHeight => "height is missing from the header",
            HeaderError::MissingEqualsSign(_) => "the header row doesn't have an equals sign for assignment",
            HeaderError::MissingWidth => "width is missing from the header",
        }
    }
}

impl fmt::Display for HeaderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            HeaderError::MissingHeight => write!(f, "height is missing from the header"),
            HeaderError::MissingEqualsSign(ref row) => {
                write!(f, "this row doesn't have an equals sign: {}", row)
            }
            HeaderError::MissingWidth => write!(f, "width is missing from the header"),
        }
    }
}

impl File {
    /// Opens a new text .irb file.
    ///
    /// # Examples
    ///
    /// ```
    /// use irb::text::File;
    /// let file = File::open("data/image.txt").unwrap();
    /// ```
    pub fn open<P: AsRef<Path>>(path: P) -> Result<File> {
        let mut reader = BufReader::new(fs::File::open(path)?);
        let header = Header::new(&mut reader)?;
        Ok(File {
               header: header,
               reader: reader,
           })
    }

    /// Reads this file's data and returns the underlying image.
    ///
    /// # Examples
    ///
    /// ```
    /// use irb::text::File;
    /// let image = File::open("data/image.txt").and_then(|file| file.into_image()).unwrap();
    /// ```
    pub fn into_image(self) -> Result<Image> {
        use Error;

        let width = self.width();
        let height = self.height();
        let mut data: Vec<f32> = Vec::new();
        let mut rows = 0;
        for line in self.reader.lines() {
            let mut cols = 0;
            for text in line?.replace(';', " ").split_whitespace() {
                let n = text.replace(',', ".").parse()?;
                data.push(n);
                cols += 1;
            }
            if cols != width {
                return Err(Error::ImageWidth(cols, width));
            }
            rows += 1;
        }
        if rows != height {
            return Err(Error::ImageHeight(rows, height));
        }
        Image::new(data, width, height)
    }

    fn height(&self) -> usize {
        self.header.height
    }

    fn width(&self) -> usize {
        self.header.width
    }
}

impl Header {
    fn new<R: BufRead>(read: R) -> Result<Header> {
        use std::io::ErrorKind;

        let mut width = None;
        let mut height = None;
        for line in read.lines() {
            match line {
                Ok(line) => {
                    if line.starts_with("ImageWidth") {
                        width = Some(Header::parse_value(&line)?);
                    } else if line.starts_with("ImageHeight") {
                        height = Some(Header::parse_value(&line)?);
                    } else if line == "[Data]" {
                        break;
                    }
                }
                Err(err) => {
                    match err.kind() {
                        ErrorKind::InvalidData => continue,
                        _ => return Err(err.into()),
                    }
                }
            }
        }
        if let Some(width) = width {
            if let Some(height) = height {
                Ok(Header {
                       width: width,
                       height: height,
                   })
            } else {
                Err(HeaderError::MissingWidth.into())
            }
        } else {
            Err(HeaderError::MissingWidth.into())
        }
    }

    fn parse_value<T>(s: &str) -> Result<T>
        where T: FromStr,
              Error: From<<T as FromStr>::Err>
    {
        if let Some(value) = s.split('=').skip(1).next() {
            Ok(value.parse()?)
        } else {
            Err(HeaderError::MissingEqualsSign(s.to_string()).into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn open() {
        File::open("data/image.txt").unwrap();
        File::open("data/image2.csv").unwrap();
    }

    #[test]
    fn into_image() {
        let image = File::open("data/image.txt").unwrap().into_image().unwrap();
        assert_eq!(1024, image.width);
        assert_eq!(768, image.height);
        assert_relative_eq!(image[(0, 0)], 0.64, epsilon = 1e-2);
        assert_relative_eq!(image[(0, 1)], 0.58, epsilon = 1e-2);
        assert_relative_eq!(image[(1, 0)], 0.57, epsilon = 1e-2);
        assert_relative_eq!(image[(767, 1023)], -37.49, epsilon = 1e-2);

        let image2 = File::open("data/image2.csv").unwrap().into_image().unwrap();
        assert_eq!(1024, image2.width);
        assert_eq!(768, image2.height);
        assert_relative_eq!(image2[(0, 0)], -38.64, epsilon = 1e-2);
        assert_relative_eq!(image2[(0, 1)], -38.74, epsilon = 1e-2);
        assert_relative_eq!(image2[(1, 0)], -39.15, epsilon = 1e-2);
        assert_relative_eq!(image2[(767, 1023)], 23.84, epsilon = 1e-2);
    }
}
