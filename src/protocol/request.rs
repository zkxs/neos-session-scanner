pub enum Request {
    NatPunch(NatPunch),
    Connect(Connect),
}

impl Request {
    pub fn nat_punch(host: String, port: u32, full_identifier: String) -> Request {
        Request::NatPunch(NatPunch { host, port, full_identifier })
    }

    pub fn connect(short_identifier: String) -> Request {
        Request::Connect(Connect { mystery_number: 0, short_identifier })
    }
}

pub struct NatPunch {
    pub host: String,
    pub port: u32,
    /// uppercase, uses C; prefix
    pub full_identifier: String,
}

pub struct Connect {
    /// should be zero?
    pub mystery_number: u32,
    /// lowercase, no prefix
    pub short_identifier: String,
}
