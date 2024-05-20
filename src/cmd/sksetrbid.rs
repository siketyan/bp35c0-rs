use crate::{Bp35c0, Result};
use crate::cmd::Encode;
use crate::payload::Payload;
use crate::utils::to_hex_bytes;

const SKSETRBID: &[u8] = b"SKSETRBID";

#[derive(Clone, Debug)]
pub struct Input {
    rbid: [u8; 16],
}

impl Encode for Input {
    fn encode(&self) -> Payload {
        Payload {
            name: SKSETRBID.into(),
            args: vec![to_hex_bytes(&self.rbid)],
        }
    }
}

impl Bp35c0 {
    pub fn set_rbid(&mut self, rbid: [u8; 16]) -> Result<()> {
        unsafe {
            self.send(&Input { rbid })?;
            self.wait_for_ok()
        }
    }
}
