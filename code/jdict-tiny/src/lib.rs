#![allow(dead_code)]

pub mod num_compression;

pub trait FromByteBuffer<'a> {
    fn from_bytes(buffer: &'a [u8]) -> Self;
}

pub struct Writing<'a> {
    pub value: &'a str,
}
impl<'a> FromByteBuffer<'a> for Writing<'a> {
    fn from_bytes(buffer: &'a [u8]) -> Self {
        Writing {
            value: std::str::from_utf8(buffer).unwrap_or_default(),
        }
    }
}

pub struct Reading<'a> {
    value: &'a str,
}
pub struct Sense<'a> {
    value: &'a str,
}

pub struct Entry {

}

pub struct Dictionary {

}
