use std::convert::TryInto;
use std::io::Read;

use bytes::{Buf, BytesMut};

use crate::lnl_protocol::{Error, Response};

pub trait Deserialize {
    fn deserialize(src: &mut BytesMut) -> Result<Response, Error>;
}

#[derive(Debug)]
pub struct NatPunch {
    pub local_host: String,
    pub local_port: u32,
    pub remote_host: String,
    pub remote_port: u32,
    /// lowercase, no prefix, otherwise matches request
    pub short_identifier: String,
}

impl Deserialize for NatPunch {
    fn deserialize(src: &mut BytesMut) -> Result<Response, Error> {
        let local_host = read_string(src)?;
        let local_port = read_u32(src)?;
        let remote_host = read_string(src)?;
        let remote_port = read_u32(src)?;
        let short_identifier = read_string(src)?;
        Ok(Response::nat_punch(local_host, local_port, remote_host, remote_port, short_identifier))
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
    /// should be 0x01
    pub mystery_number: u8,
    /// lowercase, no prefix, exactly matches request
    pub short_identifier: String,
}

impl Deserialize for Connect {
    fn deserialize(src: &mut BytesMut) -> Result<Response, Error> {
        let short_identifier = read_string(src)?;
        let mystery_number = read_u8(src)?;
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
        let vec = hex::decode("1044dbcbfbc92291610d0000003139322e3136382e312e31353007d500000c00000037312e33362e3132352e363307d5000016000000732d752d7573666e2d6f72696f6e3a7573666e617673").unwrap();
        let mut bytes = BytesMut::from(vec.as_slice());
        let response = Response::deserialize(&mut bytes);
        if let Response::NatPunch(nat_punch) = response {
            assert_eq!("192.168.1.150", nat_punch.local_host);
            assert_eq!(54535, nat_punch.local_port);
            assert_eq!("71.36.125.63", nat_punch.remote_host);
            assert_eq!(54535, nat_punch.remote_port);
            assert_eq!("s-u-usfn-orion:usfnavs", nat_punch.short_identifier);
        } else {
            panic!("response was not a NatPunch");
        }
    }

    #[test]
    fn test_deserialize_nat_punch_error() {
        let vec = hex::decode("08250000005345525645525f474f4e453a732d752d6e796172756b6f3a6d792d686f6d652d776f726c64").unwrap();
        let mut bytes = BytesMut::from(vec.as_slice());
        let response = Response::deserialize(&mut bytes);
        if let Response::NatPunchError(nat_error) = response {
            assert_eq!("SERVER_GONE:s-u-nyaruko:my-home-world", nat_error.response);
        } else {
            panic!("response was not a NatPunchError");
        }
    }

    #[test]
    fn test_deserialize_connect() {
        let vec = hex::decode("104a4f5c2e7984f8d214000000732d752d6c657865766f6e65743a6c65786c616201").unwrap();
        let mut bytes = BytesMut::from(vec.as_slice());
        let response = Response::deserialize(&mut bytes);
        if let Response::Connect(connect) = response {
            assert_eq!(1, connect.mystery_number);
            assert_eq!("s-u-lexevonet:lexlab", connect.short_identifier);
        } else {
            panic!("response was not a Connect");
        }
    }
}
