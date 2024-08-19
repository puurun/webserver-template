use std::collections::HashMap;

use log::{debug, trace};

use super::http_utils::HttpProtocol;

#[allow(dead_code)]
pub enum StatusCode {
    // 1xx Informational
    Continue,
    SwitchingProtocol,
    Processing,
    EarlyHints,
    // 2xx Successful
    OK,
    Created,
    Accepted,
    NonAuthoritativeInformation,
    NoContent,
    ResetContent,
    PartialContent,
    MultiStatus,
    AlreadyReported,
    IMUsed,
    // 3xx Redirection
    MultipleChoices,
    MovedPermanently,
    Found,
    SeeOther,
    NotModified,
    UseProxy,
    Unused,
    TemporaryRedirect,
    PermanentRedirect,
    // 4xx Client Error
    BadRequest,
    Unauthorized,
    PaymentRequired,
    Forbidden,
    NotFound,
    MethodNotAllowed,
    NotAcceptible,
    ProxyAuthenticationRequired,
    RequestTimeout,
    Conflict,
    Gone,
    LengthRequired,
    PreconditionFailed,
    PayloadTooLarge,
    URITooLong,
    UnsupportedMediaType,
    RangeNotSatisfiable,
    ExpectationFailed,
    ImATeapot,
    MisdirectedRequest,
    UnprocessableContent,
    Locked,
    FailedDependency,
    TooEarly,
    UpgradeRequired,
    PreconditionRequired,
    TooManyRequests,
    RequestHeaderFieldsTooLarge,
    UnavailableForLegalReasons,
    //5xx Server Error
    InternalServerError,
    NotImplemented,
    BadGateway,
    ServiceUnavailable,
    GatewayTimeout,
    HTTPVersionNotSupported,
    VariantsAlsoNegotiates,
    InsufficientStorage,
    LoopDetected,
    NotExtended,
    NetworkAuthenticationRequired,
}

impl StatusCode {
    pub fn value(&self) -> &'static [u8] {
        match self {
            StatusCode::Continue => b"100",
            StatusCode::SwitchingProtocol => b"101",
            StatusCode::Processing => b"102",
            StatusCode::EarlyHints => b"103",
            StatusCode::OK => b"200",
            StatusCode::Created => b"201",
            StatusCode::Accepted => b"202",
            StatusCode::NonAuthoritativeInformation => b"203",
            StatusCode::NoContent => b"204",
            StatusCode::ResetContent => b"205",
            StatusCode::PartialContent => b"206",
            StatusCode::MultiStatus => b"207",
            StatusCode::AlreadyReported => b"208",
            StatusCode::IMUsed => b"226",
            StatusCode::MultipleChoices => b"300",
            StatusCode::MovedPermanently => b"301",
            StatusCode::Found => b"302",
            StatusCode::SeeOther => b"303",
            StatusCode::NotModified => b"304",
            StatusCode::UseProxy => b"305",
            StatusCode::Unused => b"306",
            StatusCode::TemporaryRedirect => b"307",
            StatusCode::PermanentRedirect => b"308",
            StatusCode::BadRequest => b"400",
            StatusCode::Unauthorized => b"401",
            StatusCode::PaymentRequired => b"402",
            StatusCode::Forbidden => b"403",
            StatusCode::NotFound => b"404",
            StatusCode::MethodNotAllowed => b"405",
            StatusCode::NotAcceptible => b"406",
            StatusCode::ProxyAuthenticationRequired => b"407",
            StatusCode::RequestTimeout => b"408",
            StatusCode::Conflict => b"409",
            StatusCode::Gone => b"410",
            StatusCode::LengthRequired => b"411",
            StatusCode::PreconditionFailed => b"412",
            StatusCode::PayloadTooLarge => b"413",
            StatusCode::URITooLong => b"414",
            StatusCode::UnsupportedMediaType => b"415",
            StatusCode::RangeNotSatisfiable => b"416",
            StatusCode::ExpectationFailed => b"417",
            StatusCode::ImATeapot => b"418",
            StatusCode::MisdirectedRequest => b"421",
            StatusCode::UnprocessableContent => b"422",
            StatusCode::Locked => b"423",
            StatusCode::FailedDependency => b"424",
            StatusCode::TooEarly => b"425",
            StatusCode::UpgradeRequired => b"426",
            StatusCode::PreconditionRequired => b"428",
            StatusCode::TooManyRequests => b"429",
            StatusCode::RequestHeaderFieldsTooLarge => b"431",
            StatusCode::UnavailableForLegalReasons => b"451",
            StatusCode::InternalServerError => b"500",
            StatusCode::NotImplemented => b"501",
            StatusCode::BadGateway => b"502",
            StatusCode::ServiceUnavailable => b"503",
            StatusCode::GatewayTimeout => b"504",
            StatusCode::HTTPVersionNotSupported => b"505",
            StatusCode::VariantsAlsoNegotiates => b"506",
            StatusCode::InsufficientStorage => b"507",
            StatusCode::LoopDetected => b"508",
            StatusCode::NotExtended => b"510",
            StatusCode::NetworkAuthenticationRequired => b"511",
        }
    }

    fn string_value(&self) -> &'static [u8] {
        match self {
            StatusCode::Continue => b"Continue",
            StatusCode::SwitchingProtocol => b"SwitchingProtocol",
            StatusCode::Processing => b"Processing",
            StatusCode::EarlyHints => b"EarlyHints",
            StatusCode::OK => b"OK",
            StatusCode::Created => b"Created",
            StatusCode::Accepted => b"Accepted",
            StatusCode::NonAuthoritativeInformation => b"NonAuthoritativeInformation",
            StatusCode::NoContent => b"NoContent",
            StatusCode::ResetContent => b"ResetContent",
            StatusCode::PartialContent => b"PartialContent",
            StatusCode::MultiStatus => b"MultiStatus",
            StatusCode::AlreadyReported => b"AlreadyReported",
            StatusCode::IMUsed => b"IMUsed",
            StatusCode::MultipleChoices => b"MultipleChoices",
            StatusCode::MovedPermanently => b"MovedPermanently",
            StatusCode::Found => b"Found",
            StatusCode::SeeOther => b"SeeOther",
            StatusCode::NotModified => b"NotModified",
            StatusCode::UseProxy => b"UseProxy",
            StatusCode::Unused => b"Unused",
            StatusCode::TemporaryRedirect => b"TemporaryRedirect",
            StatusCode::PermanentRedirect => b"PermanentRedirect",
            StatusCode::BadRequest => b"BadRequest",
            StatusCode::Unauthorized => b"Unauthorized",
            StatusCode::PaymentRequired => b"PaymentRequired",
            StatusCode::Forbidden => b"Forbidden",
            StatusCode::NotFound => b"Not Found",
            StatusCode::MethodNotAllowed => b"MethodNotAllowed",
            StatusCode::NotAcceptible => b"NotAcceptible",
            StatusCode::ProxyAuthenticationRequired => b"ProxyAuthenticationRequired",
            StatusCode::RequestTimeout => b"RequestTimeout",
            StatusCode::Conflict => b"Conflict",
            StatusCode::Gone => b"Gone",
            StatusCode::LengthRequired => b"LengthRequired",
            StatusCode::PreconditionFailed => b"PreconditionFailed",
            StatusCode::PayloadTooLarge => b"PayloadTooLarge",
            StatusCode::URITooLong => b"URITooLong",
            StatusCode::UnsupportedMediaType => b"UnsupportedMediaType",
            StatusCode::RangeNotSatisfiable => b"RangeNotSatisfiable",
            StatusCode::ExpectationFailed => b"ExpectationFailed",
            StatusCode::ImATeapot => b"ImATeapot",
            StatusCode::MisdirectedRequest => b"MisdirectedRequest",
            StatusCode::UnprocessableContent => b"UnprocessableContent",
            StatusCode::Locked => b"Locked",
            StatusCode::FailedDependency => b"FailedDependency",
            StatusCode::TooEarly => b"TooEarly",
            StatusCode::UpgradeRequired => b"UpgradeRequired",
            StatusCode::PreconditionRequired => b"PreconditionRequired",
            StatusCode::TooManyRequests => b"TooManyRequests",
            StatusCode::RequestHeaderFieldsTooLarge => b"RequestHeaderFieldsTooLarge",
            StatusCode::UnavailableForLegalReasons => b"UnavailableForLegalReasons",
            StatusCode::InternalServerError => b"InternalServerError",
            StatusCode::NotImplemented => b"NotImplemented",
            StatusCode::BadGateway => b"BadGateway",
            StatusCode::ServiceUnavailable => b"ServiceUnavailable",
            StatusCode::GatewayTimeout => b"GatewayTimeout",
            StatusCode::HTTPVersionNotSupported => b"HTTPVersionNotSupported",
            StatusCode::VariantsAlsoNegotiates => b"VariantsAlsoNegotiates",
            StatusCode::InsufficientStorage => b"InsufficientStorage",
            StatusCode::LoopDetected => b"LoopDetected",
            StatusCode::NotExtended => b"NotExtended",
            StatusCode::NetworkAuthenticationRequired => b"NetworkAuthenticationRequired",
        }
    }
}

pub struct Response {
    protocol: HttpProtocol,
    status_code: StatusCode,
    headers: HashMap<Vec<u8>, Vec<u8>>,
    body: Vec<u8>,
}

impl Response {
    pub fn builder() -> ResponseBuilder {
        return ResponseBuilder {
            protocol: None,
            status_code: None,
            headers: None,
            body: None,
        };
    }

    pub fn serialize(&self) -> Vec<u8> {
        trace!("-- serizalize --");
        let mut raw_response: Vec<u8> = Vec::new();

        // make response
        // status line
        raw_response.extend(self.protocol.as_bytes());
        raw_response.push(b' ');
        raw_response.extend(self.status_code.value());
        raw_response.push(b' ');
        raw_response.extend(self.status_code.string_value());
        raw_response.extend(b"\r\n");

        // headers
        for (key, val) in self.headers.iter() {
            raw_response.extend(key);
            raw_response.extend(b": ");
            raw_response.extend(val);
            raw_response.extend(b"\r\n");
        }
        if !self.headers.contains_key(&b"Content-Length".to_vec()) {
            let buf = format!("Content-Length: {}\r\n", self.body.len());
            raw_response.extend(buf.as_bytes());
        }

        // some auto headers

        // body
        raw_response.extend(b"\r\n");
        raw_response.extend(&self.body);

        debug!("{}", String::from_utf8_lossy(&raw_response));
        raw_response
    }
}

impl Default for Response {
    fn default() -> Self {
        Self {
            protocol: HttpProtocol::HTTP1_1,
            status_code: StatusCode::NotFound,
            headers: Default::default(),
            body: Default::default(),
        }
    }
}

pub struct ResponseBuilder {
    protocol: Option<HttpProtocol>,
    status_code: Option<StatusCode>,
    headers: Option<HashMap<Vec<u8>, Vec<u8>>>,
    body: Option<Vec<u8>>,
}

pub enum ResponseBuilderError {
    NoStatusCode,
}

impl ResponseBuilder {
    #![allow(dead_code)]
    pub fn protocol(mut self, protocol: HttpProtocol) -> Self {
        self.protocol = Some(protocol);
        self
    }
    pub fn status_code(mut self, status_code: StatusCode) -> Self {
        self.status_code = Some(status_code);
        self
    }
    pub fn header(mut self, key: &str, val: &str) -> Self {
        match self.headers {
            Some(ref mut headers) => {
                headers.insert(key.into(), val.into());
            }
            None => {
                let mut new_map = HashMap::new();
                new_map.insert(key.into(), val.into());
                self.headers = Some(new_map);
            }
        }
        self
    }
    pub fn body(mut self, body: Vec<u8>) -> Self {
        self.body = Some(body);
        self
    }
    pub fn build(self) -> Result<Response, ResponseBuilderError> {
        let protocol = self.protocol.unwrap_or(HttpProtocol::HTTP1_1);
        let status_code = self.status_code.ok_or(ResponseBuilderError::NoStatusCode)?;
        let headers = self.headers.unwrap_or(HashMap::new());
        let body = self.body.unwrap_or(Vec::new());

        Ok(Response {
            protocol,
            status_code,
            headers,
            body,
        })
    }
}
