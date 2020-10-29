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
    //let deserialized: Response = serde_json::from_str(str).unwrap();
    //println!("deserialized = {:?}", deserialized);

    let resp = Response{
        download: 70087681.29487312,
        upload: 95260765.00399733,
        time: Utc::now(),
        ping: 3.789,
    };

    //let write_result = client.query(&resp.into_query("status")).await;

    let read_query = Query::raw_read_query("select * from status");
    let read_result = client.query(&read_query).await;
    println!("{}", read_result.unwrap());

    //return client;
}