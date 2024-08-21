use std::{thread::sleep, time::Duration};

use crate::webserver::{request::Request, response::{Response, StatusCode}};

pub fn mirror_request(request: &Request) -> Response {
    
    let mut response = Response::builder();
    for (&k, &v) in request.headers.iter() {
        response = response.header(&String::from_utf8_lossy(k), &String::from_utf8_lossy(v));
    }
    response = response.body(request.body.to_vec()).status_code(StatusCode::OK);
    response.build().unwrap_or_default()
}

pub fn long_time(_request: &Request) -> Response {
    sleep(Duration::from_millis(500));
    let body = "You have sleeped for 500 ms";
    Response::builder()
        .header("Content-Type", "text/plain")
        .status_code(StatusCode::OK)
        .body(body.into())
        .build()
        .unwrap_or_default()
}
