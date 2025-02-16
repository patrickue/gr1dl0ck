
use std::{fmt, num::ParseIntError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecodeHexError {
    OddLength,
    ParseInt(ParseIntError),
}

impl std::error::Error for DecodeHexError {}

impl From<ParseIntError> for DecodeHexError {
    fn from(e: ParseIntError) -> Self {
        DecodeHexError::ParseInt(e)
    }
}

impl fmt::Display for DecodeHexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DecodeHexError::OddLength => "input string has an odd number of bytes".fmt(f),
            DecodeHexError::ParseInt(e) => e.fmt(f),
        }
    }
}

pub fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, DecodeHexError> {
    if hex.len() % 2 != 0 {
        Err(DecodeHexError::OddLength)
    } else {
        (0..hex.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&hex[i..i + 2], 16)
                .map_err(|e| e.into()))
            .collect()
    }
}

fn bytes_to_base64(bytes: &[u8]) -> String {
    const BASE64_CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut base64 = String::new();
    for chunk in bytes.chunks(3) {
        let b0 = chunk[0];
        let b1 = chunk.get(1).copied().unwrap_or(0);
        let b2 = chunk.get(2).copied().unwrap_or(0);
        let indices = [
            b0 >> 2,
            ((b0 & 0x03) << 4) | (b1 >> 4),
            ((b1 & 0x0F) << 2) | (b2 >> 6),
            b2 & 0x3F,
        ];
        for &index in &indices {
            base64.push(BASE64_CHARS[usize::from(index)] as char);
        }
    }
    // Somewhat ugly and hacky
    match bytes.len() % 3 {
        1 => {
            base64.truncate(base64.len() - 2);
            base64.push_str("==");
        },
        2 => {
            base64.truncate(base64.len() - 1);
            base64.push_str("=");
        },
        _ => (),
    }
    base64
}

fn main() {
    let hex_input = "48656c6c6f20576f726c644";
    let bytes = hex_to_bytes(hex_input).unwrap();
    let base64_output = bytes_to_base64(&bytes);
    println!("Base64 encoded output: {}", base64_output);
}


// ==== Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn happy_path_hex_to_bytes() {
        assert_eq!(hex_to_bytes("afbe").unwrap(), vec![0xaf, 0xbe]);
    }

    #[test]
    fn fail_hex_to_bytes_odd_length() {
        let conversion_res = hex_to_bytes("afb");
        assert!(conversion_res.is_err());
        assert_eq!(conversion_res.unwrap_err(), DecodeHexError::OddLength);
    }

    #[test]
    fn happy_path_bytes_to_base64_no_padding() {
        // 0x4C, 0x69, 0x75, 0x62, 0x6F, 0x76,
        assert_eq!(bytes_to_base64(&[
            u8::from(0x4C),
            u8::from(0x69),
            u8::from(0x75),
            u8::from(0x62),
            u8::from(0x6F),
            u8::from(0x76), ]), "TGl1Ym92");
    }

    #[test]
    fn happy_path_bytes_to_base64_double_padding() {

        // 0x50, 0x61, 0x74, 0x72, 0x69, 0x63, 0x6B
        assert_eq!(bytes_to_base64(&[
            u8::from(0x50),
            u8::from(0x61),
            u8::from(0x74),
            u8::from(0x72),
            u8::from(0x69),
            u8::from(0x63),
            u8::from(0x6B)]), "UGF0cmljaw==");
    }
}