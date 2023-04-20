use std::net::Ipv4Addr;


#[derive(Debug, Clone)]
pub enum Record {
    Unknown {
        domain: String,
        qtype: u16,
        qclass: u16,
        data_length: u16,
        ttl: u32,
    },

    A {
        domain: String,
        address: Ipv4Addr,
        ttl: u32,
    },
}

pub type Answer = Record;
pub type Authority = Record;
pub type AdditionalRecord = Record;