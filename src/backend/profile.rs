use serde::{Deserialize, Serialize};

use super::{mapping::Mapping, sports::DynamicSportType};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub data_stream_url: String,
    pub subcomp_name: String,
    pub sport_type: DynamicSportType,
    pub multiple_requests: bool,
    pub mapping: Mapping,
}
