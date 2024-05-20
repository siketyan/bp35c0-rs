use std::net::Ipv6Addr;
use std::str::FromStr;

use byteorder::{BigEndian, ByteOrder};

use crate::{Bp35c0, Result};
use crate::cmd::{Decode, Encode, Response};
use crate::payload::Payload;
use crate::utils::parse_hex_bytes;

const SKINFO: &[u8] = b"SKINFO";

#[derive(Clone, Debug)]
pub struct Input {}

impl Encode for Input {
    fn encode(&self) -> Payload {
        Payload {
            name: SKINFO.into(),
            args: vec![],
        }
    }
}

#[derive(Clone, Debug)]
pub struct Output {
    pub ip_addr: Ipv6Addr,
    pub addr_64: [u8; 8],
    pub channel: u8,
    pub pan_id: u16,
    pub side: u16,
}

impl Decode for Output {
    fn decode(payload: &Payload) -> Self {
        Self {
            ip_addr: Ipv6Addr::from_str(&String::from_utf8_lossy(&payload.args[0])).unwrap(),
            addr_64: <[u8; 8]>::try_from(parse_hex_bytes(&payload.args[1])).unwrap(),
            channel: parse_hex_bytes(&payload.args[2])[0],
            pan_id: BigEndian::read_u16(&parse_hex_bytes(&payload.args[3])),
            side: match payload.args[4][0] {
                b'1' => 1,
                _ => 0,
            },
        }
    }
}

impl Response for Output {
    const NAME: &'static [u8] = b"EINFO";
}

impl Bp35c0 {
    pub fn info(&mut self) -> Result<Output> {
        unsafe {
            self.send(&Input {})?;
            self.wait_for_response::<Output>()
        }
    }
}
