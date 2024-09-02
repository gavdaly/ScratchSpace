pub struct Crock32;

/// This is an implementation of Crockford's base32 encoding
impl Crock32 {
    /// Encode a string of characters into a base32 encoded string
    pub fn encode<const N: usize>(input: [char; N]) -> Vec<u8> {
        let mut buffer: u32 = 0;
        let mut bits_in_buffer: u8 = 0;
        let mut result = Vec::new();

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

    pub fn decode<const N: usize>(_input: [u8; N]) -> String {
        unimplemented!()
    }
}

fn encode_char(input: &char) -> u8 {
    match input {
        '0' | 'O' | 'o' => 0,
        '1' | 'I' | 'i' | 'L' | 'l' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'a' | 'A' => 10,
        'b' | 'B' => 11,
        'c' | 'C' => 12,
        'd' | 'D' => 13,
        'e' | 'E' => 14,
        'f' | 'F' => 15,
        'g' | 'G' => 16,
        'h' | 'H' => 17,
        'j' | 'J' => 18,
        'k' | 'K' => 19,
        'm' | 'M' => 20,
        'n' | 'N' => 21,
        'p' | 'P' => 22,
        'q' | 'Q' => 23,
        'r' | 'R' => 24,
        's' | 'S' => 25,
        't' | 'T' => 26,
        'v' | 'V' => 27,
        'w' | 'W' => 28,
        'x' | 'X' => 29,
        'y' | 'Y' => 30,
        'z' | 'Z' => 31,
        _ => 0,
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
}
