use crate::{decode_u8, encode_char};

/// This is an implementation of Crockford's base32 encoding
pub struct Crock32;

impl Crock32 {
    /// Encode a string of characters into a base32 encoded string
    pub fn encode<const N: usize>(input: [char; N]) -> Vec<u8> {
        let mut buffer: u32 = 0;
        let mut bits_in_buffer: i8 = 0;
        let mut result = Vec::with_capacity(N * 5 / 8);

        for &char in &input {
            let value = encode_char(&char) as u32;
            buffer = (buffer << 5) | value;
            bits_in_buffer += 5;

            while bits_in_buffer >= 8 {
                bits_in_buffer -= 8;
                let byte = (buffer >> bits_in_buffer) as u8;
                result.push(byte);
                buffer &= (1 << bits_in_buffer) - 1;
            }
        }

        result
    }

    pub fn decode<const N: usize>(input: [u8; N]) -> String {
        let mut buffer: u16 = 0;
        let mut bits_in_buffer: u8 = 0;
        let mut result = String::with_capacity(N * 8 / 5);

        dbg!(result.capacity());

        for &byte in &input {
            buffer = (buffer << 8) | byte as u16;
            bits_in_buffer += 8;

            while bits_in_buffer >= 5 {
                bits_in_buffer -= 5;
                let value = ((buffer >> bits_in_buffer) & 0x1F) as u16;
                let decode = decode_u8(value);
                dbg!(&decode);
                result.push(decode);
                // result.push(decode_u8(value));
            }
        }

        if bits_in_buffer > 0 {
            let value = ((buffer << (5 - bits_in_buffer)) & 0x1F) as u16;
            result.push(decode_u8(value));
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_basic() {
        let input = ['A', 'B', 'C', 'D', 'E'];
        let encoded = Crock32::encode(input);
        assert_eq!(encoded, vec![0x52, 0xD8, 0xD7]);
    }

    #[test]
    fn test_encode_case_insensitivity() {
        let input = ['a', 'b', 'c', 'd', 'e'];
        let encoded = Crock32::encode(input);
        assert_eq!(encoded, vec![0x52, 0xD8, 0xD7]);
    }

    #[test]
    fn test_encode_special_characters() {
        let input = ['0', 'O', '1', 'I', 'L'];
        let encoded = Crock32::encode(input);
        assert_eq!(encoded, vec![0x00, 0x02, 0x10]);
    }

    #[test]
    fn test_encode_empty_input() {
        let input: [char; 0] = [];
        let encoded = Crock32::encode(input);
        assert_eq!(encoded, vec![]);
    }

    #[test]
    fn test_encode_full_alphabet() {
        let input = [
            'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'J', 'K', 'M', 'N', 'P', 'Q', 'R', 'S', 'T',
            'V', 'W', 'X', 'Y', 'Z',
        ];
        let encoded = Crock32::encode(input);
        assert_eq!(
            encoded,
            vec![0x52, 0xD8, 0xD7, 0x3E, 0x11, 0x94, 0xE9, 0x5B, 0x5F, 0x19, 0xD6, 0xF9, 0xDF]
        );
    }

    #[test]
    fn test_decode_special_characters() {
        let input = [0x00, 0x02, 0x10];
        let decoded = Crock32::decode(input);
        assert_eq!(decoded, format!("00111"));
    }

    #[test]
    fn test_decode_empty_input() {
        let input: [u8; 0] = [];
        let decoded = Crock32::decode(input);
        assert_eq!(decoded, "");
    }

    #[test]
    fn test_decode_full_alphabet() {
        let input = [
            0x52, 0xD8, 0xD7, 0x3E, 0x11, 0x94, 0xE9, 0x5B, 0x5F, 0x19, 0xD6, 0xF9, 0xDF,
        ];
        let decoded = Crock32::decode(input);
        assert_eq!(decoded, "ABCDEFGHJKMNPQRSTVWXYZ".to_string());
    }
}
