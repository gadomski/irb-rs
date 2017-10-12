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
    /// Returns an error if the length of the data does equal width * height. Data are expected to
    /// be in row-major order.
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

    /// Gets the color at the provided column and row.
    ///
    /// Returns None if the row and column are out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use irb::Image;
    /// let image = Image::new(vec![1., 2., 3., 4.], 2, 2).unwrap();
    /// assert_eq!(1., *image.get(0, 0).unwrap());
    /// assert_eq!(2., *image.get(1, 0).unwrap());
    /// assert_eq!(3., *image.get(0, 1).unwrap());
    /// assert_eq!(None, image.get(2, 2));
    /// ```
    pub fn get(&self, col: usize, row: usize) -> Option<&f32> {
        if row < self.height && col < self.width {
            self.data.get(row * self.width + col)
        } else {
            None
        }
    }
}

impl Index<(usize, usize)> for Image {
    type Output = f32;
    fn index(&self, (col, row): (usize, usize)) -> &f32 {
        self.get(col, row).expect(&format!(
            "Index out of bounds for {}x{} image: ({}, {})",
            self.width,
            self.height,
            row,
            col
        ))
    }
}
