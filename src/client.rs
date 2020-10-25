pub mod request;
pub mod response;

use influxdb::{Client, Query, Timestamp};
use std::process::Command;
use serde::{Serialize, Deserialize};
use serde_json;
use chrono::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    download: f64,
    upload: f64,
    ping: f64,
    timestamp: DateTime<Utc>,
}
pub fn hello() -> String {
    "hello".to_string()
}

pub fn run() {
    //let client = Client::new("http://localhost:8086", "test");
    let out = Command::new("speedtest")
        .args(&["--json"])
        .output()
        .expect("failed to start");

    let str = std::str::from_utf8(&out.stdout).unwrap();
    let deserialized: Response = serde_json::from_str(str).unwrap();

    println!("deserialized = {:?}", deserialized);
    //return client;
}