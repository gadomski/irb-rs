//! Read text files exported from InfraTec software.
//!
//! This doesn't need the external **irbasc-sys** project.

use std::io::{BufRead, BufReader};
use std::ops::Index;
use std::path::Path;
use std::str::FromStr;
use {Error, Result};

/// A text irb file.
#[derive(Debug)]
pub struct Irb {
    #[allow(dead_code)]
    height: usize,
    width: usize,
    data: Vec<f64>,
}

impl Irb {
    /// Opens a new text .irb file.
    ///
    /// # Examples
    ///
    /// ```
    /// use irb::text::Irb;
    /// let irb = Irb::from_path("data/image.txt").unwrap();
    /// ```
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Irb> {
        use std::fs::File;

        let mut read = BufReader::new(File::open(path)?);
        let (width, height) = Irb::read_header(&mut read)?;
        let mut data = Vec::with_capacity(width * height);
        let mut rows = 0;
        for line in read.lines() {
            let mut cols = 0;
            for text in line?.replace(';', " ").split_whitespace() {
                let n = text.replace(',', ".").parse()?;
                data.push(n);
                cols += 1;
            }
            if cols != width {
                return Err(Error::TextDataParse(format!(
                    "Expected {} cols, got {}",
                    width, cols
                )));
            }
            rows += 1;
        }
        if rows != height {
            return Err(Error::TextDataParse(format!(
                "Expected {} rows, got {}",
                width, rows
            )));
        }
        Ok(Irb {
            height: height,
            width: width,
            data: data,
        })
    }

    /// Returns the temperature at the given column and row.
    ///
    /// # Examples
    ///
    /// ```
    /// use irb::text::Irb;
    /// let irb = Irb::from_path("data/image.txt").unwrap();
    /// let temperature = irb.temperature(0, 0).unwrap();
    /// ```
    pub fn temperature(&self, col: usize, row: usize) -> Option<&f64> {
        self.data.get(row * self.width + col)
    }

    fn read_header<R: BufRead>(read: R) -> Result<(usize, usize)> {
        use std::io::ErrorKind;

        let mut width = None;
        let mut height = None;
        for line in read.lines() {
            match line {
                Ok(line) => {
                    if line.starts_with("ImageWidth") {
                        width = Some(parse_value(&line)?);
                    } else if line.starts_with("ImageHeight") {
                        height = Some(parse_value(&line)?);
                    } else if line == "[Data]" {
                        break;
                    }
                }
                Err(err) => match err.kind() {
                    ErrorKind::InvalidData => continue,
                    _ => return Err(err.into()),
                },
            }
        }
        if let Some(width) = width {
            if let Some(height) = height {
                Ok((width, height))
            } else {
                Err(Error::TextHeaderParse("Missing height".to_string()))
            }
        } else {
            Err(Error::TextHeaderParse("Missing width".to_string()))
        }
    }
}

impl Index<(usize, usize)> for Irb {
    type Output = f64;
    fn index(&self, (col, row): (usize, usize)) -> &f64 {
        self.temperature(col, row)
            .expect(&format!("Index out of bounds: ({}, {})", col, row))
    }
}

fn parse_value<T>(s: &str) -> Result<T>
where
    T: FromStr,
    Error: From<<T as FromStr>::Err>,
{
    if let Some(value) = s.split('=').skip(1).next() {
        Ok(value.parse()?)
    } else {
        Err(Error::TextHeaderParse(s.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_path() {
        Irb::from_path("data/image.txt").unwrap();
        Irb::from_path("data/image2.csv").unwrap();
        assert!(Irb::from_path("data/image2.irb").is_err());
    }

    #[test]
    fn temperature() {
        let irb = Irb::from_path("data/image.txt").unwrap();
        assert_eq!(1024, irb.width);
        assert_eq!(768, irb.height);
        assert_relative_eq!(irb[(0, 0)], 0.64, epsilon = 1e-2);
        assert_relative_eq!(irb[(1, 0)], 0.58, epsilon = 1e-2);
        assert_relative_eq!(irb[(0, 1)], 0.57, epsilon = 1e-2);
        assert_relative_eq!(irb[(1023, 767)], -37.49, epsilon = 1e-2);

        let irb2 = Irb::from_path("data/image2.csv").unwrap();
        assert_eq!(1024, irb2.width);
        assert_eq!(768, irb2.height);
        assert_relative_eq!(irb2[(0, 0)], -38.64, epsilon = 1e-2);
        assert_relative_eq!(irb2[(1, 0)], -38.74, epsilon = 1e-2);
        assert_relative_eq!(irb2[(0, 1)], -39.15, epsilon = 1e-2);
        assert_relative_eq!(irb2[(1023, 767)], 23.84, epsilon = 1e-2);
    }
}
