use std::{
    error::Error,
    sync::Arc,
    time::{Duration, Instant},
};

use daktronics_allsport_5000::{rtd_state, sports::Sport, RTDState};
use latency_graph::{LatencyGraphData, LatencySample, SerialEvent};
use tokio::{
    sync::{
        mpsc::{self, Receiver},
        Mutex,
    },
    task::JoinHandle,
};
use tokio_serial::SerialPortBuilderExt;

use crate::{mock::MockDataSource, APP_USER_AGENT};

use super::{network::put_to_server, profile::Profile};

mod latency_graph {
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
}

#[derive(Debug, Clone)]
pub struct ErrorInfo {
    pub msg: String,
    pub timestamp: Instant,
}

impl From<String> for ErrorInfo {
    fn from(value: String) -> Self {
        ErrorInfo {
            msg: value,
            timestamp: Instant::now(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum WorkerEvent {
    ErrorEvent(ErrorInfo),
    SerialEvent(SerialEvent),
    LatencySampleEvent(LatencySample, Option<String>),
}

#[derive(Debug)]
pub struct ActiveStream {
    latency_graph_data: LatencyGraphData,
    /// The payload about to undergo processing
    serialized: Arc<Mutex<Option<serde_json::Value>>>,
    /// The latest payload the server is currently holding right now
    latest_payload: Option<String>,
    errors: Vec<ErrorInfo>,

    config: Profile,

    serial_join_handle: JoinHandle<()>,
    network_processing_join_handle: JoinHandle<()>,
    pub(crate) worker_event_rx: Arc<Mutex<Receiver<WorkerEvent>>>,
}

impl ActiveStream {
    pub fn new(config: Profile, tty_path: String) -> Result<Self, Box<dyn Error>> {
        let (worker_event_tx, worker_event_rx) = mpsc::channel(255);

        /*// allow because cargo gets suspicious on Windows
        #[allow(unused_mut)]
        let mut port = tokio_serial::new(tty_path, 19200)
            .parity(tokio_serial::Parity::None)
            .open_native_async()?;

        #[cfg(unix)]
        port.set_exclusive(false)
            .expect("unable to set serial port exclusive to false");

        let rtd_state = RTDState::from_serial_stream(port, true)?;*/
        let rtd_state = RTDState::new(MockDataSource::new());

        let serialized = Arc::new(Mutex::new(None));
        let mut sport = config
            .sport_type
            .ok_or("You must specify a sport before streaming.")?
            .as_dynamic_sport(rtd_state);

        let (new_msg_tx, mut new_msg_rx) = mpsc::unbounded_channel();

        let serial_join_handle = {
            let serialized = serialized.clone();
            let worker_event_tx = worker_event_tx.clone();
            tokio::task::spawn(async move {
                loop {
                    // get the underlying rtd_state to update it
                    let has_new_data = sport.rtd_state().update_async().await;

                    match has_new_data {
                        Ok(true) => match sport.serialize_to_value() {
                            Ok(new_data) => {
                                {
                                    let mut serialized = serialized.lock().await;
                                    *serialized = Some(new_data);
                                }
                                worker_event_tx
                                    .send(WorkerEvent::SerialEvent(SerialEvent {
                                        timestamp: Instant::now(),
                                    }))
                                    .await
                                    .expect("worker event tx closed!");
                                if let Err(err) = new_msg_tx.send(()) {
                                    worker_event_tx
                                        .send(WorkerEvent::ErrorEvent(
                                            format!("failed to write to new_msg_tx: {err}").into(),
                                        ))
                                        .await
                                        .expect("worker event tx closed!")
                                }
                            }
                            Err(err) => worker_event_tx
                                .send(WorkerEvent::ErrorEvent(
                                    format!("couldn't serialize sport: {err}").into(),
                                ))
                                .await
                                .expect("worker event tx closed!"),
                        },
                        Ok(false) => {
                            // don't bother to update if nothing changed
                        }
                        Err(err) => worker_event_tx
                            .send(WorkerEvent::ErrorEvent(
                                format!("failed to update RTD state: {err}").into(),
                            ))
                            .await
                            .expect("worker event tx closed!"),
                    }
                    tokio::task::yield_now().await;
                }
            })
        };

        let network_processing_join_handle = {
            let serialized = serialized.clone();
            let data_stream_url = config.data_stream_url.clone();
            let mapping = config.mapping.clone();
            let client = reqwest::Client::builder()
                .user_agent(APP_USER_AGENT)
                .http2_keep_alive_while_idle(true)
                .http2_keep_alive_interval(Some(Duration::from_secs(1)))
                .build()?;
            let worker_event_tx = worker_event_tx.clone();
            tokio::task::spawn(async move {
                // pre-connect to the server
                if let Err(err) = client.head(&data_stream_url).send().await {
                    worker_event_tx
                        .send(WorkerEvent::ErrorEvent(
                            format!("server pre-connect failed: {err:?}").into(),
                        ))
                        .await
                        .expect("worker event tx closed!")
                }

                loop {
                    let serialized = { serialized.lock().await.take() };
                    if let Some(value) = serialized {
                        match mapping.map(&value) {
                            Ok(serialized) => {
                                let pretty_stringified =
                                    serde_json::to_string_pretty(&serialized).ok();
                                if config.multiple_requests {
                                    let client = client.clone();
                                    let data_stream_url = data_stream_url.clone();
                                    let worker_event_tx = worker_event_tx.clone();
                                    // TODO: this might be a performance bottleneck; fix?
                                    tokio::task::spawn(async move {
                                        match put_to_server(
                                            &client,
                                            &data_stream_url,
                                            serialized.to_string(),
                                        )
                                        .await
                                        {
                                            Err(err) => worker_event_tx
                                                .send(WorkerEvent::ErrorEvent(format!("server PUT failed (multiple_requests=true): {err:?}").into()))
                                                .await
                                                .expect("worker event tx closed!"),
                                            Ok(latency) => worker_event_tx
                                                .send(WorkerEvent::LatencySampleEvent(
                                                    LatencySample {
                                                        timestamp: Instant::now(),
                                                        latency,
                                                    },
                                                    pretty_stringified,
                                                ))
                                                .await
                                                .expect("worker event tx closed!"),
                                        }
                                    });
                                } else {
                                    match put_to_server(
                                        &client,
                                        &data_stream_url,
                                        serialized.to_string(),
                                    )
                                    .await
                                    {
                                        Err(err) => worker_event_tx
                                            .send(WorkerEvent::ErrorEvent(format!("server PUT failed (multiple_requests=false): {err:?}").into()))
                                            .await
                                            .expect("worker event tx closed!"),
                                        Ok(latency) => worker_event_tx
                                            .send(WorkerEvent::LatencySampleEvent(
                                                LatencySample {
                                                    timestamp: Instant::now(),
                                                    latency,
                                                },
                                                pretty_stringified,
                                            ))
                                            .await
                                            .expect("worker event tx closed!"),
                                    }
                                }
                            }
                            Err(err) => worker_event_tx
                                .send(WorkerEvent::ErrorEvent(
                                    format!("failed to map from serial stream to network: {err}")
                                        .into(),
                                ))
                                .await
                                .expect("worker event tx closed!"),
                        }
                    }
                    if new_msg_rx.is_empty() {
                        // if it's empty right now, wait for the next signal
                        new_msg_rx.recv().await;
                    }
                    // flush the signal stream and go again
                    while !new_msg_rx.is_empty() {
                        new_msg_rx.recv().await;
                    }
                }
            })
        };

        Ok(Self {
            latency_graph_data: LatencyGraphData {
                samples: vec![],
                serial_events: vec![],
            },
            serialized,
            latest_payload: None,
            errors: vec![],
            // config: Config {
            //     data_stream_url: "https://datastreams.singular.live/whatever".into(),
            //     mapping: Mapping { items: vec![] },
            //     multiple_requests: true,
            //     sport: DynamicSport::Basketball,
            //     subcomp_name: "Basketball Bug".into(),
            // },
            config,
            serial_join_handle,
            network_processing_join_handle,
            worker_event_rx: Arc::new(Mutex::new(worker_event_rx)),
        })
    }

    pub async fn update_stats(&mut self) {
        {
            let mut rx = self.worker_event_rx.lock().await;
            while !rx.is_empty() {
                let event = rx.recv().await;
                if let Some(event) = event {
                    match event {
                        WorkerEvent::ErrorEvent(err) => self.errors.push(err),
                        WorkerEvent::LatencySampleEvent(sample, latest_payload) => {
                            self.latest_payload = latest_payload;
                            self.latency_graph_data.samples.push(sample)
                        }
                        WorkerEvent::SerialEvent(event) => {
                            self.latency_graph_data.serial_events.push(event)
                        }
                    }
                }
            }
        }
        self.purge_old_data(Duration::from_secs(60 * 5), 20)
    }

    pub fn update_from_events(&mut self, events: Vec<WorkerEvent>) {
        for event in events {
            match event {
                WorkerEvent::ErrorEvent(err) => self.errors.push(err),
                WorkerEvent::LatencySampleEvent(sample, latest_payload) => {
                    self.latest_payload = latest_payload;
                    self.latency_graph_data.samples.push(sample)
                }
                WorkerEvent::SerialEvent(event) => {
                    self.latency_graph_data.serial_events.push(event)
                }
            }
        }
        self.purge_old_data(Duration::from_secs(60 * 5), 20)
    }

    pub async fn read_events(&mut self, limit: usize) -> Vec<WorkerEvent> {
        let mut buffer = Vec::new();
        self.worker_event_rx
            .lock()
            .await
            .recv_many(&mut buffer, limit)
            .await;
        buffer
    }

    pub fn latency_graph_data(&self) -> &LatencyGraphData {
        &self.latency_graph_data
    }

    pub fn latest_payload(&self) -> Option<&str> {
        self.latest_payload.as_ref().map(String::as_str)
    }

    pub fn errors(&self) -> &[ErrorInfo] {
        &self.errors
    }

    pub fn clear_errors(&mut self) {
        self.errors.clear()
    }

    /// Deletes all latency graph data with ages more than the specific duration
    pub fn purge_old_data(&mut self, keep_graph: Duration, keep_errors: usize) {
        self.latency_graph_data
            .samples
            .retain(|x| x.timestamp.elapsed() < keep_graph);
        self.latency_graph_data
            .serial_events
            .retain(|x| x.timestamp.elapsed() < keep_graph);
        self.errors = self
            .errors
            .split_off(self.errors.len().saturating_sub(keep_errors));
    }
}

impl Drop for ActiveStream {
    fn drop(&mut self) {
        self.serial_join_handle.abort();
        self.network_processing_join_handle.abort();
    }
}
