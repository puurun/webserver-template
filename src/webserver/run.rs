use std::collections::HashMap;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc, Mutex};
use std::thread::{JoinHandle, self};
use std::{
    fmt::Debug,
    io::{BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
};

use log::{debug, error, info};

use crate::webserver::request::Request;

use super::{api_endpoint_manager::ApiEndPointManager, request::parse_request_before_body};

struct Worker {
    join_handle: Option<JoinHandle<()>>,
}

struct ThreadPool {
    workers: Vec<Worker>,
    receiver: Arc<Mutex<Receiver<TcpStream>>>,
    sender: Sender<TcpStream>,
}

impl ThreadPool {
    fn new() -> Self {
        let (tx, rx) = mpsc::channel::<TcpStream>();
        let mutex = Mutex::new(rx);
        Self {
            workers: Vec::new(),
            receiver: Arc::new(mutex),
            sender: tx,
        }
    }

    fn spawn_threads(&mut self, manager: Arc<ApiEndPointManager>, num_threads: i32) {
        for _ in 0..num_threads {
            let manager = Arc::clone(&manager);
            let rx = self.receiver.clone();
            let join_handle = std::thread::spawn(|| {
                info!("Created: {:?}", thread::current());
                thread_main(manager, rx);
            });
            let join_handle = Some(join_handle);

            self.workers.push(Worker { join_handle });
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        
        for worker in self.workers.iter_mut() {
            info!("Shutting down worker");
            if let Some(handle) = worker.join_handle.take() {
                handle.join().unwrap();
            }
        }
    }
}

#[allow(dead_code)]
pub fn run_ipv4_server_event_based(ip: &str, port: u16) {}

#[allow(dead_code)]
pub fn run_ipv4_server_multithreaded(ip: &str, port: u16) {
    info!("Starting Server (multi) on {}:{}...", ip, port);

    let ip_port_string = format!("{}:{}", ip, port);
    let listener = TcpListener::bind(ip_port_string).unwrap();

    let manager = ApiEndPointManager::get();
    let manager = Arc::new(manager);

    let mut thread_pool = ThreadPool::new();
    thread_pool.spawn_threads(manager, 1024);

    info!("Started...");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let result = thread_pool.sender.send(stream);
                if let Err(e) = result {
                    error!("Error while distributing TcpStream: {e}")
                }
            }
            Err(e) => {
                error!("Connection Failed, {}", e);
            }
        }
    }
}

pub fn thread_main(manager: Arc<ApiEndPointManager>, rx: Arc<Mutex<Receiver<TcpStream>>>) {
    loop {
        let acquire = rx.lock();
        info!("{:?} alive", thread::current());
        let receiver = match acquire {
            Ok(rx) => rx,
            Err(e) => {
                error!("Error while holding the lock: {}", e);
                continue;
            }
        };
        
        let stream = match receiver.recv() {
            Ok(stream) => stream,
            Err(e) => {
                error!("Error while receiving TcpStream, {e}");
                continue;
            }
        };

        drop(receiver);


        handle_stream(manager.clone(), stream);    
    }
}

#[allow(dead_code)]
pub fn run_ipv4_server(ip: &str, port: u16) {
    info!("Starting Server on {}:{}...", ip, port);

    let ip_port_string = format!("{}:{}", ip, port);
    let listener = TcpListener::bind(ip_port_string).unwrap();

    let manager = ApiEndPointManager::get();
    let manager = Arc::new(manager);

    info!("Started...");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let manager = Arc::clone(&manager);
                std::thread::spawn(|| {
                    handle_stream(manager, stream);
                });
            }
            Err(e) => {
                error!("Connection Failed, {}", e);
            }
        }
    }
}

pub fn handle_stream(endpoint_manager: Arc<ApiEndPointManager>, mut stream: TcpStream) {
    let mut request_buf = Vec::new();
    let mut reader = BufReader::new(&stream);

    // Parse request line and headers
    let result = read_until_double_crlf(&mut reader, &mut request_buf);
    if let Err(e) = result {
        error!("Error in reading before body: {}", e);
    }

    let (request_line, headers) = match parse_request_before_body(&request_buf) {
        Ok(request) => request,
        Err(e) => {
            error!("Parsing Error! {}", e);
            return;
        }
    };

    // Get body
    let mut body_buf = Vec::new();
    let result = read_body(&mut reader, &mut body_buf, &headers).unwrap_or_else(|e| {
        info!("Determined body doesn't exist: {}", e);
        return 0;
    });

    let body = &body_buf[..result];

    debug!(
        "\n{:?} {:} {:?}\n{:?}\n{:}",
        request_line.method,
        request_line.path,
        request_line.protocol,
        HeaderDebugWrapper {
            headers: headers.clone()
        },
        String::from_utf8_lossy(body),
    );

    // Make request struct
    let request = Request {
        request_line,
        headers,
        body,
    };

    let response = endpoint_manager.handle_request(&request);
    let raw_response = response.serialize();
    debug!("raw_response\n{}", String::from_utf8_lossy(&raw_response));
    let _ = stream.write_all(&raw_response).unwrap();
}

fn read_until_double_crlf(
    reader: &mut BufReader<&TcpStream>,
    buf: &mut Vec<u8>,
) -> Result<usize, &'static str> {
    let total_bytes_read = 0;
    loop {
        let read_result = reader.read_until(b'\n', buf);
        let bytes_read = match read_result {
            Ok(count) => count,
            Err(_) => return Err("Error in reading until double crlf"),
        };

        if bytes_read == 0 || buf.ends_with(b"\r\n\r\n") {
            break;
        }
    }
    Ok(total_bytes_read)
}

fn read_body(
    reader: &mut BufReader<&TcpStream>,
    request_buf: &mut Vec<u8>,
    headers: &std::collections::HashMap<&[u8], &[u8]>,
) -> Result<usize, &'static str> {
    // Get size from Content-Length
    let size = headers
        .get(&b"Content-Length"[..])
        .ok_or("No Content Length")?;
    let size = byte_slice_to_i32(size)?;

    // Read size bytes from reader
    let mut chunk = reader.take(size as u64);
    let bytes_read = chunk
        .read_to_end(request_buf)
        .map_err(|_| "Error in reading body")?;

    Ok(bytes_read)
}

fn byte_slice_to_i32(bytes_slice: &[u8]) -> Result<i32, &'static str> {
    let mut total: i32 = 0;
    for &byte in bytes_slice {
        if b'0' <= byte && byte <= b'9' {
            let cur = (byte - b'0') as i32;
            total *= 10;
            total += cur;
        } else {
            return Err("Error in parsing byte slice to i32");
        }
    }
    Ok(total)
}

struct HeaderDebugWrapper<'a> {
    headers: HashMap<&'a [u8], &'a [u8]>,
}

impl Debug for HeaderDebugWrapper<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (&key, &val) in self.headers.iter() {
            write!(
                f,
                "{}: {}\n",
                String::from_utf8_lossy(&key),
                String::from_utf8_lossy(&val)
            )?;
        }
        Ok(())
    }
}
