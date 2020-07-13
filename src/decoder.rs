use tiff::decoder::{Decoder as TiffDecoder};
use crate::GeoTiffResult;
use std::io::{Seek, Read};

#[derive(Debug)]
pub struct Decoder<R> where
    R: Read + Seek {
    decoder: TiffDecoder<R>
}

impl<R: Read + Seek> Decoder<R> {
    pub fn new(r: R) -> GeoTiffResult<Decoder<R>> {
        let mut decoder = TiffDecoder::new(r)?;
        Ok(Decoder{decoder})
    }
}