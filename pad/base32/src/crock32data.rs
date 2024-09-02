use crate::encode_char;

pub struct Crock32(Vec<u8>);

impl Into<String> for Crock32 {
    fn into(self) -> String {
        unimplemented!()
    }
}

impl From<String> for Crock32 {
    fn from(val: String) -> Self {
        let mut buffer: u32 = 0;
        let mut bits_in_buffer: i8 = 0;
        let mut data = Vec::with_capacity(val.len());

        for char in val.chars() {
            let value = encode_char(&char) as u32;
            buffer = (buffer << 5) | value;
            bits_in_buffer += 5;

            while bits_in_buffer >= 8 {
                bits_in_buffer -= 8;
                let byte = (buffer >> bits_in_buffer) as u8;
                data.push(byte);
                buffer &= (1 << bits_in_buffer) - 1;
            }
        }

        Self(data)
    }
}

impl Into<Vec<u8>> for Crock32 {
    fn into(self) -> Vec<u8> {
        self.0
    }
}
