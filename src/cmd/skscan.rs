use crate::{Bp35c0, Result};
use crate::cmd::Encode;
use crate::event::{Event, EVENT, EventBody, RawEvent};
use crate::event::epandesc::{EPanDesc, EPANDESC};
use crate::payload::Payload;
use crate::utils::{itoa, u32_to_hex_bytes};

const SKSCAN: &[u8] = b"SKSCAN";

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum Mode {
    ED = 0,
    Active = 2,
    ActiveWithoutIE = 3,
}

#[derive(Clone, Debug)]
pub struct Input {
    pub mode: Mode,
    pub channel_mask: u32,
    pub duration: u8,
    pub side: u8,
}

impl Encode for Input {
    fn encode(&self) -> Payload {
        Payload {
            name: SKSCAN.into(),
            args: vec![
                itoa(self.mode as u8)[1..].into(),
                u32_to_hex_bytes(self.channel_mask).into(),
                itoa(self.duration).into(),
                itoa(self.side)[1..].into(),
            ],
        }
    }
}

impl Bp35c0 {
    pub unsafe fn scan_active_nowait(
        &mut self,
        ie: bool,
        channel_mask: u32,
        duration: u8,
        side: u8,
    ) -> Result<()> {
        self.send(&Input {
            mode: if ie {
                Mode::Active
            } else {
                Mode::ActiveWithoutIE
            },
            channel_mask,
            duration,
            side,
        })?;
        self.wait_for_ok()
    }

    pub fn scan_active(
        &mut self,
        ie: bool,
        channel_mask: u32,
        duration: u8,
        side: u8,
    ) -> Result<Vec<EPanDesc>> {
        let mut descs = Vec::<EPanDesc>::new();
        let mut buf = Vec::<Payload>::new();

        unsafe {
            self.scan_active_nowait(ie, channel_mask, duration, side)?;

            loop {
                let payload = self.receive_payload()?;

                if payload.name == EPANDESC {
                    descs.push(self.receive_epandesc()?);
                    continue;
                }

                if payload.name == EVENT {
                    let event = Event::from(&RawEvent::from(&payload));
                    if let EventBody::ActiveScanFinished = event.body {
                        break;
                    }
                }

                buf.push(payload);
            }
        }

        buf.into_iter().for_each(|p| self.buf.push_back(p));

        Ok(descs)
    }
}
