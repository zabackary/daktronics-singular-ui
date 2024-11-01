use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct LatencySample {
    pub timestamp: Instant,
    pub latency: Duration,
}

#[derive(Debug, Clone)]
pub struct SerialEvent {
    pub timestamp: Instant,
}

#[derive(Debug, Clone)]
pub struct LatencyGraphData {
    pub samples: Vec<LatencySample>,
    pub serial_events: Vec<SerialEvent>,
}
