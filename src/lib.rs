// #[macro_use]
// extern crate tiff;

pub mod decoder;
pub mod encoder;
pub mod error;
pub mod tags;

pub use tiff::*;

pub use error::GeoTiffResult;
