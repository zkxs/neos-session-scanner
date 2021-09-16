use std::convert::TryInto;
use std::io::Read;

use bytes::{Buf, BytesMut};

use crate::lnl_protocol::{Error, Response};

pub trait Deserialize {
    fn deserialize(src: &mut BytesMut) -> Result<Response, Error>;
}

#[derive(Debug)]
pub struct NatPunch {
    /// should be zero?
    pub mystery_number: u8,
    pub local_host: String,
    pub local_port: u32,
    pub remote_host: String,
    pub remote_port: u32,
    /// lowercase, no prefix, otherwise matches request
    pub short_identifier: String,
}

impl Deserialize for NatPunch {
    fn deserialize(src: &mut BytesMut) -> Result<Response, Error> {
        let mystery_number = read_u8(src)?;
        let local_host = read_string(src)?;
        let local_port = read_u32(src)?;
        let remote_host = read_string(src)?;
        let remote_port = read_u32(src)?;
        let short_identifier = read_string(src)?;
        Ok(Response::nat_punch(mystery_number, local_host, local_port, remote_host, remote_port, short_identifier))
    }
}

#[derive(Debug)]
pub struct NatPunchError {
    pub response: String,
}

impl Deserialize for NatPunchError {
    fn deserialize(src: &mut BytesMut) -> Result<Response, Error> {
        let error_string = read_string(src)?;
        Ok(Response::nat_punch_error(error_string))
    }
}

#[derive(Debug)]
pub struct Connect {
    /// should be one?
    pub mystery_number: u8,
    /// lowercase, no prefix, exactly matches request
    pub short_identifier: String,
}

impl Deserialize for Connect {
    fn deserialize(src: &mut BytesMut) -> Result<Response, Error> {
        let mystery_number = read_u8(src)?;
        let short_identifier = read_string(src)?;
        Ok(Response::connect(mystery_number, short_identifier))
    }
}

#[derive(Debug)]
pub struct Unknown {
    pub bytes: Vec<u8>,
}

fn read_u8(bytes: &mut BytesMut) -> Result<u8, Error> {
    if bytes.remaining() >= 1 {
        Ok(bytes.get_u8())
    } else {
        Err(Error::from_string("not enough buffer remaining to read a u8"))
    }
}

fn read_u32(bytes: &mut BytesMut) -> Result<u32, Error> {
    if bytes.remaining() >= 4 {
        Ok(bytes.get_u32_le())
    } else {
        Err(Error::from_string("not enough buffer remaining to read a u32"))
    }
}

fn read_string(bytes: &mut BytesMut) -> Result<String, Error> {
    if bytes.remaining() >= 4 {
        let length: usize = bytes.get_u32_le().try_into().expect("could not fit u32 into usize");
        if bytes.remaining() >= length {
            let mut str_buf = String::with_capacity(length);
            let read = bytes.take(length).reader().read_to_string(&mut str_buf).unwrap();
            if read == length {
                Ok(str_buf)
            } else {
                Err(Error::from_string("read wrong number of bytes from string"))
            }
        } else {
            Err(Error::from_string("length prefix for string passed end of buffer"))
        }
    } else {
        Err(Error::from_string("not enough buffer remaining to read a length prefix"))
    }
}

#[cfg(test)]
mod tests {
    use bytes::BytesMut;

    use crate::lnl_protocol::response::Response;

    #[test]
    fn test_deserialize_nat_punch() {
        let vec = hex::decode("0D000D0000003139322E3136382E312E313530EE9B00000D00000037312E33362E3130312E313936EE9B00001C000000732D752D7573666E2D6F72696F6E3A7573666E6C756E61727061726B").unwrap();
        let mut bytes = BytesMut::from(vec.as_slice());
        let response = Response::deserialize(&mut bytes);
        if let Response::NatPunch(nat_punch) = response {
            assert_eq!(0, nat_punch.mystery_number);
            assert_eq!("192.168.1.150", nat_punch.local_host);
            assert_eq!(39918, nat_punch.local_port);
            assert_eq!("71.36.101.196", nat_punch.remote_host);
            assert_eq!(39918, nat_punch.remote_port);
            assert_eq!("s-u-usfn-orion:usfnlunarpark", nat_punch.short_identifier);
        } else {
            panic!("response was not a NatPunch");
        }
    }

    #[test]
    fn test_deserialize_nat_punch_error() {
        let vec = hex::decode("0B300000005345525645525F474F4E453A31633336633131662D636238652D343366312D393835642D396162393364366638366432").unwrap();
        let mut bytes = BytesMut::from(vec.as_slice());
        let response = Response::deserialize(&mut bytes);
        if let Response::NatPunchError(nat_error) = response {
            assert_eq!("SERVER_GONE:1c36c11f-cb8e-43f1-985d-9ab93d6f86d2", nat_error.response);
        } else {
            panic!("response was not a NatPunchError");
        }
    }

    #[test]
    fn test_deserialize_connect() {
        let vec = hex::decode("0e011c000000732d752d7573666e2d6f72696f6e3a7573666e6c756e61727061726b").unwrap();
        let mut bytes = BytesMut::from(vec.as_slice());
        let response = Response::deserialize(&mut bytes);
        if let Response::Connect(connect) = response {
            assert_eq!(1, connect.mystery_number);
            assert_eq!("s-u-usfn-orion:usfnlunarpark", connect.short_identifier);
        } else {
            panic!("response was not a Connect");
        }
    }
}
