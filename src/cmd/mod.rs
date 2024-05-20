use crate::payload::Payload;

pub mod skinfo;
pub mod skjoin;
pub mod skll64;
pub mod skreset;
pub mod skscan;
pub mod sksetpwd;
pub mod sksetrbid;
pub mod sksreg;
pub mod skver;

pub trait Encode {
    fn encode(&self) -> Payload;
}

pub trait Decode: Sized {
    fn decode(payload: &Payload) -> Self;
}

pub trait Response: Decode {
    const NAME: &'static [u8];
}
