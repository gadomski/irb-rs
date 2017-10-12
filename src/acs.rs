//! Wrappers to make **irbasc-sys** play nice.

use {Error, Result};
use irbacs_sys;
use std::ffi::CString;

/// Returns the dll version.
///
/// # Examples
///
/// ```
/// let version = irb::acs::version().unwrap();
/// println!("main={}, sub={}", version.main, version.sub);
/// ```
pub fn version() -> Result<Version> {
    let mut main = 0;
    let mut sub = 0;
    if unsafe { irbacs_sys::version(&mut main, &mut sub) } == 1 {
        Ok(Version {
            main: main,
            sub: sub,
        })
    } else {
        Err(Error::IrbacsSys("version".to_string()))
    }
}

/// Simple version structure.
#[derive(Clone, Copy, Debug)]
pub struct Version {
    /// The main version number.
    pub main: i32,
    /// The sub version number.
    pub sub: i32,
}

/// An irb file.
#[derive(Debug)]
pub struct Irb {
    handle: irbacs_sys::PtrUint,
}

impl Irb {
    /// Opens an irb file from a path.
    ///
    /// # Examples
    ///
    /// ```
    /// use irb::Irb;
    /// let irb = Irb::from_path("data/image.irb").unwrap();
    /// ```
    pub fn from_path<P: Into<Vec<u8>>>(path: P) -> Result<Irb> {
        let path = CString::new(path)?;
        let result = unsafe { irbacs_sys::loadIRB(path.as_ptr()) };
        if result.is_null() {
            Err(Error::IrbacsSys(
                format!("loadIRB: {}", path.to_string_lossy()),
            ))
        } else {
            Ok(Irb { handle: result })
        }
    }

    /// Returns the image width.
    ///
    /// # Examples
    ///
    /// ```
    /// use irb::Irb;
    /// let irb = Irb::from_path("data/image.irb").unwrap();
    /// let width = irb.image_width().unwrap();
    /// ```
    pub fn image_width(&self) -> Result<i32> {
        self.param(0).map(|n| n as i32)
    }

    /// Returns the image height.
    ///
    /// # Examples
    ///
    /// ```
    /// use irb::Irb;
    /// let irb = Irb::from_path("data/image.irb").unwrap();
    /// let height = irb.image_height().unwrap();
    /// ```
    pub fn image_height(&self) -> Result<i32> {
        self.param(1).map(|n| n as i32)
    }

    /// Returns the frame count.
    ///
    /// # Examples
    ///
    /// ```
    /// use irb::Irb;
    /// let irb = Irb::from_path("data/image.irb").unwrap();
    /// let frame_count = irb.frame_count().unwrap();
    /// ```
    pub fn frame_count(&self) -> Result<i32> {
        let result = unsafe { irbacs_sys::getFrameCount(self.handle) };
        if result == 0 {
            Err(Error::IrbacsSys("getFrameCount".to_string()))
        } else {
            Ok(result)
        }
    }

    /// Returns the number of irb-indices.
    ///
    /// # Examples
    ///
    /// ```
    /// use irb::Irb;
    /// let irb = Irb::from_path("data/image.irb").unwrap();
    /// let index_count = irb.index_count().unwrap();
    /// ```
    pub fn index_count(&self) -> Result<i32> {
        use std::ptr;
        let result = unsafe { irbacs_sys::getIRBIndices(self.handle, ptr::null()) };
        if result == 0 {
            Err(Error::IrbacsSys("getIRBIndices (with null)".to_string()))
        } else {
            Ok(result)
        }
    }

    /// Returns the temperature at the given position, in Kelvin.
    ///
    /// # Examples
    ///
    /// ```
    /// use irb::Irb;
    /// let irb = Irb::from_path("data/image.irb").unwrap();
    /// let temperature = irb.temperature(0, 0).unwrap();
    /// ```
    pub fn temperature(&self, col: i32, row: i32) -> Result<f64> {
        let result = unsafe { irbacs_sys::getTempXY(self.handle, col, row) };
        if result == 0. {
            Err(Error::IrbacsSys(format!("getTempXY({}, {})", col, row)))
        } else {
            Ok(result)
        }
    }

    /// Returns the blackbody temperature at the given position, in Kelvin.
    ///
    /// # Examples
    ///
    /// ```
    /// use irb::Irb;
    /// let irb = Irb::from_path("data/image.irb").unwrap();
    /// let blackbody_temperature = irb.blackbody_temperature(0, 0).unwrap();
    /// ```
    pub fn blackbody_temperature(&self, col: i32, row: i32) -> Result<f64> {
        let result = unsafe { irbacs_sys::getTempBBXY(self.handle, col, row) };
        if result == 0. {
            Err(Error::IrbacsSys(format!("getTempBBXY({}, {})", col, row)))
        } else {
            Ok(result)
        }
    }

    fn param(&self, what: i32) -> Result<f64> {
        let mut param = 0.;
        if unsafe { irbacs_sys::getParam(self.handle, what, &mut param) } == 1 {
            Ok(param)
        } else {
            Err(Error::IrbacsSys(format!("getParam({})", what)))
        }
    }
}

impl Drop for Irb {
    fn drop(&mut self) {
        unsafe { irbacs_sys::unloadIRB(self.handle) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_path() {
        Irb::from_path("data/image.irb").unwrap();
        assert!(Irb::from_path("data/image.txt").is_err());
    }

    #[test]
    fn image_width() {
        let irb = Irb::from_path("data/image.irb").unwrap();
        assert_eq!(1024, irb.image_width().unwrap());
    }

    #[test]
    fn image_height() {
        let irb = Irb::from_path("data/image.irb").unwrap();
        assert_eq!(768, irb.image_height().unwrap());
    }

    #[test]
    fn frame_count() {
        let irb = Irb::from_path("data/image.irb").unwrap();
        assert_eq!(1, irb.frame_count().unwrap());
    }

    #[test]
    fn index_count() {
        let irb = Irb::from_path("data/image.irb").unwrap();
        assert_eq!(1, irb.index_count().unwrap());
    }

    #[test]
    fn temperature() {
        let irb = Irb::from_path("data/image.irb").unwrap();
        assert_relative_eq!(230.5399932861328, irb.temperature(0, 0).unwrap());
        assert!(irb.temperature(1024, 0).is_err());
        assert!(irb.temperature(0, 768).is_err());
    }

    #[test]
    fn blackbody_temperature() {
        let irb = Irb::from_path("data/image.irb").unwrap();
        assert_relative_eq!(230.5399932861328, irb.blackbody_temperature(0, 0).unwrap());
    }
}
