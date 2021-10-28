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
        dst.put_u8(0x10);

        // packet hash
        dst.put_u64(0x88BE1026BFB1669C);

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
    /// should be 0x01
    pub mystery_number: u8,
    /// lowercase, no prefix
    pub short_identifier: String,
}

impl Serialize for Connect {
    fn serialize(self, dst: &mut BytesMut) -> Result<(), Error> {
        // magic number
        dst.put_u8(0x10);

        // packet hash
        dst.put_u64(0x4A4F5C2E7984F8D2);

        // length prefixed string: identifier
        let identifier_bytes = self.short_identifier.as_bytes();
        dst.put_u32_le(identifier_bytes.len().try_into().expect("could not fit usize into u32"));
        dst.put(identifier_bytes);

        // mystery number
        dst.put_u8(self.mystery_number);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use bytes::BytesMut;

    use crate::lnl_protocol::request::{Connect, NatPunch, Serialize};

    #[test]
    fn test_serialize_nat_punch() {
        let expected = hex::decode("1088be1026bfb1669c0b0000003137322e32322e302e3131e3ca000028000000433b532d33323434336264392d633730392d343739382d396636352d633339326432643230316134").unwrap();
        let mut bytes = BytesMut::new();

        let request = NatPunch { host: "172.22.0.11".into(), port: 51939, full_identifier: "C;S-32443bd9-c709-4798-9f65-c392d2d201a4".into() };
        request.serialize(&mut bytes).unwrap();
        let actual: &[u8] = bytes.as_ref();
        assert_eq!(expected.as_slice(), actual);
    }

    #[test]
    fn test_serialize_connect() {
        let expected = hex::decode("104a4f5c2e7984f8d214000000732d752d6c657865766f6e65743a6c65786c616201").unwrap();
        let mut bytes = BytesMut::new();

        let request = Connect { mystery_number: 1, short_identifier: "s-u-lexevonet:lexlab".into() };
        request.serialize(&mut bytes).unwrap();
        let actual: &[u8] = bytes.as_ref();
        assert_eq!(expected.as_slice(), actual);
    }
}
