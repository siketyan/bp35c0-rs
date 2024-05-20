use byteorder::{BigEndian, ByteOrder};

#[inline]
fn atoi(b: u8) -> u8 {
    if b <= b'9' {
        b - 48
    } else {
        b - 55
    }
}

pub(crate) fn itoa(i: u8) -> [u8; 2] {
    let left = i >> 4;
    let right = i & 0xF;

    [
        if left <= 0x9 { left + 48 } else { left + 55 },
        if right <= 0x9 { right + 48 } else { right + 55 },
    ]
}

pub(crate) fn parse_hex_bytes(src: &[u8]) -> Vec<u8> {
    #[allow(arithmetic_overflow)]
    src.chunks_exact(2)
        .map(|octet| (atoi(octet[0]) << 4) | atoi(octet[1]))
        .collect()
}

pub(crate) fn to_hex_bytes(src: &[u8]) -> Vec<u8> {
    src.iter().flat_map(|v| itoa(*v)).collect()
}

pub(crate) fn u16_to_hex_bytes(u: u16) -> [u8; 4] {
    let mut src = [0u8; 2];
    BigEndian::write_u16(&mut src, u);

    let mut dst = [0u8; 4];
    for i in 0..2 {
        let b = itoa(src[i]);
        dst[i * 2] = b[0];
        dst[i * 2 + 1] = b[1];
    }

    dst
}

pub(crate) fn u32_to_hex_bytes(u: u32) -> [u8; 8] {
    let mut src = [0u8; 4];
    BigEndian::write_u32(&mut src, u);

    let mut dst = [0u8; 8];
    for i in 0..4 {
        let b = itoa(src[i]);
        dst[i * 2] = b[0];
        dst[i * 2 + 1] = b[1];
    }

    dst
}

#[cfg(test)]
mod tests {
    use crate::utils::parse_hex_bytes;

    #[test]
    fn test_parse_hex_bytes() {
        assert_eq!(
            vec![0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF],
            parse_hex_bytes(b"0123456789ABCDEF"),
        );
    }
}
