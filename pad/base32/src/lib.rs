#![feature(array_chunks)]
#![feature(iter_array_chunks)]

use bit_vec::BitVec;

fn decode(input: &[u8]) -> Vec<char> {
    let bits = BitVec::from_bytes(input);
    bits.iter()
        .array_chunks::<5>()
        .fold(vec![], |mut acc, chunks| {
            acc.push(decode_5(chunks));
            acc
        })
}

///
// fn dec<const N: usize, const R: usize>(input: &[u8; N]) -> [char; R] {
//     assert!(R != N * 8 / 5 + 1, "array must be a respective size");
//     assert!(N != 0, "chunk size must be non-zero");
//     let return_array_length = N * 8 / 5 + 1;
//     let mut ret = ['0'; R];
//     let ba = BitVec::new();

//     ba.iter().array_chunks::<5>();
//     //get a const bit array into chunks of size 5;
//     todo!()
// };

// fn encode_const<const N: usize>(input: &[char; N]) -> Vec<u8> {
//     let b = BitVec::from_bytes(input);
//     assert!(N != 0, "chunk size must be non-zero");
//     assert!(N % 2 != 0, "chunk size must be an even length");
//     let mask: u8 = 0b11111000;
//     input.array_chunks::<2>().map(
//         |[first, second]|
//         (encode_nibble(first) << 4) + encode_nibble(second)).collect::<Vec<_>>()
//     }

// fn encode(input: &[char]) -> Vec<u8> {
//     let mask: u8 = 0b11111000;
//     input.array_chunks::<2>().map(
//         |[first, second]|
//         (encode_nibble(first) << 4) + encode_nibble(second)).collect::<Vec<_>>()
//     }

// fn decode<const N: usize>(input: &[u8; N]) -> Vec<char> {
//     let b = BitVec::from_bytes(input);

//     input.iter().fold(Vec::with_capacity(N * 2),  |mut acc, bit| {
//         acc.push(decode_nibble(bit % 128));
//         acc.push(decode_nibble(bit >> 4));
//         acc
//     })
// }

fn encode(input: &char) -> u8 {
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

fn decode_5(input: [bool; 5]) -> char {
    match input {
        [false, false, false, false, false] => '0',
        [false, false, false, false, true] => '1',
        [false, false, false, true, false] => '2',
        [false, false, false, true, true] => '3',
        [false, false, true, false, false] => '4',
        [false, false, true, false, true] => '5',
        [false, false, true, true, false] => '6',
        [false, false, true, true, true] => '7',
        [false, true, false, false, false] => '8',
        [false, true, false, false, true] => '9',
        [false, true, false, true, false] => 'A',
        [false, true, false, true, true] => 'B',
        [false, true, true, false, false] => 'C',
        [false, true, true, false, true] => 'D',
        [false, true, true, true, false] => 'E',
        [false, true, true, true, true] => 'F',
        [true, false, false, false, false] => 'G',
        [true, false, false, false, true] => 'H',
        [true, false, false, true, false] => 'J',
        [true, false, false, true, true] => 'K',
        [true, false, true, false, false] => 'M',
        [true, false, true, false, true] => 'N',
        [true, false, true, true, false] => 'P',
        [true, false, true, true, true] => 'Q',
        [true, true, false, false, false] => 'R',
        [true, true, false, false, true] => 'S',
        [true, true, false, true, false] => 'T',
        [true, true, false, true, true] => 'V',
        [true, true, true, false, false] => 'W',
        [true, true, true, false, true] => 'X',
        [true, true, true, true, false] => 'Y',
        [true, true, true, true, true] => 'Z',
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {}

    #[test]
    fn test_decode() {}
}
