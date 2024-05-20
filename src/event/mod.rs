use std::net::Ipv6Addr;
use std::str::FromStr;

use crate::payload::Payload;
use crate::utils::parse_hex_bytes;

pub mod epandesc;

pub const EVENT: &[u8] = b"EVENT";

#[allow(dead_code)]
#[repr(u8)]
#[derive(Copy, Clone, Debug)]
enum EventType {
    NSReceived = 0x01,
    NAReceived = 0x02,
    EchoRequest = 0x05,
    EDScanFinished = 0x1F,
    Beacon = 0x20,
    UDPSendFinished = 0x21,
    ActiveScanFinished = 0x22,
    PanaError = 0x24,
    PanaConnected = 0x25,
    PanaTerminationRequest = 0x26,
    PanaTerminated = 0x27,
    PanaTerminationTimeout = 0x28,
    PanaTimedOut = 0x29,
    Arib108QuotaExceeded = 0x32,
    Arib108QuotaRecovered = 0x33,
    InvalidCipherReceived = 0x45,
    KeyUpdateTimedOut = 0x46,
    KeyUpdateRequested = 0x50,
    KeyUpdateResponse = 0x51,
    KeyUpdateNoResponse = 0x52,
    KeyRequest = 0x53,
    KeyDistributionStarted = 0x54,
    KeyDistributionFinished = 0x55,
    InitialSetupStarted = 0x56,
    InitialSetupFinished = 0x57,
}

pub struct RawEvent {
    pub num: u8,
    pub sender: Ipv6Addr,
    pub side: u8,
    pub param: Option<Vec<u8>>,
}

impl From<&Payload> for RawEvent {
    fn from(value: &Payload) -> Self {
        Self {
            num: parse_hex_bytes(&value.args[0])[0],
            sender: Ipv6Addr::from_str(&String::from_utf8_lossy(&value.args[1])).unwrap(),
            side: if value.args[2][0] == b'1' { 1 } else { 0 },
            param: value.args.get(3).map(|a| a.to_vec()),
        }
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum UDPSendResult {
    Success = 0,
    Failure = 1,
    NSDispatched = 2,
}

#[derive(Clone, Debug)]
pub enum EventBody {
    NSReceived,
    NAReceived,
    EchoRequest,
    EDScanFinished,
    Beacon,
    UDPSendFinished { result: UDPSendResult },
    ActiveScanFinished,
    PanaError,
    PanaConnected,
    PanaTerminationRequest,
    PanaTerminated,
    PanaTerminationTimeout,
    PanaTimedOut,
    Arib108QuotaExceeded,
    Arib108QuotaRecovered,
    InvalidCipherReceived { actual: u8 },
    KeyUpdateTimedOut,
    KeyUpdateRequested,
    KeyUpdateResponse,
    KeyUpdateNoResponse,
    KeyRequest,
    KeyDistributionStarted,
    KeyDistributionFinished,
    InitialSetupStarted,
    InitialSetupFinished,
}

impl From<&RawEvent> for EventBody {
    fn from(value: &RawEvent) -> Self {
        match unsafe { std::mem::transmute::<u8, EventType>(value.num) } {
            EventType::NSReceived => Self::NSReceived,
            EventType::NAReceived => Self::NAReceived,
            EventType::EchoRequest => Self::EchoRequest,
            EventType::EDScanFinished => Self::EDScanFinished,
            EventType::Beacon => Self::Beacon,
            EventType::UDPSendFinished => Self::UDPSendFinished {
                result: unsafe { std::mem::transmute(value.param.as_ref().unwrap()[0]) },
            },
            EventType::ActiveScanFinished => Self::ActiveScanFinished,
            EventType::PanaError => Self::PanaError,
            EventType::PanaConnected => Self::PanaConnected,
            EventType::PanaTerminationRequest => Self::PanaTerminationRequest,
            EventType::PanaTerminated => Self::PanaTerminated,
            EventType::PanaTerminationTimeout => Self::PanaTerminationTimeout,
            EventType::PanaTimedOut => Self::PanaTimedOut,
            EventType::Arib108QuotaExceeded => Self::Arib108QuotaExceeded,
            EventType::Arib108QuotaRecovered => Self::Arib108QuotaRecovered,
            EventType::InvalidCipherReceived => Self::InvalidCipherReceived {
                actual: value.param.as_ref().unwrap()[0],
            },
            EventType::KeyUpdateTimedOut => Self::KeyUpdateTimedOut,
            EventType::KeyUpdateRequested => Self::KeyUpdateRequested,
            EventType::KeyUpdateResponse => Self::KeyUpdateResponse,
            EventType::KeyUpdateNoResponse => Self::KeyUpdateNoResponse,
            EventType::KeyRequest => Self::KeyRequest,
            EventType::KeyDistributionStarted => Self::KeyDistributionStarted,
            EventType::KeyDistributionFinished => Self::KeyDistributionFinished,
            EventType::InitialSetupStarted => Self::InitialSetupStarted,
            EventType::InitialSetupFinished => Self::InitialSetupFinished,
        }
    }
}

pub struct Header {
    pub sender: Ipv6Addr,
    pub side: u8,
}

pub struct Event {
    pub header: Header,
    pub body: EventBody,
}

impl From<&RawEvent> for Event {
    fn from(value: &RawEvent) -> Self {
        Self {
            header: Header {
                sender: value.sender,
                side: value.side,
            },
            body: value.into(),
        }
    }
}
