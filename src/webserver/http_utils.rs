#[derive(Debug)]
pub enum HttpProtocol {
    HTTP1_0,
    HTTP1_1,
    HTTP2,
    HTTP3,
}

impl TryFrom<&[u8]> for HttpProtocol {
    type Error = &'static str;
    fn try_from(protocol: &[u8]) -> Result<Self, Self::Error> {
        match protocol {
            b"HTTP/1.0" => Ok(HttpProtocol::HTTP1_0),
            b"HTTP/1.1" => Ok(HttpProtocol::HTTP1_1),
            b"HTTP/2" => Ok(HttpProtocol::HTTP2),
            b"HTTP/3" => Ok(HttpProtocol::HTTP3),
            _ => Err("Met unknown Http Protocol while parsing"),
        }
    }
}

impl HttpProtocol {
    pub fn as_bytes(&self) -> &'static [u8] {
        match self {
            HttpProtocol::HTTP1_0 => b"HTTP/1.0",
            HttpProtocol::HTTP1_1 => b"HTTP/1.1",
            HttpProtocol::HTTP2 => b"HTTP/2",
            HttpProtocol::HTTP3 => b"HTTP/3",
        }
    }
}
