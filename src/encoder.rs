pub use tiff::encoder::colortype::ColorType;
pub use tiff::encoder::*;
// use crate::GeoTiffResult;
// use std::io::{Seek, Write};

// pub struct Encoder<W> where
//     W: Write + Seek {
//     pub encoder: TiffEncoder<W>
// }

// impl<W: Write + Seek> Encoder<W> {
//     pub fn new(w: W) -> GeoTiffResult<Encoder<W>> {
//         let mut encoder = TiffEncoder::new(w)?;
//         Ok(Encoder{encoder})
//     }
// }
