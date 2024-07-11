use super::{mapping::Mapping, sports::DynamicSportType};

#[derive(Debug, Clone)]
pub struct Config {
    pub data_stream_url: String,
    pub subcomp_name: String,
    pub sport_type: DynamicSportType,
    pub multiple_requests: bool,
    pub mapping: Mapping,
}
