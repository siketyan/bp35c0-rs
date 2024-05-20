use std::net::Ipv6Addr;

use bstr::BString;

use crate::{Bp35c0, Result};
use crate::cmd::Encode;
use crate::event::{Event, EVENT, EventBody, RawEvent};
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
        let mut buf = Vec::<Payload>::new();

        unsafe {
            self.join_nowait(ip_addr)?;

            loop {
                let payload = self.receive_payload()?;
                if payload.name == EVENT {
                    let event = Event::from(&RawEvent::from(&payload));
                    if let EventBody::PanaConnected = event.body {
                        self.wait_for_ok()?;
                        break;
                    }
                }

                buf.push(payload);
            }
        }

        buf.into_iter().for_each(|p| self.buf.push_back(p));

        Ok(())
    }
}
