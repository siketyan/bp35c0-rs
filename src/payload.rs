use std::fmt::{Debug, Formatter};

#[derive(Clone)]
pub struct Payload {
    pub name: Vec<u8>,
    pub args: Vec<Vec<u8>>,
}

impl Debug for Payload {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = String::from_utf8_lossy(&self.name);
        if self.args.is_empty() {
            write!(f, "{}", name)
        } else {
            write!(
                f,
                "{} {}",
                name,
                self.args
                    .iter()
                    .map(|arg| match arg.is_ascii() {
                        true => String::from_utf8_lossy(arg).to_string(),
                        _ => hex::encode(arg),
                    })
                    .collect::<Vec<_>>()
                    .join(" "),
            )
        }
    }
}

impl From<Vec<u8>> for Payload {
    fn from(value: Vec<u8>) -> Self {
        let parts = value
            .split(|v| *v == 0x20)
            .map(|v| v.to_vec())
            .collect::<Vec<_>>();

        if let Some((name, args)) = parts.split_first() {
            Self {
                name: name.to_vec(),
                args: args.to_vec(),
            }
        } else {
            panic!("Illegal payload");
        }
    }
}

impl From<&Payload> for Vec<u8> {
    fn from(value: &Payload) -> Self {
        let mut bytes = Self::new();

        bytes.extend_from_slice(&value.name);

        value.args.iter().for_each(|arg| {
            bytes.push(0x20);
            bytes.extend_from_slice(arg);
        });

        bytes
    }
}
