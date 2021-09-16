use std::convert::TryInto;

use bytes::{BufMut, BytesMut};

use crate::lnl_protocol::Error;

pub trait Serialize {
    fn serialize(self, dst: &mut BytesMut) -> Result<(), Error>;
}

pub struct NatPunch {
    pub host: String,
    pub port: u32,
    /// uppercase, uses C; prefix
    pub full_identifier: String,
}

impl Serialize for NatPunch {
    fn serialize(self, dst: &mut BytesMut) -> Result<(), Error> {
        // magic number
        dst.put_u8(0x0C);

        // length prefixed string: host
        let host_bytes = self.host.as_bytes();
        dst.put_u32_le(host_bytes.len().try_into().expect("could not fit usize into u32"));
        dst.put(host_bytes);

        // port
        dst.put_u32_le(self.port);

        // length prefixed string: identifier
        let identifier_bytes = self.full_identifier.as_bytes();
        dst.put_u32_le(identifier_bytes.len().try_into().expect("could not fit usize into u32"));
        dst.put(identifier_bytes);

        Ok(())
    }
}

pub struct Connect {
    /// should be zero?
    pub mystery_number: u8,
    /// lowercase, no prefix
    pub short_identifier: String,
}

impl Serialize for Connect {
    fn serialize(self, dst: &mut BytesMut) -> Result<(), Error> {
        // magic number
        dst.put_u8(0x0E);

        // mystery number
        dst.put_u8(self.mystery_number);

        // length prefixed string: identifier
        let identifier_bytes = self.short_identifier.as_bytes();
        dst.put_u32_le(identifier_bytes.len().try_into().expect("could not fit usize into u32"));
        dst.put(identifier_bytes);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use bytes::BytesMut;

    use crate::lnl_protocol::request::{Connect, NatPunch, Serialize};

    #[test]
    fn test_serialize_nat_punch() {
        let expected = hex::decode("0C0F0000003136392E3235342E3230332E323234D20400001E000000433B532D552D5553464E2D4F72696F6E3A5553464E4C756E61725061726B").unwrap();
        let mut bytes = BytesMut::new();

        let request = NatPunch { host: "169.254.203.224".into(), port: 1234, full_identifier: "C;S-U-USFN-Orion:USFNLunarPark".into() };
        request.serialize(&mut bytes).unwrap();
        let actual: &[u8] = bytes.as_ref();
        assert_eq!(expected.as_slice(), actual);
    }

    #[test]
    fn test_serialize_connect() {
        let expected = hex::decode("0e001c000000732d752d7573666e2d6f72696f6e3a7573666e6c756e61727061726b").unwrap();
        let mut bytes = BytesMut::new();

        let request = Connect { mystery_number: 0, short_identifier: "s-u-usfn-orion:usfnlunarpark".into() };
        request.serialize(&mut bytes).unwrap();
        let actual: &[u8] = bytes.as_ref();
        assert_eq!(expected.as_slice(), actual);
    }
}
