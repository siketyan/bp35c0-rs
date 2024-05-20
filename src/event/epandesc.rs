use byteorder::{BigEndian, ByteOrder};
use tracing::debug;

use crate::{Bp35c0, Result};
use crate::payload::Payload;
use crate::utils::parse_hex_bytes;

pub(crate) const EPANDESC: &[u8] = b"EPANDESC";

#[derive(Clone, Debug)]
pub struct EPanDesc {
    pub channel: u8,
    pub channel_page: u8,
    pub pan_id: u16,
    pub addr: [u8; 8],
    pub lqi: u8,
    pub side: u8,
    pub pair_id: Option<[u8; 2]>,
}

impl Bp35c0 {
    pub unsafe fn receive_epandesc(&mut self) -> Result<EPanDesc> {
        let mut desc = EPanDesc {
            channel: 0,
            channel_page: 0,
            pan_id: 0,
            addr: [0u8; 8],
            lqi: 0,
            side: 0,
            pair_id: None,
        };

        loop {
            let line = self.receive_until_crlf()?;

            debug!("< {}", String::from_utf8_lossy(&line));

            let line = match line.strip_prefix(b"  ") {
                Some(l) => l,
                _ => {
                    self.buf.push_back(Payload::from(line));
                    break;
                }
            };

            if let Some(ch) = line.strip_prefix(b"Channel:") {
                desc.channel = parse_hex_bytes(ch)[0];
            }

            if let Some(chp) = line.strip_prefix(b"Channel Page:") {
                desc.channel_page = parse_hex_bytes(chp)[0];
            }

            if let Some(pid) = line.strip_prefix(b"Pan ID:") {
                desc.pan_id = BigEndian::read_u16(&parse_hex_bytes(pid));
            }

            if let Some(addr) = line.strip_prefix(b"Addr:") {
                desc.addr = parse_hex_bytes(addr).try_into().unwrap();
            }
        }

        Ok(desc)
    }
}
