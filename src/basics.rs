use base64::engine::{general_purpose, Engine as _};
use hex;

#[allow(dead_code)]
pub fn hex2base64(hex_str: &str) -> String {
    general_purpose::STANDARD_NO_PAD.encode(hex::decode(hex_str).unwrap())
}

#[allow(dead_code)]
pub fn fixed_xor(hex_str: &str, other_hex_str: &str) -> String {
    let hex_bytes = hex::decode(hex_str).unwrap();
    let other_hex_bytes = hex::decode(other_hex_str).unwrap();

    let xor_result: Vec<u8> = hex_bytes
        .iter()
        .zip(other_hex_bytes)
        .map(|(byte, other_byte)| byte ^ other_byte)
        .collect();

    hex::encode(xor_result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex2base64() {
        let hex_str = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let expected_result = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

        assert_eq!(hex2base64(hex_str), expected_result);
    }

    #[test]
    fn test_fixed_xor() {
        let hex_str = "1c0111001f010100061a024b53535009181c";
        let other_hex_str = "686974207468652062756c6c277320657965";
        let expected_result = "746865206b696420646f6e277420706c6179";

        assert_eq!(fixed_xor(hex_str, other_hex_str), expected_result);
    }
}
