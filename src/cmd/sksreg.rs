use byteorder::{BigEndian, ByteOrder};

use crate::{Bp35c0, Result};
use crate::cmd::{Decode, Encode, Response};
use crate::payload::Payload;
use crate::utils::{itoa, parse_hex_bytes, to_hex_bytes};

const SKSREG: &[u8] = b"SKSREG";

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum Register {
    /// 自端末が使用する周波数の論理チャネル番号
    ///
    /// 初期値: 0x21, 値域: 0x21 - 0x3C
    S02 = 0x02,

    /// 自端末の PAN ID
    /// 0xFFFF を除いて、B, H 面で同じ PAN ID を設定することはできません。
    /// それぞれの面で値は保存されます。
    ///
    /// 初期値: 0xFFFF, 値域: 0x0000 - 0xFFFF
    S03 = 0x03,

    S07 = 0x07,
    S0A = 0x0A,
    S0B = 0x0B,
    S15 = 0x15,
    S16 = 0x16,
    S17 = 0x17,
    S1C = 0x1C,
    SA1 = 0xA1,
    SA2 = 0xA2,
    SA9 = 0xA9,
    SF0 = 0xF0,
    SFB = 0xFB,
    SFD = 0xFD,

    /// エコーバックフラグ
    ///
    /// 0: コマンド入力のエコーバックをしない
    /// 1: エコーバックあり
    ///
    /// 初期値: 1, 値域: 0 or 1
    SFE = 0xFE,
    SFF = 0xFF,
}

#[derive(Clone, Debug)]
pub enum Value {
    Bool(bool),
    Uint8(u8),
    Uint16(u16),
    Uint32(u32),
}

impl From<&Value> for Vec<u8> {
    fn from(value: &Value) -> Self {
        match value {
            Value::Bool(b) => Self::from(if *b { b"1" } else { b"0" }),
            Value::Uint8(u) => itoa(*u).to_vec(),
            Value::Uint16(u) => {
                let mut buf = [0u8, 2];
                BigEndian::write_u16(&mut buf, *u);
                to_hex_bytes(&buf)
            }
            Value::Uint32(u) => {
                let mut buf = [0u8, 4];
                BigEndian::write_u32(&mut buf, *u);
                to_hex_bytes(&buf)
            }
        }
    }
}

impl From<&[u8]> for Value {
    fn from(value: &[u8]) -> Self {
        if value == b"1" {
            return Self::Bool(true);
        }

        if value == b"0" {
            return Self::Bool(false);
        }

        match value.len() {
            1 => match value[0] {
                b'1' => Self::Bool(true),
                b'0' => Self::Bool(false),
                v => panic!("Illegal value: {}", v),
            },
            2 => Self::Uint8(parse_hex_bytes(value)[0]),
            4 => Self::Uint16(BigEndian::read_u16(&parse_hex_bytes(value))),
            8 => Self::Uint32(BigEndian::read_u32(&parse_hex_bytes(value))),
            l => panic!("Illegal length of value: {}", l),
        }
    }
}

pub struct Input {
    pub register: Register,
    pub value: Option<Value>,
}

impl Encode for Input {
    fn encode(&self) -> Payload {
        let mut reg = vec![b'S'];
        reg.extend(itoa(self.register as u8));

        let mut args = vec![reg];
        if let Some(v) = &self.value {
            args.push(v.into());
        }

        Payload {
            name: SKSREG.into(),
            args,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Output {
    pub value: Value,
}

impl Decode for Output {
    fn decode(payload: &Payload) -> Self {
        Self {
            value: payload.args[0].as_slice().into(),
        }
    }
}

impl Response for Output {
    const NAME: &'static [u8] = b"ESREG";
}

impl Bp35c0 {
    pub unsafe fn read_register(&mut self, register: Register) -> Result<Output> {
        self.send(&Input {
            register,
            value: None,
        })?;
        self.wait_for_response()
    }

    pub unsafe fn set_register(&mut self, register: Register, value: Value) -> Result<()> {
        self.send(&Input {
            register,
            value: Some(value),
        })?;
        self.wait_for_ok()?;
        Ok(())
    }
}
