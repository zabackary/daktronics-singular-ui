use std::fmt::Debug;

use daktronics_allsport_5000::{
    rtd_state::data_source::RTDStateDataSource, sports::basketball::BasketballSport, RTDState,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum DynamicSportType {
    Basketball,
}

impl DynamicSportType {
    pub fn as_dynamic_sport<DS: RTDStateDataSource>(
        &self,
        rtd_state: RTDState<DS>,
    ) -> DynamicSport<DS> {
        match self {
            DynamicSportType::Basketball => {
                DynamicSport::Basketball(BasketballSport::new(rtd_state))
            }
        }
    }
}

pub enum DynamicSport<DS: RTDStateDataSource> {
    Basketball(BasketballSport<DS>),
}

impl<DS: RTDStateDataSource> Debug for DynamicSport<DS> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DynamicSport")
    }
}

impl<DS: RTDStateDataSource> daktronics_allsport_5000::sports::Sport<DS> for DynamicSport<DS> {
    fn name(&self) -> &'static str {
        "Dynamic Sport"
    }

    fn rtd_state(&mut self) -> &mut daktronics_allsport_5000::rtd_state::RTDState<DS> {
        match self {
            DynamicSport::Basketball(x) => x.rtd_state(),
        }
    }
}

impl<DS: RTDStateDataSource> DynamicSport<DS> {
    pub fn serialize_to_value(&self) -> serde_json::Result<serde_json::Value> {
        match self {
            DynamicSport::Basketball(x) => serde_json::to_value(x),
        }
    }
}
