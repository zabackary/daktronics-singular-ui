use std::{
    error::Error,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

use daktronics_allsport_5000::{sports::Sport, RTDState};
use latency_graph::{LatencyGraphData, LatencySample, SerialEvent};
use tokio::{
    sync::mpsc::{self, Receiver},
    task::JoinHandle,
};
use tokio_serial::SerialPortBuilderExt;

use crate::APP_USER_AGENT;

use super::{network::put_to_server, profile::Profile};

mod latency_graph {
    use std::time::{Duration, Instant};

    #[derive(Debug)]
    pub struct LatencySample {
        pub timestamp: Instant,
        pub latency: Duration,
    }

    #[derive(Debug)]
    pub struct SerialEvent {
        pub timestamp: Instant,
    }

    #[derive(Debug)]
    pub struct LatencyGraphData {
        pub samples: Vec<LatencySample>,
        pub serial_events: Vec<SerialEvent>,
    }
}

enum WorkerEvent {
    ErrorEvent(Box<dyn Error + Send>),
    SerialEvent(SerialEvent),
    LatencySampleEvent(LatencySample),
}

#[derive(Debug)]
pub struct ActiveStream {
    latency_graph_data: LatencyGraphData,
    /// The payload about to undergo processing
    serialized: Arc<Mutex<Option<serde_json::Value>>>,
    /// The latest payload the server is currently holding right now
    latest_payload: String,
    errors: Vec<Box<dyn Error>>,

    config: Profile,

    serial_join_handle: JoinHandle<()>,
    network_processing_join_handle: JoinHandle<()>,
    worker_event_rx: Receiver<WorkerEvent>,
}

impl ActiveStream {
    pub fn new(config: Profile, tty_path: String) -> Result<Self, Box<dyn Error>> {
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
                                    let mut serialized = serialized.lock().unwrap();
                                    *serialized = Some(new_data);
                                }
                                if let Err(err) = new_msg_tx.send(()) {
                                    worker_event_tx
                                        .send(WorkerEvent::ErrorEvent(Box::new(err)))
                                        .await
                                        .expect("worker event tx closed!")
                                }
                            }
                            Err(err) => worker_event_tx
                                .send(WorkerEvent::ErrorEvent(Box::new(err)))
                                .await
                                .expect("worker event tx closed!"),
                        },
                        Ok(false) => {
                            // don't bother to update if nothing changed
                        }
                        Err(err) => worker_event_tx
                            .send(WorkerEvent::ErrorEvent(Box::new(err)))
                            .await
                            .expect("worker event tx closed!"),
                    }
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
                        .send(WorkerEvent::ErrorEvent(Box::new(err)))
                        .await
                        .expect("worker event tx closed!")
                }

                loop {
                    let serialized = { serialized.lock().unwrap().take() };
                    if let Some(value) = serialized {
                        match mapping.map(&value) {
                            Ok(serialized) => {
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
                                                .send(WorkerEvent::ErrorEvent(Box::new(err)))
                                                .await
                                                .expect("worker event tx closed!"),
                                            Ok(latency) => worker_event_tx
                                                .send(WorkerEvent::LatencySampleEvent(
                                                    LatencySample {
                                                        timestamp: Instant::now(),
                                                        latency,
                                                    },
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
                                            .send(WorkerEvent::ErrorEvent(Box::new(err)))
                                            .await
                                            .expect("worker event tx closed!"),
                                        Ok(latency) => worker_event_tx
                                            .send(WorkerEvent::LatencySampleEvent(LatencySample {
                                                timestamp: Instant::now(),
                                                latency,
                                            }))
                                            .await
                                            .expect("worker event tx closed!"),
                                    }
                                }
                            }
                            Err(err) => worker_event_tx
                                .send(WorkerEvent::ErrorEvent(Box::new(err)))
                                .await
                                .expect("worker event tx closed!"),
                        }
                    }
                    while !new_msg_rx.is_empty() {
                        new_msg_rx.recv().await;
                    }
                }
            })
        };

        Ok(Self {
            latency_graph_data: LatencyGraphData {
                samples: vec![
                    LatencySample {
                        timestamp: Instant::now().checked_sub(Duration::from_secs(3)).unwrap(),
                        latency: Duration::from_millis(340),
                    },
                    LatencySample {
                        timestamp: Instant::now().checked_sub(Duration::from_secs(2)).unwrap(),
                        latency: Duration::from_millis(320),
                    },
                    LatencySample {
                        timestamp: Instant::now()
                            .checked_sub(Duration::from_millis(600))
                            .unwrap(),
                        latency: Duration::from_millis(312),
                    },
                ],
                serial_events: vec![
                    SerialEvent {
                        timestamp: Instant::now()
                            .checked_sub(Duration::from_millis(3350))
                            .unwrap(),
                    },
                    SerialEvent {
                        timestamp: Instant::now()
                            .checked_sub(Duration::from_millis(2330))
                            .unwrap(),
                    },
                    SerialEvent {
                        timestamp: Instant::now()
                            .checked_sub(Duration::from_millis(1020))
                            .unwrap(),
                    },
                ],
            },
            serialized,
            latest_payload: "{\n    \"key\": \"value\"\n}".into(),
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
            worker_event_rx,
        })
    }

    pub async fn update_stats(&mut self) {
        while !self.worker_event_rx.is_empty() {
            let event = self.worker_event_rx.recv().await;
            if let Some(event) = event {
                match event {
                    WorkerEvent::ErrorEvent(err) => self.errors.push(err),
                    WorkerEvent::LatencySampleEvent(sample) => {
                        self.latency_graph_data.samples.push(sample)
                    }
                    WorkerEvent::SerialEvent(event) => {
                        self.latency_graph_data.serial_events.push(event)
                    }
                }
            }
        }
    }

    pub fn latency_graph_data(&self) -> &LatencyGraphData {
        &self.latency_graph_data
    }

    pub fn latest_payload(&self) -> &str {
        &self.latest_payload
    }

    /// Deletes all latency graph data with ages more than the specific duration
    pub fn purge_old_latency_graph_data(&mut self, keep: Duration) {
        self.latency_graph_data
            .samples
            .retain(|x| x.timestamp.elapsed() < keep);
        self.latency_graph_data
            .serial_events
            .retain(|x| x.timestamp.elapsed() < keep);
    }
}

impl Drop for ActiveStream {
    fn drop(&mut self) {}
}
