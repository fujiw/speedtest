fn main() {
    assert_eq!(speedtest::client::hello(), "hello".to_string());

    assert_eq!(speedtest::client::request::hello(), "request!".to_string());
    assert_eq!(speedtest::client::response::hello(), "response!!".to_string());
}
