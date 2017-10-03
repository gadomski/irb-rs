use Result;
use std::ops::Index;

/// An InfraTec image.
#[derive(Debug)]
pub struct Image {
    /// The height of the image.
    pub height: usize,
    /// The width of the image.
    pub width: usize,
    data: Vec<f32>,
}

impl Image {
    /// Creates a new image from the provided bytes, width, and height.
    ///
    /// Returns an error if the length of the data does equal width * height.
    ///
    /// # Examples
    ///
    /// ```
    /// use irb::Image;
    /// let image = Image::new(vec![1., 2., 3., 4.], 2, 2).unwrap();
    /// assert_eq!(1., image[(0, 0)]);
    /// assert!(Image::new(vec![], 1, 1).is_err());
    /// ```
    pub fn new(data: Vec<f32>, width: usize, height: usize) -> Result<Image> {
        use Error;

        if data.len() != width * height {
            Err(Error::ImageLength(data.len(), width * height))
        } else {
            Ok(Image {
                   data: data,
                   height: height,
                   width: width,
               })
        }
    }
}

impl Index<(usize, usize)> for Image {
    type Output = f32;
    fn index(&self, (row, col): (usize, usize)) -> &f32 {
        if row < self.height && col < self.width {
            &self.data[row * self.width + col]
        } else {
            panic!("Index out of bounds for {}x{} image: ({}, {})",
                   self.height,
                   self.width,
                   row,
                   col);
        }
    }
}
