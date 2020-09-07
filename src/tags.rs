use crate::{error::GeoTiffError, GeoTiffResult};
use quick_xml::de::from_str;
use serde::Deserialize;
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;
pub use tiff::tags::*;

#[derive(Clone, Debug)]
pub struct GdalMetadata {
    pub scale_factor: Option<String>,
    pub offset: Option<String>,
    pub valid_range: Option<String>,
    pub units: Option<String>,
    pub min_range: Option<f64>,
    pub max_range: Option<f64>,
    pub dimensions: HashMap<String, HashMap<String, String>>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct GdalMetadataXMLItem {
    name: String,
    role: Option<String>,
    sample: Option<String>,
    #[serde(rename = "$value")]
    content: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct GdalMetadataXML {
    #[serde(rename = "Item", default)]
    items: Vec<GdalMetadataXMLItem>,
}

impl FromStr for GdalMetadata {
    type Err = GeoTiffError;

    fn from_str(medatada: &str) -> GeoTiffResult<Self> {
        let xml: GdalMetadataXML = from_str(medatada).unwrap();
        let mut res = GdalMetadata {
            scale_factor: None,
            offset: None,
            valid_range: None,
            units: None,
            min_range: None,
            max_range: None,
            dimensions: HashMap::new(),
        };
        for i in xml.items {
            match i.role.as_deref() {
                Some("dimension") => {
                    if let (Some(s), Some(c)) = (&i.sample, &i.content) {
                        if let Some(d) = res.dimensions.get_mut(&i.name) {
                            d.insert(c.into(), s.into());
                        } else {
                            let mut values = HashMap::new();
                            values.insert(c.into(), s.into());
                            res.dimensions.insert(i.name.clone(), values);
                        }
                    }
                }
                Some("dimension_units") => {
                    if let Some(c) = &i.content {
                        if let Some(d) = res.dimensions.get_mut(&i.name) {
                            d.insert("units".into(), c.into());
                        } else {
                            let mut values = HashMap::new();
                            values.insert("units".into(), c.into());
                            res.dimensions.insert(i.name.clone(), values);
                        }
                    }
                }
                _ => (),
            }
            match i.name.as_ref() {
                "minRange" => {
                    if let Some(r) = i.content {
                        res.min_range = Some(r.parse::<f64>().expect("Fail to parse min range"))
                    }
                }
                "maxRange" => {
                    if let Some(r) = i.content {
                        res.max_range = Some(r.parse::<f64>().expect("Fail to parse max range"))
                    }
                }
                "validRange" => res.valid_range = i.content,
                "units" => res.units = i.content,
                "add_offset" => res.offset = i.content,
                "scale_factor" => res.scale_factor = i.content,
                _ => (),
            }
        }

        if res.valid_range.is_none() {
            res.valid_range = if let (Some(min), Some(max)) = (res.min_range, res.max_range) {
                Some(format!("{{{},{}}}", min, max))
            } else {
                None
            };
        }
        Ok(res)
    }
}

impl fmt::Display for GdalMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut metadata_string = "<GDALMetadata>".to_string();
        if let Some(d) = &self.scale_factor {
            metadata_string = format!(
                r#"{}\n<Item name="scale_factor" sample="0">{}</Item>\n"#,
                metadata_string, d
            )
        }
        if let Some(d) = &self.valid_range {
            metadata_string = format!(
                r#"{}\n<Item name="valid_range" sample="0">{}</Item>\n"#,
                metadata_string, d
            )
        }
        if let Some(d) = &self.offset {
            metadata_string = format!(
                r#"{}\n<Item name="add_offset" sample="0">{}</Item>\n"#,
                metadata_string, d
            )
        }
        if let Some(d) = &self.units {
            metadata_string = format!(
                r#"{}\n<Item name="units" sample="0">{}</Item>\n"#,
                metadata_string, d
            )
        }
        metadata_string = format!("{}</GDALMetadata>", metadata_string);
        write!(f, "{}", metadata_string)
    }
}

// GeoTiff
pub const MODELPIXELSCALE: Tag = Tag::Unknown(33550);
pub const MODELTIEPOINT: Tag = Tag::Unknown(33922);
pub const MODELTRANSFORMATIONTAG: Tag = Tag::Unknown(34264);
pub const GEOKEYDIRECTORYTAG: Tag = Tag::Unknown(34735);
pub const GEODOUBLEPARAMSTAG: Tag = Tag::Unknown(34736);
pub const GEOASCIIPARAMSTAG: Tag = Tag::Unknown(34737);

// GDAL
pub const GDALMETADATA: Tag = Tag::Unknown(42112);
pub const GDALNODATA: Tag = Tag::Unknown(42113);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_gdalmetadata_parsing() -> GeoTiffResult<()> {
        let xml = "<GDALMetadata>
    <Item name=\"maxRange\" sample=\"0\">255.0</Item>
    <Item name=\"meanValue\" sample=\"0\">13.578433674475427</Item>
    <Item name=\"minRange\" sample=\"0\">0.0</Item>
    <Item name=\"stdDevValue\" sample=\"0\">38.21933888612938</Item>
    <Item name=\"altitude\" role=\"dimension_units\">mb</Item>
    <Item name=\"altitude\" sample=\"0\" role=\"dimension\">200</Item>
    <Item name=\"altitude\" sample=\"1\" role=\"dimension\">400</Item>
</GDALMetadata>";
        let gdal: GdalMetadataXML = from_str(xml).unwrap();
        assert_eq!(
            GdalMetadataXML {
                items: vec![
                    GdalMetadataXMLItem {
                        name: "maxRange".into(),
                        sample: Some("0".into(),),
                        role: None,
                        content: Some("255.0".into(),),
                    },
                    GdalMetadataXMLItem {
                        name: "meanValue".into(),
                        sample: Some("0".into(),),
                        role: None,
                        content: Some("13.578433674475427".into(),),
                    },
                    GdalMetadataXMLItem {
                        name: "minRange".into(),
                        sample: Some("0".into(),),
                        role: None,
                        content: Some("0.0".into(),),
                    },
                    GdalMetadataXMLItem {
                        name: "stdDevValue".into(),
                        sample: Some("0".into(),),
                        role: None,
                        content: Some("38.21933888612938".into(),),
                    },
                    GdalMetadataXMLItem {
                        name: "altitude".into(),
                        sample: None,
                        role: Some("dimension_units".into()),
                        content: Some("mb".into()),
                    },
                    GdalMetadataXMLItem {
                        name: "altitude".into(),
                        sample: Some("0".into(),),
                        role: Some("dimension".into()),
                        content: Some("200".into()),
                    },
                    GdalMetadataXMLItem {
                        name: "altitude".into(),
                        sample: Some("1".into(),),
                        role: Some("dimension".into()),
                        content: Some("400".into()),
                    },
                ],
            },
            gdal
        );

        let meta: GdalMetadata = xml.parse()?;

        assert_eq!(meta.min_range, Some(0.0));
        assert_eq!(meta.max_range, Some(255.0));
        let mut dim = HashMap::new();
        let mut alt = HashMap::new();
        alt.insert("units".into(), "mb".into());
        alt.insert("200".into(), "0".into());
        alt.insert("400".into(), "1".into());
        dim.insert("altitude".into(), alt);
        assert_eq!(meta.dimensions, dim);
        println!("{:?}", meta.dimensions);
        Ok(())
    }
}
