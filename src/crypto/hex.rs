pub fn encode(data: &[u8]) -> String {
    let mut s = String::new();
    s.reserve_exact(data.len());

    for b in data {
        s.push_str(&format!("{:02x}", b));
    }

    s
}

pub fn decode(hex: &str) -> Vec<u8> {
    let mut bytes = Vec::new();
    bytes.reserve_exact(hex.len() / 2);

    for b in hex.as_bytes().chunks(2) {
        bytes.push(u8::from_str_radix(std::str::from_utf8(b).unwrap(), 16).unwrap());
    }

    bytes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        let data = [0x00, 0x01, 0x02, 0x03];
        let hex = encode(&data);
        assert_eq!(hex, "00010203");
    }

    #[test]
    fn test_decode() {
        let hex = "00010203";
        let data = decode(hex);
        assert_eq!(data, [0x00, 0x01, 0x02, 0x03]);
    }
}