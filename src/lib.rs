#![allow(clippy::missing_safety_doc)]

use std::collections::VecDeque;
use std::io::{BufRead, BufReader, Write};

use serialport::SerialPort;
use tracing::debug;

use crate::cmd::*;
use crate::event::{Event, EVENT, RawEvent};
use crate::payload::Payload;

pub mod cmd;
mod event;
mod payload;
mod utils;

type Result<T> = std::result::Result<T, serialport::Error>;

const CR: u8 = b'\r';
const LF: u8 = b'\n';
const CRLF: &[u8] = &[CR, LF];
const OK: &[u8] = b"OK";

pub struct Bp35c0<Port = Box<dyn SerialPort>> {
    port: Port,
    reader: BufReader<Port>,
    buf: VecDeque<Payload>,
}

pub enum WaitMap<T> {
    Consume,
    Continue(Payload),
    Finish(T),
}

impl Bp35c0 {
    pub fn connect(port: Box<dyn SerialPort>) -> Result<Self> {
        let reader = BufReader::new(port.try_clone()?);
        let mut this = Self {
            port,
            reader,
            buf: Default::default(),
        };

        unsafe {
            // バッファに溜まっているコマンドと被って SKRESET がエラーにならないように CRLF を送信
            this.send_crlf()?;

            // リセット
            this.reset()?;

            // エコーバックは要らないので切っておく
            this.set_register(sksreg::Register::SFE, sksreg::Value::Bool(false))?;
        }

        Ok(this)
    }

    pub(crate) unsafe fn receive_until_crlf(&mut self) -> Result<Vec<u8>> {
        let mut buf = Vec::new();

        self.reader.read_until(LF, &mut buf)?;

        if let Some(&LF) = buf.last() {
            _ = buf.pop();
        }

        if let Some(&CR) = buf.last() {
            _ = buf.pop();
        }

        Ok(buf)
    }

    pub unsafe fn receive_payload(&mut self) -> Result<Payload> {
        self.buf
            .pop_front()
            .map(Ok)
            .unwrap_or_else(|| self.receive_payload_unbuffered())
    }

    pub unsafe fn receive_payload_unbuffered(&mut self) -> Result<Payload> {
        loop {
            let payload = Payload::from(self.receive_until_crlf()?);
            if payload.name.starts_with(b"SK")
                || payload.name.starts_with(b"W")
                || payload.name.starts_with(b"R")
            {
                continue; // skipping echo-backs
            }

            debug!("< {payload:?}");

            return Ok(payload);
        }
    }

    unsafe fn send_crlf(&mut self) -> Result<()> {
        self.port.write_all(CRLF)?;
        Ok(())
    }

    pub unsafe fn send_payload(&mut self, payload: &Payload) -> Result<()> {
        debug!("> {payload:?}");

        self.port.write_all(Vec::<u8>::from(payload).as_slice())?;
        self.send_crlf()?;
        Ok(())
    }

    pub unsafe fn send<E>(&mut self, input: &E) -> Result<()>
    where
        E: Encode,
    {
        self.send_payload(&input.encode())
    }

    pub unsafe fn wait_map<F, T>(&mut self, mut f: F) -> Result<T>
    where
        F: FnMut(Payload) -> WaitMap<T>,
    {
        let mut buf = Vec::<Payload>::new();

        let value = loop {
            match f(self.receive_payload()?) {
                WaitMap::Consume => {}
                WaitMap::Continue(payload) => {
                    buf.push(payload);
                }
                WaitMap::Finish(value) => {
                    break value;
                }
            }
        };

        buf.into_iter().for_each(|p| self.buf.push_back(p));

        return Ok(value);
    }

    pub unsafe fn wait_for<F>(&mut self, criteria: F) -> Result<Payload>
    where
        F: Fn(&Payload) -> bool,
    {
        self.wait_map(|p| {
            if criteria(&p) {
                WaitMap::Finish(p)
            } else {
                WaitMap::Continue(p)
            }
        })
    }

    pub unsafe fn wait_for_event<F>(&mut self, criteria: F) -> Result<Event>
    where
        F: Fn(&Event) -> bool,
    {
        self.wait_map(|p| {
            if p.name == EVENT {
                let event = Event::from(&RawEvent::from(&p));
                if criteria(&event) {
                    return WaitMap::Finish(event);
                }
            }

            WaitMap::Continue(p)
        })
    }

    pub unsafe fn wait_for_ok(&mut self) -> Result<()> {
        self.wait_for(|p| p.name == OK)?;
        Ok(())
    }

    pub unsafe fn wait_for_response<R>(&mut self) -> Result<R>
    where
        R: Response,
    {
        let payload = self.wait_for(|p| p.name == R::NAME)?;
        let response = R::decode(&payload);
        self.wait_for_ok()?;
        Ok(response)
    }
}
