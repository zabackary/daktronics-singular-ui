use std::{error::Error, fmt::Display, time::Duration};

use daktronics_allsport_5000::rtd_state::data_source::RTDStateDataSource;

#[derive(Debug)]
pub struct MockDataSource {}

impl MockDataSource {
    pub fn new() -> Self {
        MockDataSource {}
    }
}

impl RTDStateDataSource for MockDataSource {
    type Error = MockDataSourceError;

    fn read_packet(
        &mut self,
    ) -> Result<Option<daktronics_allsport_5000::packet::Packet>, Self::Error> {
        Err(MockDataSourceError::Unsupported)
    }

    async fn read_packet_async(
        &mut self,
    ) -> Result<Option<daktronics_allsport_5000::packet::Packet>, Self::Error> {
        tokio::time::sleep(Duration::from_millis(900)).await;
        Ok(Some(
            daktronics_allsport_5000::packet::Packet::try_from(
                &b"first part\x010042101\x0211:1111:11.1 \x04"[..],
            )
            .unwrap(),
        ))
    }
}

#[derive(Debug)]
pub enum MockDataSourceError {
    Unsupported,
}

impl Display for MockDataSourceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MockDataSourceError::Unsupported => write!(f, "unsupported on mock"),
        }
    }
}

impl Error for MockDataSourceError {}
