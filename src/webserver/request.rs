use std::collections::HashMap;

use log::trace;

use super::http_utils::HttpProtocol;

#[derive(PartialEq, Debug)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    OPTIONS,
    TRACE,
    CONNECT,
}

impl TryFrom<&[u8]> for HttpMethod {
    type Error = &'static str;
    fn try_from(method: &[u8]) -> Result<Self, Self::Error> {
        match method {
            b"GET" => Ok(HttpMethod::GET),
            b"POST" => Ok(HttpMethod::POST),
            b"PUT" => Ok(HttpMethod::PUT),
            b"DELETE" => Ok(HttpMethod::DELETE),
            b"HEAD" => Ok(HttpMethod::HEAD),
            b"OPTIONS" => Ok(HttpMethod::OPTIONS),
            b"TRACE" => Ok(HttpMethod::TRACE),
            b"CONNECT" => Ok(HttpMethod::CONNECT),
            _ => Err("Met Unknown Http Method while parsing."),
        }
    }
}

pub struct RequestLine {
    pub method: HttpMethod,
    pub path: String,
    pub protocol: HttpProtocol,
}

pub struct Request<'buf> {
    pub request_line: RequestLine,
    pub headers: HashMap<&'buf [u8], &'buf [u8]>,
    pub body: &'buf [u8],
}

// Parse status line, headers, and body
pub fn parse_request_before_body(
    request_buf: &Vec<u8>,
) -> Result<(RequestLine, HashMap<&[u8], &[u8]>), &'static str> {
    trace!("Enter parse_request_before_body");
    let crlf_position: Vec<usize> = request_buf
        .windows(2)
        .enumerate()
        .filter(|(_, s)| s.starts_with(b"\r\n"))
        .map(|(i, _)| i as usize)
        .collect();

    if crlf_position.len() <= 2 {
        return Err("Something is wrong in request. CRLF <= 2 detected");
    }

    // extract request_line
    let request_line = parse_request_line(&request_buf[0..crlf_position[0]])?;

    // extract headers
    let mut headers: HashMap<&[u8], &[u8]> = HashMap::new();
    let mut cur_start_position = crlf_position[0] + 2;
    for &cur_crlf_position in &crlf_position[1..] {
        // crlf two times => body
        if cur_start_position == cur_crlf_position {
            break;
        }

        let mut header_line =
            request_buf[cur_start_position..cur_crlf_position].split(|&s| s == b':');
        let header = header_line.next().unwrap();
        let value = trim_byte_slice(header_line.next().unwrap());
        headers.insert(header, value);

        cur_start_position = cur_crlf_position + 2;
    }

    trace!("Leave parse_request_buf_before_body");

    Ok((request_line, headers))
}

fn parse_request_line(request_line: &[u8]) -> Result<RequestLine, &'static str> {
    let mut request_line = request_line.split(|c| c.is_ascii_whitespace());
    let method = request_line.next().unwrap();
    let path = request_line.next().unwrap();
    let protocol = request_line.next().unwrap();

    // convert method string to enum
    let method = HttpMethod::try_from(method)?;
    let path =
        String::from_utf8(path.to_vec()).map_err(|_| "Error in converting path to string")?;

    let protocol = HttpProtocol::try_from(protocol)?;

    Ok(RequestLine {
        method,
        path,
        protocol,
    })
}

fn trim_byte_slice(byte_slice: &[u8]) -> &[u8] {
    let mut start = 0;
    let mut end = byte_slice.len();
    for (idx, &c) in byte_slice.iter().enumerate() {
        if !c.is_ascii_whitespace() {
            start = idx;
            break;
        } 
    }

    for(idx, &c) in byte_slice.iter().enumerate().rev() {
        if !c.is_ascii_whitespace() {
            end = idx+1;
            break;
        } 

    }

    &byte_slice[start..end]
}
