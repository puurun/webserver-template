use std::{
    fs::File,
    io::Read,
    path::PathBuf,
};

use log::{info, error, debug, trace};

use crate::webserver::response::{Response, StatusCode};

use super::request::{HttpMethod, Request};

impl ApiEndPointManager {
    // Register Endpoints in this function
    fn register_endpoints(&mut self) {
        self.register_endpoint(vec![HttpMethod::GET], "/*", ApiType::Static);
    }
}

#[allow(dead_code)]
enum ApiType {
    Static,
    Function,
}

struct ApiEndPoint {
    method: Vec<HttpMethod>,
    path: String,
    api_type: ApiType,
}

impl ApiEndPoint {
    fn path_matches(&self, other: &str) -> bool {
        let mut pattern = self.path.chars();
        let mut other = other.chars();

        loop {
            let pc = pattern.next();
            let oc = other.next();
            match (pc, oc) {
                (Some(pc), Some(oc)) => {
                    if pc == '*' {
                        break;
                    }
                    if pc != oc {
                        return false;
                    }
                }
                _ => break,
            }
        }

        true
    }

    fn contain_method(&self, method: &HttpMethod) -> bool {
        if self.method.contains(&method) {
            true
        } else {
            false
        }
    }
}

pub struct ApiEndPointManager {
    endpoints: Vec<ApiEndPoint>,
}

impl ApiEndPointManager {
    pub fn get() -> ApiEndPointManager {
        let mut manager = Self {
            endpoints: Vec::new(),
        };
        manager.register_endpoints();
        manager
    }

    pub fn handle_request(&self, request: &Request) -> Response {
        // iterate through endpoints registered
        // checks if path matches and contains method
        for endpoint in &self.endpoints {
            trace!("{}", endpoint.path);
            let path_match = endpoint.path_matches(&request.request_line.path);
            let contain_method = endpoint.contain_method(&request.request_line.method);

            return match (path_match, contain_method) {
                (true, true) => match endpoint.api_type {
                    ApiType::Static => self.serve_file(request),
                    ApiType::Function => todo!(),
                },
                (true, false) => Response::builder()
                    .status_code(StatusCode::MethodNotAllowed)
                    .build()
                    .unwrap_or_default(),

                (_, _) => continue,
            };
        }

        Response::builder()
            .status_code(StatusCode::NotFound)
            .build()
            .unwrap_or_default()
    }

    fn serve_file(&self, request: &Request) -> Response {
        let mut path = PathBuf::from("static");
        path.push(&request.request_line.path[1..]);
        debug!("path: {:?}", path);

        let served_file = File::open(path);
        match served_file {
            Ok(mut served_file) => {
                let mut buf = Vec::new();
                let _ = served_file.read_to_end(&mut buf);

                Response::builder()
                    .status_code(StatusCode::OK)
                    .body(buf)
                    .build()
                    .unwrap_or_default()
            }
            Err(e) => {
                error!("Error in serving static file: {}", e);
                Response::default()
            }
        }
    }

    fn register_endpoint(&mut self, method: Vec<HttpMethod>, path: &str, api_type: ApiType) {
        info!("Registering Api Endpoints...");
        let path = String::from(path);
        self.endpoints.push(ApiEndPoint {
            method,
            path,
            api_type,
        });
    }
}
