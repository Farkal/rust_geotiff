pub use tiff::tags::*;
use std::fmt;

#[derive(Clone, Debug)]
pub struct GdalMetadata {
    pub scale_factor: Option<String>,
    pub offset: Option<String>,
    pub valid_range: Option<String>,
    pub units: Option<String>,
    pub min_range: Option<f64>,
    pub max_range: Option<f64>,
}

impl fmt::Display for GdalMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut metadata_string = "<GDALMetadata>".to_string();
        if let Some(d) = &self.scale_factor {
            metadata_string = format!(r#"{}\n<Item name="scale_factor" sample="0">{}</Item>\n"#, metadata_string, d)
        }
        if let Some(d) = &self.valid_range {
            metadata_string = format!(r#"{}\n<Item name="valid_range" sample="0">{}</Item>\n"#, metadata_string, d)
        }
        if let Some(d) = &self.offset {
            metadata_string = format!(r#"{}\n<Item name="add_offset" sample="0">{}</Item>\n"#, metadata_string, d)
        }
        if let Some(d) = &self.units {
            metadata_string = format!(r#"{}\n<Item name="units" sample="0">{}</Item>\n"#, metadata_string, d)
        }
        metadata_string = format!("{}</GDALMetadata>", metadata_string);
        write!(f, "{}", metadata_string)
    }
}


// GeoTiff
pub const MODELPIXELSCALE : Tag = Tag::Unknown(33550);
pub const MODELTIEPOINT : Tag = Tag::Unknown(33922);
pub const MODELTRANSFORMATIONTAG : Tag = Tag::Unknown(34264);
pub const GEOKEYDIRECTORYTAG : Tag = Tag::Unknown(34735);
pub const GEODOUBLEPARAMSTAG : Tag = Tag::Unknown(34736);
pub const GEOASCIIPARAMSTAG : Tag = Tag::Unknown(34737);

// GDAL
pub const GDALMETADATA: Tag = Tag::Unknown(42112);
pub const GDALNODATA: Tag = Tag::Unknown(42113);
