mod webserver;

fn main() {
    init();

    let ip = "localhost";
    let port = 9090;
    webserver::run::run_ipv4_server(ip, port);
}

fn init() {
    env_logger::init();
}

