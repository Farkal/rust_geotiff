// #[macro_use]
// extern crate tiff;

pub mod decoder;
pub mod encoder;
pub mod tags;
pub mod error;

pub use tiff::*;

pub use error::GeoTiffResult;