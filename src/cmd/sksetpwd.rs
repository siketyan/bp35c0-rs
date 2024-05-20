use crate::{Bp35c0, Result};
use crate::cmd::Encode;
use crate::payload::Payload;
use crate::utils::itoa;

const SKSETPWD: &[u8] = b"SKSETPWD";

#[derive(Clone, Debug)]
pub struct Input {
    pwd: Vec<u8>,
}

impl Encode for Input {
    fn encode(&self) -> Payload {
        Payload {
            name: SKSETPWD.into(),
            args: vec![itoa(self.pwd.len() as u8).into(), self.pwd.clone()],
        }
    }
}

impl Bp35c0 {
    pub fn set_pwd(&mut self, pwd: &[u8]) -> Result<()> {
        unsafe {
            self.send(&Input { pwd: pwd.to_vec() })?;
            self.wait_for_ok()
        }
    }
}
