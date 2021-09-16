use bytes::{Buf, BytesMut};
use tokio_util::codec::{Decoder, Encoder};

use request::Serialize;
use response::Deserialize;

pub mod request;
pub mod response;

#[derive(Default)]
pub struct Codec {}

impl Decoder for Codec {
    type Item = Response;
    type Error = Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.is_empty() {
            Ok(None)
        } else {
            Ok(Some(Response::deserialize(src)))
        }
    }
}

impl Encoder<Request> for Codec {
    type Error = Error;

    fn encode(&mut self, item: Request, dst: &mut BytesMut) -> Result<(), Self::Error> {
        item.serialize(dst)
    }
}

#[derive(Debug)]
pub struct Error {
    pub value: String,
}

impl Error {
    fn from_string<T: Into<String>>(value: T) -> Error {
        Error { value: value.into() }
    }
}

impl From<std::io::Error> for Error {
    fn from(underlying: std::io::Error) -> Self {
        Error { value: underlying.to_string() }
    }
}

pub enum Request {
    NatPunch(request::NatPunch),
    Connect(request::Connect),
}

impl Request {
    pub fn nat_punch(host: String, port: u32, full_identifier: String) -> Request {
        Request::NatPunch(request::NatPunch { host, port, full_identifier })
    }

    pub fn connect(short_identifier: String) -> Request {
        Request::Connect(request::Connect { mystery_number: 0, short_identifier })
    }

    pub fn serialize(self, dst: &mut BytesMut) -> Result<(), Error> {
        match self {
            Request::NatPunch(r) => r.serialize(dst),
            Request::Connect(r) => r.serialize(dst),
        }
    }
}

pub enum Response {
    NatPunch(response::NatPunch),
    NatPunchError(response::NatPunchError),
    Connect(response::Connect),
    Unknown(response::Unknown),
}

impl Response {
    pub fn deserialize(src: &mut BytesMut) -> Response {
        // dupe the bytes into a vec because the buffer sucks
        let bytes: Vec<u8> = src.as_ref().into();

        let magic_number = src.get_u8();
        match magic_number {
            0x0B => {
                response::NatPunchError::deserialize(src).unwrap_or_else(|e| handle_err(e, bytes))
            }
            0x0D => {
                response::NatPunch::deserialize(src).unwrap_or_else(|e| handle_err(e, bytes))
            }
            0x0E => {
                response::Connect::deserialize(src).unwrap_or_else(|e| handle_err(e, bytes))
            }
            _ => Response::unknown(bytes)
        }
    }

    fn nat_punch(mystery_number: u8, local_host: String, local_port: u32, remote_host: String, remote_port: u32, short_identifier: String) -> Response {
        Response::NatPunch(response::NatPunch { mystery_number, local_host, local_port, remote_host, remote_port, short_identifier })
    }

    fn nat_punch_error(response: String) -> Response {
        Response::NatPunchError(response::NatPunchError { response })
    }

    fn connect(mystery_number: u8, short_identifier: String) -> Response {
        Response::Connect(response::Connect { mystery_number, short_identifier })
    }

    fn unknown(bytes: Vec<u8>) -> Response {
        Response::Unknown(response::Unknown { bytes })
    }
}

fn handle_err(e: Error, bytes: Vec<u8>) -> Response {
    eprintln!("parsing error: {}", e.value);
    Response::unknown(bytes)
}
