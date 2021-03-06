pub mod request;
pub mod response;

use influxdb::{Client, Query, Timestamp};
use influxdb::InfluxDbWriteable;
use std::process::Command;
use serde::{Serialize, Deserialize};
use serde_json;
use chrono::{DateTime, Utc};

#[derive(InfluxDbWriteable, Serialize, Deserialize, Debug)]
struct Response {
    download: f64,
    upload: f64,
    ping: f64,
    #[serde(rename="timestamp")]
    time: DateTime<Utc>,
}

pub fn hello() -> String {
    "hello".to_string()
}

#[tokio::main]
pub async fn run() {
    let client = Client::new("http://localhost:8086", "test");
    let out = Command::new("speedtest")
        .args(&["--json"])
        .output()
        .expect("failed to start");

    let str = std::str::from_utf8(&out.stdout).unwrap();
    let mut deserialized: Response = serde_json::from_str(str).unwrap();
    //println!("deserialized = {:?}", deserialized);

    let _write_result = client.query(&deserialized.into_query("status")).await;

    let read_query = Query::raw_read_query("select * from status");
    let read_result = client.query(&read_query).await;
    println!("{}", read_result.unwrap());
}