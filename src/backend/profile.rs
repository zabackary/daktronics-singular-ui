use serde::{de::Error, Deserialize, Serialize};
use serde_json::Value;

use super::{mapping::Mapping, sports::DynamicSportType};

pub type Profile = ProfileV2;
pub type ProfileCompositionMapping = ProfileV2CompositionMapping;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileV2 {
    /// The version of the program used to create the profile.
    pub name: String,
    pub ui_version: String,
    pub data_stream_url: String,
    pub multiple_requests: bool,
    pub exclude_incomplete_data: bool,
    pub sport_type: Option<DynamicSportType>,
    pub mappings: Vec<ProfileV2CompositionMapping>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProfileV2CompositionMapping {
    pub subcomp_name: String,
    pub mapping: Mapping,
    pub enabled_checkbox_name: Option<String>,
}

impl Default for ProfileV2 {
    fn default() -> Self {
        Self {
            ui_version: env!("CARGO_PKG_VERSION").to_owned(),
            data_stream_url: Default::default(),
            sport_type: Default::default(),
            multiple_requests: true,
            exclude_incomplete_data: true,
            name: "New profile".to_owned(),
            mappings: vec![],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileV1 {
    /// The version of the program used to create the profile.
    pub ui_version: String,
    pub data_stream_url: String,
    pub subcomp_name: String,
    pub sport_type: Option<DynamicSportType>,
    pub multiple_requests: bool,
    pub exclude_incomplete_data: bool,
    pub mapping: Mapping,
    pub name: String,
}

impl Default for ProfileV1 {
    fn default() -> Self {
        Self {
            ui_version: env!("CARGO_PKG_VERSION").to_owned(),
            data_stream_url: Default::default(),
            sport_type: Default::default(),
            multiple_requests: true,
            exclude_incomplete_data: true,
            name: "New profile".to_owned(),
            mapping: Default::default(),
            subcomp_name: Default::default(),
        }
    }
}

impl Into<ProfileV2> for ProfileV1 {
    fn into(self) -> ProfileV2 {
        ProfileV2 {
            name: self.name,
            ui_version: env!("CARGO_PKG_VERSION").to_owned(),
            data_stream_url: self.data_stream_url,
            multiple_requests: self.multiple_requests,
            exclude_incomplete_data: self.exclude_incomplete_data,
            sport_type: self.sport_type,
            mappings: vec![ProfileV2CompositionMapping {
                enabled_checkbox_name: None,
                mapping: self.mapping,
                subcomp_name: self.subcomp_name,
            }],
        }
    }
}

impl Profile {
    pub fn export(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&self)
    }

    pub fn import(source: &str) -> Result<Self, serde_json::Error> {
        let value = serde_json::from_str::<Value>(source)?;
        let version = value
            .as_object()
            .ok_or(serde_json::Error::invalid_type(
                serde::de::Unexpected::Other("not an object"),
                &"object with key 'ui_version'",
            ))?
            .get_key_value("ui_version")
            .ok_or(serde_json::Error::missing_field("ui_version"))?
            .1
            .as_str()
            .ok_or(serde_json::Error::invalid_type(
                serde::de::Unexpected::Other("not a string"),
                &"string",
            ))?;
        Ok(match version.split(".").next().unwrap() {
            "1" => serde_json::from_value::<ProfileV1>(value)?.into(),
            // "2" => serde_json::from_value::<ProfileV2>(value)?.into(),
            _ => Err(serde_json::Error::invalid_value(
                serde::de::Unexpected::Other("unknown version"),
                &"1.0 or 2.0",
            ))?,
        })
    }
}
