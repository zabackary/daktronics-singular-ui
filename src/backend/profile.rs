use serde::{Deserialize, Serialize};

use super::{mapping::Mapping, sports::DynamicSportType};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Profile {
    pub data_stream_url: String,
    pub subcomp_name: String,
    pub sport_type: Option<DynamicSportType>,
    pub multiple_requests: bool,
    pub mapping: Mapping,
    pub name: String,
}
