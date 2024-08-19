use std::{
    io::{BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
};

use log::{debug, error, info};

use crate::webserver::request::Request;

use super::{api_endpoint_manager::ApiEndPointManager, request::parse_request_before_body};

pub fn run_ipv4_server(ip: &str, port: u16) {
    info!("Starting Server on {}:{}...", ip, port);

    let ip_port_string = format!("{}:{}", ip, port);
    let listener = TcpListener::bind(ip_port_string).unwrap();

    let manager = ApiEndPointManager::get();

    info!("Started...");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_stream(&manager, stream),
            Err(e) => {
                error!("Connection Failed, {}", e);
            }
        }
    }
}

pub fn handle_stream(endpoint_manager: &ApiEndPointManager, mut stream: TcpStream) {
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
        error!("Error in reading body: {}", e);
        return 0;
    });

    let body = &body_buf[..result];

    // Make request struct
    debug!("Incoming Request:");
    debug!("{:?} {:?} {:?}", request_line.method, request_line.path, request_line.protocol);
    debug!("{:?}", headers);
    debug!("-- body start -- \n{:?}", String::from_utf8_lossy(body));
    let request = Request {
        request_line,
        headers,
        body,
    };

    let response = endpoint_manager.handle_request(&request);
    let raw_response = response.serialize();
    debug!("raw_response: {}", String::from_utf8_lossy(&raw_response));
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
