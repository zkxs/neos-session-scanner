use bytes::BytesMut;
use tokio_util::codec::{Decoder, Encoder};

use crate::protocol::request::Request;
use crate::protocol::response::Response;

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
            Ok(Some(Response::parse(src)))
        }
    }
}

impl Encoder<Request> for Codec {
    type Error = Error;

    fn encode(&mut self, item: Request, dst: &mut BytesMut) -> Result<(), Self::Error> {
        todo!()
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
