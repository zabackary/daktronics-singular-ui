use serde::{Deserialize, Serialize};

use super::{mapping::Mapping, sports::DynamicSportType};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
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

impl Default for Profile {
    fn default() -> Self {
        Profile {
            ui_version: env!("CARGO_PKG_VERSION").to_owned(),
            data_stream_url: Default::default(),
            subcomp_name: Default::default(),
            sport_type: Default::default(),
            multiple_requests: true,
            exclude_incomplete_data: true,
            mapping: Default::default(),
            name: "New profile".to_owned(),
        }
    }
}
