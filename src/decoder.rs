pub use tiff::decoder::*;
// use crate::GeoTiffResult;
use std::io::{Seek, Read};
use crate::tags::{self, Tag};
use tiff::{TiffResult, TiffError, TiffUnsupportedError};
// #[derive(Debug)]
// pub struct Decoder<R> where
//     R: Read + Seek {
//     pub decoder: TiffDecoder<R>
// }

// impl<R: Read + Seek> Decoder<R> {
//     pub fn new(r: R) -> GeoTiffResult<Decoder<R>> {
//         let mut decoder = TiffDecoder::new(r)?;
//         Ok(Decoder{decoder})
//     }
// }

pub fn get_offsets<R: Read + Seek>(decoder: &mut Decoder<R>) -> TiffResult<Vec<u32>> {
    // If there is no strip  offset it's tiled
    if decoder.get_tag_u32(Tag::StripOffsets).is_err() {
        decoder.get_tag_u32_vec(Tag::TileOffsets)
    } else {
        decoder.get_tag_u32_vec(Tag::StripOffsets)
    }
}

pub fn get_byte_counts<R: Read + Seek>(decoder: &mut Decoder<R>) -> TiffResult<Vec<u32>> {
    // If there is no strip  offset it's tiled
    if decoder.get_tag_u32(Tag::StripOffsets).is_err() {
        decoder.get_tag_u32_vec(Tag::TileByteCounts)
    } else {
        decoder.get_tag_u32_vec(Tag::StripByteCounts)
    }
}

pub fn get_origin<R: Read + Seek>(decoder: &mut Decoder<R>) -> TiffResult<[f64; 2]> {
    match decoder.get_tag_f64_vec(tags::MODELTIEPOINT) {
        Ok(ref tie_points) if tie_points.len() == 6 => Ok([tie_points[3], tie_points[4]]),
        _ => {
            if let Ok(model_transformation) =
                decoder.get_tag_f64_vec(tags::MODELTRANSFORMATIONTAG)
            {
                Ok([model_transformation[3], model_transformation[7]])
            } else {
                Err(TiffError::UnsupportedError(
                    TiffUnsupportedError::UnknownInterpretation,
                ))
            }
        }
    }
}

pub fn get_resolution<R: Read + Seek>(decoder: &mut Decoder<R>) -> TiffResult<[f64; 2]> {
        match decoder.get_tag_f64_vec(tags::MODELPIXELSCALE) {
            Ok(mps) => Ok([mps[0], -mps[1]]),
            _ => {
                if let Ok(model_transformation) =
                    decoder.get_tag_f64_vec(tags::MODELTRANSFORMATIONTAG)
                {
                    Ok([model_transformation[0], model_transformation[5]])
                } else {
                    Err(TiffError::UnsupportedError(
                        TiffUnsupportedError::UnknownInterpretation,
                    ))
                }
            }
        }
    }