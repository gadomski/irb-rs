#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_is_ok() {
        let mut mainver = 0;
        let mut subver = 0;
        assert_eq!(1, unsafe { version(&mut mainver, &mut subver) });
    }
}
