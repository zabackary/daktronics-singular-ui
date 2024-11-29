pub mod latency_graph;

use std::{
    error::Error,
    sync::Arc,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

use daktronics_allsport_5000::{sports::Sport, RTDState};
use latency_graph::{LatencyGraphData, LatencySample, SerialEvent};
use tokio::{
    select,
    sync::{
        mpsc::{self, Receiver},
        Mutex,
    },
    task::JoinHandle,
};
use tokio_serial::SerialPortBuilderExt;

use crate::{backend::serializer::serialize_mappings, APP_USER_AGENT};

use super::{network::put_to_server, profile::Profile};

const MAX_SERIAL_PACKET_DELAY: u64 = 3000;

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
    LatencySampleEvent(LatencySample, Option<String>, usize),
}

#[derive(Debug)]
pub struct ActiveStream {
    latency_graph_data: LatencyGraphData,
    /// The latest payload the server is currently holding right now
    latest_payload: Option<String>,
    latest_payload_size: Option<usize>,
    errors: Vec<ErrorInfo>,

    serial_join_handle: JoinHandle<()>,
    network_processing_join_handle: JoinHandle<()>,
    pub(crate) worker_event_rx: Arc<Mutex<Receiver<WorkerEvent>>>,
}

impl ActiveStream {
    pub fn new(profile: Profile, tty_path: String) -> Result<Self, Box<dyn Error>> {
        eprintln!(
            "INFO stream Creating stream bound to {} with profile {}",
            tty_path, profile.name
        );
        let (worker_event_tx, worker_event_rx) = mpsc::channel(255);

        // allow because cargo gets suspicious on Windows
        #[allow(unused_mut)]
        let mut port = tokio_serial::new(tty_path, 19200)
            .parity(tokio_serial::Parity::None)
            .open_native_async()?;

        #[cfg(unix)]
        port.set_exclusive(false)
            .expect("unable to set serial port exclusive to false");

        let rtd_state = RTDState::from_serial_stream(port, true)?;

        let serialized = Arc::new(Mutex::new(None));
        let mut sport = profile
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
                    let has_new_data = select! {
                        result = sport.rtd_state().update_async() => {
                            result.map_err(Some)
                        },
                        () = tokio::time::sleep(Duration::from_millis(MAX_SERIAL_PACKET_DELAY)) => {
                            Err(None)
                        }
                    };

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
                                    format!("couldn't serialize sport: {err}\nThis might be caused by the DSU sport type not matching the Daktronics sport type").into(),
                                ))
                                .await
                                .expect("worker event tx closed!"),
                        },
                        Ok(false) => {
                            // don't bother to update if nothing changed
                        }
                        Err(Some(err)) => worker_event_tx
                            .send(WorkerEvent::ErrorEvent(
                                format!("failed to update RTD state: {err}").into(),
                            ))
                            .await
                            .expect("worker event tx closed!"),
                        Err(None) => worker_event_tx
                            .send(WorkerEvent::ErrorEvent("timeout when waiting for new score data from the serial connection".to_owned().into()))
                            .await
                            .expect("worker event tx closed!"),
                    }
                    tokio::task::yield_now().await;
                }
            })
        };

        let network_processing_join_handle = {
            let serialized = serialized.clone();
            let data_stream_url = profile.data_stream_url.clone();
            let mappings = profile.mappings.clone();
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
                    let timestamp = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .expect("what are you doing with your clock set so early?!")
                        .as_millis();
                    if let Some(value) = serialized {
                        match serialize_mappings(
                            &mappings,
                            &value,
                            profile.exclude_incomplete_data,
                            Some(timestamp as i64),
                        ) {
                            Ok(serialized) => {
                                let stringified = serialized.to_string();
                                let stringified_bytes = stringified.as_bytes().len();
                                let pretty_stringified =
                                    serde_json::to_string_pretty(&serialized).ok();
                                if profile.multiple_requests {
                                    let client = client.clone();
                                    let data_stream_url = data_stream_url.clone();
                                    let worker_event_tx = worker_event_tx.clone();
                                    // TODO: this might be a performance bottleneck; fix?
                                    tokio::task::spawn(async move {
                                        match put_to_server(
                                            &client,
                                            &data_stream_url,
                                            stringified,
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
                                                    stringified_bytes
                                                ))
                                                .await
                                                .expect("worker event tx closed!"),
                                        }
                                    });
                                } else {
                                    match put_to_server(
                                        &client,
                                        &data_stream_url,
                                        stringified,
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
                                                stringified_bytes
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
            latest_payload: None,
            latest_payload_size: None,
            errors: vec![],
            serial_join_handle,
            network_processing_join_handle,
            worker_event_rx: Arc::new(Mutex::new(worker_event_rx)),
        })
    }

    fn update_from_event(
        errors: &mut Vec<ErrorInfo>,
        latest_payload: &mut Option<String>,
        latest_payload_size: &mut Option<usize>,
        latency_graph_data: &mut LatencyGraphData,
        event: WorkerEvent,
    ) {
        match event {
            WorkerEvent::ErrorEvent(err) => {
                eprintln!("WARN stream {}", err.msg);
                errors.push(err)
            }
            WorkerEvent::LatencySampleEvent(
                sample,
                new_latest_payload,
                new_latest_payload_size,
            ) => {
                *latest_payload = new_latest_payload;
                *latest_payload_size = Some(new_latest_payload_size);
                latency_graph_data.samples.push(sample)
            }
            WorkerEvent::SerialEvent(event) => latency_graph_data.serial_events.push(event),
        }
    }

    pub async fn update_stats(&mut self) {
        {
            let mut rx = self.worker_event_rx.lock().await;
            while !rx.is_empty() {
                let event = rx.recv().await;
                if let Some(event) = event {
                    ActiveStream::update_from_event(
                        &mut self.errors,
                        &mut self.latest_payload,
                        &mut self.latest_payload_size,
                        &mut self.latency_graph_data,
                        event,
                    );
                }
            }
        }
        self.purge_old_data(Duration::from_secs(60 * 5), 20)
    }

    pub fn update_from_events(&mut self, events: Vec<WorkerEvent>) {
        for event in events {
            ActiveStream::update_from_event(
                &mut self.errors,
                &mut self.latest_payload,
                &mut self.latest_payload_size,
                &mut self.latency_graph_data,
                event,
            );
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

    pub fn latest_payload_size(&self) -> Option<usize> {
        self.latest_payload_size
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
