use base64::engine::{general_purpose, Engine as _};
use hex;
use itertools::Itertools;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref LETTER_FREQUENCY: HashMap<char, f64> = {
        let mut map = HashMap::new();

        map.insert('A', 0.0812);
        map.insert('B', 0.0154);
        map.insert('C', 0.0271);
        map.insert('D', 0.0432);
        map.insert('E', 0.1202);
        map.insert('F', 0.0230);
        map.insert('G', 0.0203);
        map.insert('H', 0.0609);
        map.insert('I', 0.0731);
        map.insert('J', 0.0010);
        map.insert('K', 0.0077);
        map.insert('L', 0.0398);
        map.insert('M', 0.0261);
        map.insert('N', 0.0695);
        map.insert('O', 0.0768);
        map.insert('P', 0.0166);
        map.insert('Q', 0.0010);
        map.insert('R', 0.0592);
        map.insert('S', 0.0628);
        map.insert('T', 0.0910);
        map.insert('U', 0.0288);
        map.insert('V', 0.0099);
        map.insert('W', 0.0209);
        map.insert('X', 0.0015);
        map.insert('Y', 0.0192);
        map.insert('Z', 0.0007);
        map.insert(' ', 0.1829);

        map
    };
}

#[allow(dead_code)]
pub fn hex2base64(hex_str: &str) -> String {
    general_purpose::STANDARD_NO_PAD.encode(hex::decode(hex_str).unwrap())
}

#[allow(dead_code)]
pub fn xor_encode(hex_str: &str, other_hex_str: &str) -> String {
    let hex_bytes = hex::decode(hex_str).unwrap();
    let other_hex_bytes = hex::decode(other_hex_str).unwrap();

    let xor_result: Vec<u8> = hex_bytes
        .iter()
        .zip(other_hex_bytes)
        .map(|(byte, other_byte)| byte ^ other_byte)
        .collect::<Vec<_>>();

    hex::encode(xor_result)
}

#[allow(dead_code)]
pub fn xor_decode_single_char(hex_str: &str, key: char) -> String {
    let hex_bytes = hex::decode(hex_str).unwrap();

    let result: Vec<u8> = hex_bytes
        .iter()
        .map(|byte| byte ^ key as u8)
        .collect::<Vec<_>>();
    String::from_utf8(result).unwrap_or_default()
}

#[allow(dead_code)]
pub fn xor_encode_repeating_key(text: &str, key: &str) -> String {
    let xor_result = text
        .bytes()
        .zip(key.bytes().cycle())
        .map(|(letter, key_char)| letter ^ key_char)
        .collect::<Vec<_>>();

    hex::encode(xor_result)
}

#[allow(dead_code)]
pub fn englishness(text: &str) -> f64 {
    let total_chars = text.len() as f64;
    text.chars()
        .counts()
        .iter()
        .map(|(letter, &count)| {
            f64::sqrt(
                *LETTER_FREQUENCY
                    .get(&letter.to_ascii_uppercase())
                    .unwrap_or(&0.0)
                    * count as f64
                    / total_chars,
            )
        })
        .sum()
}

#[allow(dead_code)]
pub fn xor_bruteforce_single_char(hex_str: &str) -> String {
    let ascii_chars: Vec<char> = (0..=127).map(|x| x as u8 as char).collect();
    let text2score = ascii_chars.iter().map(|&c| {
        let text = xor_decode_single_char(hex_str, c);
        let score = englishness(&text);
        (text, score)
    });
    text2score
        .max_by(|a, b| a.1.total_cmp(&b.1))
        .map(|(text, _)| text)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

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

        assert_eq!(xor_encode(hex_str, other_hex_str), expected_result);
    }

    #[test]
    fn test_bruteforce_xor_cipher() {
        let hex_str = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
        let expected_result = "Cooking MC's like a pound of bacon";

        assert_eq!(xor_bruteforce_single_char(hex_str), expected_result);
    }

    #[test]
    fn test_bruteforce_xor_cipher_complex() {
        let file = File::open("data/xor_cipher.in").unwrap();
        let reader = BufReader::new(file);
        let expected_result = "Now that the party is jumping\n";

        let result = reader
            .lines()
            .map(|line| xor_bruteforce_single_char(&line.unwrap()))
            .max_by(|a, b| englishness(a).total_cmp(&englishness(b)))
            .unwrap();
        assert_eq!(result, expected_result);
    }

    #[test]
    fn tets_repeating_key_xor() {
        let text = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
        let key = "ICE";
        let expected_result= "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";

        assert_eq!(xor_encode_repeating_key(text, key), expected_result);
    }
}
