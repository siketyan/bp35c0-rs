use std::env::args;
use std::time::Duration;

use anyhow::bail;
use tracing::{debug, info};
use tracing_subscriber::filter::LevelFilter;

use bp35c0::Bp35c0;
use bp35c0::cmd::*;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::DEBUG)
        .compact()
        .init();

    let args = args().collect::<Vec<_>>();
    if args.len() < 3 {
        bail!("Usage: <tty> <bid> <pwd>");
    }

    let tty = &args[0];
    let bid = &args[1];
    let pwd = &args[2];

    let port = serialport::new(tty, 115_200)
        .timeout(Duration::from_secs(1000))
        .open()?;

    let mut device = Bp35c0::connect(port)?;
    let skver::Output { version } = device.version()?;

    info!("Version: {version}");

    let info = device.info()?;

    info!("IP Address: {}", info.ip_addr);
    info!("MAC Address: {}", hex::encode(info.addr_64));
    info!("Channel: {0:#x} ({0})", info.channel);
    info!("PAN ID: {0:#x} ({0})", info.pan_id);
    info!("Active Side: {}", info.side);

    device.set_rbid(hex::decode(bid)?.try_into().unwrap())?;
    device.set_pwd(pwd.as_bytes())?;

    let descs = device.scan_active(true, 0xFFFFFFFF, 0x7, 0x0)?;
    let desc = match descs.first() {
        Some(d) => d,
        _ => bail!("Coordinator Not Found."),
    };

    info!("Coordinator Found: {}", hex::encode(desc.addr));

    let skll64::Output { ip_addr } = device.mac_to_ip_addr(desc.addr)?;

    info!("IP Address: {}", &ip_addr);

    unsafe {
        device.set_register(sksreg::Register::S02, sksreg::Value::Uint8(desc.channel))?;
        device.set_register(sksreg::Register::S03, sksreg::Value::Uint16(desc.pan_id))?;
    }

    info!("Joining to Network");

    device.join(ip_addr)?;

    loop {
        unsafe {
            let payload = device.receive_payload()?;
            debug!("< {payload:?}")
        }
    }
}
