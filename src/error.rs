use thiserror::Error;
use tiff::TiffError;

pub type GeoTiffResult<T> = Result<T, GeoTiffError>;

#[derive(Debug, Error)]
pub enum GeoTiffError {
    #[error("Impossible to decode file")]
    DecodingFailed,
    #[error("Tiff Error")]
    TError(#[from] TiffError),
    

}