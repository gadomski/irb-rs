use Result;

pub struct FixedLengthString {
    bytes: Vec<u8>,
}

impl FixedLengthString {
    pub fn new(length: usize) -> FixedLengthString {
        FixedLengthString { bytes: vec![0; length] }
    }

    pub fn to_string(&self) -> Result<String> {
        use Error;

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
