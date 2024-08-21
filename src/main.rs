use std::fs;

use toml::Table;

mod webserver;

fn main() {
    init();
    let config_str = fs::read_to_string("server-config.toml").expect("Config file doesn't exist");
    let config: Table = toml::from_str(&config_str).expect("Config file not structured as toml");

    let ip = config
        .get("ip")
        .and_then(|v| v.as_str())
        .unwrap_or("localhost");

    let port = config
        .get("ip")
        .and_then(|v| v.as_integer())
        .unwrap_or(8080) as u16;
    
    let server_type = config.get("server_type").and_then(|v| v.as_str()).unwrap_or("single");

    match server_type {
        "single" => webserver::run::run_ipv4_server(ip, port),
        "multi" => webserver::run::run_ipv4_server_multithreaded(ip, port),
        "event" => webserver::run::run_ipv4_server_event_based(ip, port),
        _ => panic!("No available server type"),
    }
}

fn init() {
    env_logger::init();
}
