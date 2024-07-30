use std::fmt::{Debug, Display};

use daktronics_allsport_5000::{
    rtd_state::data_source::RTDStateDataSource,
    sports::{
        auto_racing::AutoRacingSport,
        baseball::BaseballSport,
        basketball::BasketballSport,
        cricket::CricketSport,
        event_counter::{
            day_time::DateTimeEventCounterSport, external_input::ExternalInputEventCountdownSport,
            time_base::TimeBaseEventCounterSport,
        },
        football::FootballSport,
        hockey_lacrosse::HockeyLacrosseSport,
        judo::JudoSport,
        karate::KarateSport,
        lane_timer::LaneTimerSport,
        pitch_and_speed::PitchAndSpeedSport,
        rodeo::RodeoSport,
        soccer::SoccerSport,
        strike_out_count::StrikeOutCountSport,
        taekwondo::TaekwondoSport,
        tennis::TennisSport,
        track::TrackSport,
        volleyball::VolleyballSport,
        water_polo::WaterPoloSport,
        wrestling::WrestlingSport,
    },
    RTDState,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum DynamicSportType {
    AutoRacing,
    Baseball,
    Basketball,
    Cricket,
    EventCounterDayTime,
    EventCounterExternalInput,
    EventCounterTimeBase,
    Football,
    HockeyLacrosse,
    Judo,
    Karate,
    LaneTimer,
    PitchAndSpeed,
    Rodeo,
    Soccer,
    StrikeOutCount,
    Taekwondo,
    Tennis,
    Track,
    Volleyball,
    WaterPolo,
    Wrestling,
}

impl Display for DynamicSportType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            DynamicSportType::AutoRacing => "Auto Racing",
            DynamicSportType::Baseball => "Baseball",
            DynamicSportType::Basketball => "Basketball",
            DynamicSportType::Cricket => "Cricket",
            DynamicSportType::EventCounterDayTime => "Event Counter: Day/Time",
            DynamicSportType::EventCounterExternalInput => "Event Counter: External Input",
            DynamicSportType::EventCounterTimeBase => "Event Counter: Time Base",
            DynamicSportType::Football => "Football",
            DynamicSportType::HockeyLacrosse => "Hockey/Lacrosse",
            DynamicSportType::Judo => "Judo",
            DynamicSportType::Karate => "Karate",
            DynamicSportType::LaneTimer => "Lane Timer",
            DynamicSportType::PitchAndSpeed => "Pitch and Speed",
            DynamicSportType::Rodeo => "Rodeo",
            DynamicSportType::Soccer => "Soccer",
            DynamicSportType::StrikeOutCount => "StrikeOutCount",
            DynamicSportType::Taekwondo => "Taekwondo",
            DynamicSportType::Tennis => "Tennis",
            DynamicSportType::Track => "Track",
            DynamicSportType::Volleyball => "Volleyball",
            DynamicSportType::WaterPolo => "Water Polo",
            DynamicSportType::Wrestling => "Wrestling",
        })
    }
}

impl DynamicSportType {
    pub const ALL: [DynamicSportType; 22] = [
        DynamicSportType::AutoRacing,
        DynamicSportType::Baseball,
        DynamicSportType::Basketball,
        DynamicSportType::Cricket,
        DynamicSportType::EventCounterDayTime,
        DynamicSportType::EventCounterExternalInput,
        DynamicSportType::EventCounterTimeBase,
        DynamicSportType::Football,
        DynamicSportType::HockeyLacrosse,
        DynamicSportType::Judo,
        DynamicSportType::Karate,
        DynamicSportType::LaneTimer,
        DynamicSportType::PitchAndSpeed,
        DynamicSportType::Rodeo,
        DynamicSportType::Soccer,
        DynamicSportType::StrikeOutCount,
        DynamicSportType::Taekwondo,
        DynamicSportType::Tennis,
        DynamicSportType::Track,
        DynamicSportType::Volleyball,
        DynamicSportType::WaterPolo,
        DynamicSportType::Wrestling,
    ];

    pub fn as_dynamic_sport<DS: RTDStateDataSource>(
        &self,
        rtd_state: RTDState<DS>,
    ) -> DynamicSport<DS> {
        match self {
            DynamicSportType::AutoRacing => {
                DynamicSport::AutoRacing(AutoRacingSport::new(rtd_state))
            }
            DynamicSportType::Baseball => DynamicSport::Baseball(BaseballSport::new(rtd_state)),
            DynamicSportType::Basketball => {
                DynamicSport::Basketball(BasketballSport::new(rtd_state))
            }
            DynamicSportType::Cricket => DynamicSport::Cricket(CricketSport::new(rtd_state)),
            DynamicSportType::EventCounterDayTime => {
                DynamicSport::EventCounterDayTime(DateTimeEventCounterSport::new(rtd_state))
            }
            DynamicSportType::EventCounterExternalInput => DynamicSport::EventCounterExternalInput(
                ExternalInputEventCountdownSport::new(rtd_state),
            ),
            DynamicSportType::EventCounterTimeBase => {
                DynamicSport::EventCounterTimeBase(TimeBaseEventCounterSport::new(rtd_state))
            }
            DynamicSportType::Football => DynamicSport::Football(FootballSport::new(rtd_state)),
            DynamicSportType::HockeyLacrosse => {
                DynamicSport::HockeyLacrosse(HockeyLacrosseSport::new(rtd_state))
            }
            DynamicSportType::Judo => DynamicSport::Judo(JudoSport::new(rtd_state)),
            DynamicSportType::Karate => DynamicSport::Karate(KarateSport::new(rtd_state)),
            DynamicSportType::LaneTimer => DynamicSport::LaneTimer(LaneTimerSport::new(rtd_state)),
            DynamicSportType::PitchAndSpeed => {
                DynamicSport::PitchAndSpeed(PitchAndSpeedSport::new(rtd_state))
            }
            DynamicSportType::Rodeo => DynamicSport::Rodeo(RodeoSport::new(rtd_state)),
            DynamicSportType::Soccer => DynamicSport::Soccer(SoccerSport::new(rtd_state)),
            DynamicSportType::StrikeOutCount => {
                DynamicSport::StrikeOutCount(StrikeOutCountSport::new(rtd_state))
            }
            DynamicSportType::Taekwondo => DynamicSport::Taekwondo(TaekwondoSport::new(rtd_state)),
            DynamicSportType::Tennis => DynamicSport::Tennis(TennisSport::new(rtd_state)),
            DynamicSportType::Track => DynamicSport::Track(TrackSport::new(rtd_state)),
            DynamicSportType::Volleyball => {
                DynamicSport::Volleyball(VolleyballSport::new(rtd_state))
            }
            DynamicSportType::WaterPolo => DynamicSport::WaterPolo(WaterPoloSport::new(rtd_state)),
            DynamicSportType::Wrestling => DynamicSport::Wrestling(WrestlingSport::new(rtd_state)),
        }
    }
}

pub enum DynamicSport<DS: RTDStateDataSource> {
    AutoRacing(AutoRacingSport<DS>),
    Baseball(BaseballSport<DS>),
    Basketball(BasketballSport<DS>),
    Cricket(CricketSport<DS>),
    EventCounterDayTime(DateTimeEventCounterSport<DS>),
    EventCounterExternalInput(ExternalInputEventCountdownSport<DS>),
    EventCounterTimeBase(TimeBaseEventCounterSport<DS>),
    Football(FootballSport<DS>),
    HockeyLacrosse(HockeyLacrosseSport<DS>),
    Judo(JudoSport<DS>),
    Karate(KarateSport<DS>),
    LaneTimer(LaneTimerSport<DS>),
    PitchAndSpeed(PitchAndSpeedSport<DS>),
    Rodeo(RodeoSport<DS>),
    Soccer(SoccerSport<DS>),
    StrikeOutCount(StrikeOutCountSport<DS>),
    Taekwondo(TaekwondoSport<DS>),
    Tennis(TennisSport<DS>),
    Track(TrackSport<DS>),
    Volleyball(VolleyballSport<DS>),
    WaterPolo(WaterPoloSport<DS>),
    Wrestling(WrestlingSport<DS>),
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
            DynamicSport::AutoRacing(x) => x.rtd_state(),
            DynamicSport::Baseball(x) => x.rtd_state(),
            DynamicSport::Basketball(x) => x.rtd_state(),
            DynamicSport::Cricket(x) => x.rtd_state(),
            DynamicSport::EventCounterDayTime(x) => x.rtd_state(),
            DynamicSport::EventCounterExternalInput(x) => x.rtd_state(),
            DynamicSport::EventCounterTimeBase(x) => x.rtd_state(),
            DynamicSport::Football(x) => x.rtd_state(),
            DynamicSport::HockeyLacrosse(x) => x.rtd_state(),
            DynamicSport::Judo(x) => x.rtd_state(),
            DynamicSport::Karate(x) => x.rtd_state(),
            DynamicSport::LaneTimer(x) => x.rtd_state(),
            DynamicSport::PitchAndSpeed(x) => x.rtd_state(),
            DynamicSport::Rodeo(x) => x.rtd_state(),
            DynamicSport::Soccer(x) => x.rtd_state(),
            DynamicSport::StrikeOutCount(x) => x.rtd_state(),
            DynamicSport::Taekwondo(x) => x.rtd_state(),
            DynamicSport::Tennis(x) => x.rtd_state(),
            DynamicSport::Track(x) => x.rtd_state(),
            DynamicSport::Volleyball(x) => x.rtd_state(),
            DynamicSport::WaterPolo(x) => x.rtd_state(),
            DynamicSport::Wrestling(x) => x.rtd_state(),
        }
    }
}

impl<DS: RTDStateDataSource> DynamicSport<DS> {
    pub fn serialize_to_value(&self) -> serde_json::Result<serde_json::Value> {
        match self {
            DynamicSport::AutoRacing(x) => serde_json::to_value(x),
            DynamicSport::Baseball(x) => serde_json::to_value(x),
            DynamicSport::Basketball(x) => serde_json::to_value(x),
            DynamicSport::Cricket(x) => serde_json::to_value(x),
            DynamicSport::EventCounterDayTime(x) => serde_json::to_value(x),
            DynamicSport::EventCounterExternalInput(x) => serde_json::to_value(x),
            DynamicSport::EventCounterTimeBase(x) => serde_json::to_value(x),
            DynamicSport::Football(x) => serde_json::to_value(x),
            DynamicSport::HockeyLacrosse(x) => serde_json::to_value(x),
            DynamicSport::Judo(x) => serde_json::to_value(x),
            DynamicSport::Karate(x) => serde_json::to_value(x),
            DynamicSport::LaneTimer(x) => serde_json::to_value(x),
            DynamicSport::PitchAndSpeed(x) => serde_json::to_value(x),
            DynamicSport::Rodeo(x) => serde_json::to_value(x),
            DynamicSport::Soccer(x) => serde_json::to_value(x),
            DynamicSport::StrikeOutCount(x) => serde_json::to_value(x),
            DynamicSport::Taekwondo(x) => serde_json::to_value(x),
            DynamicSport::Tennis(x) => serde_json::to_value(x),
            DynamicSport::Track(x) => serde_json::to_value(x),
            DynamicSport::Volleyball(x) => serde_json::to_value(x),
            DynamicSport::WaterPolo(x) => serde_json::to_value(x),
            DynamicSport::Wrestling(x) => serde_json::to_value(x),
        }
    }
}
