use crate::{Bp35c0, Result};
use crate::cmd::{Decode, Encode, Response};
use crate::payload::Payload;

const SKVER: &[u8] = b"SKVER";

#[derive(Clone, Debug)]
pub struct Input {}

impl Encode for Input {
    fn encode(&self) -> Payload {
        Payload {
            name: SKVER.into(),
            args: vec![],
        }
    }
}

#[derive(Clone, Debug)]
pub struct Output {
    pub version: String,
}

impl Decode for Output {
    fn decode(payload: &Payload) -> Self {
        Self {
            version: String::from_utf8_lossy(&payload.args[0]).to_string(),
        }
    }
}

impl Response for Output {
    const NAME: &'static [u8] = b"EVER";
}

impl Bp35c0 {
    pub fn version(&mut self) -> Result<Output> {
        unsafe {
            self.send(&Input {})?;
            self.wait_for_response::<Output>()
        }
    }
}
