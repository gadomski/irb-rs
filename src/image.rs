use std::ops::Index;

/// An InfraTec image.
#[derive(Clone, Copy, Debug)]
pub struct Image {
    /// The height of the image.
    pub height: usize,
    /// The width of the image.
    pub width: usize,
}

impl Index<(usize, usize)> for Image {
    type Output = f32;
    fn index(&self, (row, col): (usize, usize)) -> &f32 {
        unimplemented!()
    }
}
