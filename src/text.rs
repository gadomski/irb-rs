//! Read text files exported from InfraTec software.

use Result;
use std::path::Path;

/// A text irb file.
#[derive(Clone, Copy, Debug)]
pub struct File {}

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
        Ok(File {})
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn open() {
        File::open("data/image.txt").unwrap();
    }
}
