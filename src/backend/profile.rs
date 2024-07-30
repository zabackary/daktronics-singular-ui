use serde::{Deserialize, Serialize};

use super::{mapping::Mapping, sports::DynamicSportType};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub data_stream_url: String,
    pub subcomp_name: String,
    pub sport_type: Option<DynamicSportType>,
    pub multiple_requests: bool,
    pub mapping: Mapping,
    pub name: String,
}

impl Default for Profile {
    fn default() -> Self {
        Profile {
            data_stream_url: Default::default(),
            subcomp_name: Default::default(),
            sport_type: Default::default(),
            multiple_requests: true,
            mapping: Default::default(),
            name: "New profile".to_owned(),
        }
    }
}
