use std::net::Ipv6Addr;

use bstr::BString;

use crate::{Bp35c0, Result};
use crate::cmd::Encode;
use crate::event::EventBody;
use crate::payload::Payload;
use crate::utils::u16_to_hex_bytes;

const SKJOIN: &[u8] = b"SKJOIN";

#[derive(Clone, Debug)]
pub struct Input {
    pub ip_addr: Ipv6Addr,
}

impl Encode for Input {
    fn encode(&self) -> Payload {
        Payload {
            name: SKJOIN.into(),
            args: vec![bstr::join(
                b":",
                self.ip_addr
                    .segments()
                    .map(u16_to_hex_bytes)
                    .map(BString::from),
            )],
        }
    }
}

impl Bp35c0 {
    pub unsafe fn join_nowait(&mut self, ip_addr: Ipv6Addr) -> Result<()> {
        self.send(&Input { ip_addr })
    }

    pub fn join(&mut self, ip_addr: Ipv6Addr) -> Result<()> {
        unsafe {
            self.join_nowait(ip_addr)?;
            self.wait_for_event(|e| matches!(e.body, EventBody::PanaConnected))?;
            self.wait_for_ok()
        }
    }
}
