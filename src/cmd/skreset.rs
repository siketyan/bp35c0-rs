use crate::{Bp35c0, Result};
use crate::cmd::Encode;
use crate::payload::Payload;

const SKRESET: &[u8] = b"SKRESET";

#[derive(Clone, Debug)]
pub struct Input {}

impl Encode for Input {
    fn encode(&self) -> Payload {
        Payload {
            name: SKRESET.into(),
            args: vec![],
        }
    }
}

impl Bp35c0 {
    pub unsafe fn reset(&mut self) -> Result<()> {
        self.send(&Input {})?;
        self.wait_for_ok()
    }
}
