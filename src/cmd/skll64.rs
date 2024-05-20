use std::net::Ipv6Addr;
use std::str::FromStr;

use crate::{Bp35c0, Result};
use crate::cmd::Encode;
use crate::payload::Payload;
use crate::utils::to_hex_bytes;

const SKLL64: &[u8] = b"SKLL64";

#[derive(Clone, Debug)]
pub struct Input {
    pub addr_64: [u8; 8],
}

impl Encode for Input {
    fn encode(&self) -> Payload {
        Payload {
            name: SKLL64.into(),
            args: vec![to_hex_bytes(&self.addr_64)],
        }
    }
}

#[derive(Clone, Debug)]
pub struct Output {
    pub ip_addr: Ipv6Addr,
}

impl From<&[u8]> for Output {
    fn from(value: &[u8]) -> Self {
        Self {
            ip_addr: Ipv6Addr::from_str(&String::from_utf8_lossy(value)).unwrap(),
        }
    }
}

impl Bp35c0 {
    pub fn mac_to_ip_addr(&mut self, addr_64: [u8; 8]) -> Result<Output> {
        unsafe {
            self.send(&Input { addr_64 })?;
            Ok(self.receive_until_crlf()?.as_slice().into())
        }
    }
}
