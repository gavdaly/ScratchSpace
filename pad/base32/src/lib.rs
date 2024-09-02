mod crock32;
mod crock32data;

/// This is an implementation of Crockford's base32 encoding
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
        _ => panic!("Invalid character"),
    }
}

fn decode_u8(input: u16) -> char {
    match input {
        0 => '0',
        1 => '1',
        2 => '2',
        3 => '3',
        4 => '4',
        5 => '5',
        6 => '6',
        7 => '7',
        8 => '8',
        9 => '9',
        10 => 'A',
        11 => 'B',
        12 => 'C',
        13 => 'D',
        14 => 'E',
        15 => 'F',
        16 => 'G',
        17 => 'H',
        18 => 'J',
        19 => 'K',
        20 => 'M',
        21 => 'N',
        22 => 'P',
        23 => 'Q',
        24 => 'R',
        25 => 'S',
        26 => 'T',
        27 => 'V',
        28 => 'W',
        29 => 'X',
        30 => 'Y',
        31 => 'Z',
        _ => panic!("invalid input"),
    }
}
