use base64::engine::{general_purpose, Engine as _};
use hex;

#[allow(dead_code)]
pub fn hex2base64(hex_str: &str) -> String {
    general_purpose::STANDARD_NO_PAD.encode(hex::decode(hex_str).unwrap())
}

#[cfg(test)]
mod tests {
    use super::hex2base64;

    #[test]
    fn it_converts() {
        let hex_str = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let base64_str = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

        assert_eq!(hex2base64(hex_str), base64_str);
    }
}
