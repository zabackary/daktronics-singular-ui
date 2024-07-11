use std::time::{Duration, Instant};

use reqwest::{Client, IntoUrl};

pub async fn put_to_server<U: IntoUrl>(
    client: &Client,
    data_stream_url: U,
    serialized: String,
) -> Result<Duration, reqwest::Error> {
    let start_instant = Instant::now();
    client
        .put(data_stream_url)
        .body(serialized)
        .header("Content-Type", "application/json")
        .send()
        .await?;
    Ok(start_instant.elapsed())
}
