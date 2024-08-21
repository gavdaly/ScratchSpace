use rand::random;
use std::fmt::{Display, Formatter, Result};
use std::marker::PhantomData;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

struct Day;
struct Hour;
struct Milisecond;

#[derive(Debug)]
struct Suid<T> {
    value: Vec<u8>,
    data: std::marker::PhantomData<T>,
}

impl<T> Suid<T> {
    pub fn len(&self) -> usize {
        self.value.len()
    }
}

static DAY: u64 = 60 * 60 * 24;
static HOUR: u64 = 60 * 60;

/// A simple unique identifier generator for 32-bit unsigned integers.
/// Creates a time-based unique identifier from
///
/// |   16 bits |       16 bits |
/// |-----------|---------------|
/// | timestamp | random number |
impl Suid<Day> {
    pub fn new() -> Suid<Day> {
        let value = (((now().as_secs() / DAY) as u32) << 16 | random::<u16>() as u32)
            .to_be_bytes()
            .to_vec();
        Suid {
            data: PhantomData,
            value,
        }
    }

    pub fn new_with_timestamp(timestamp: u16) -> Self {
        let value = ((timestamp as u32) << 16 | random::<u16>() as u32)
            .to_be_bytes()
            .to_vec();
        Suid {
            data: PhantomData,
            value,
        }
    }

    pub fn as_u32(&self) -> u32 {
        self.value.iter().fold(0, |acc, &x| acc << 8 | x as u32)
    }
}

impl Display for Suid<Day> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:08x}", self.as_u32())
    }
}

/// A simple unique identifier generator for 64-bit unsigned integers.
/// Creates a time-based unique identifier from
///
/// |   32 bits |       32 bits |
/// |-----------|---------------|
/// | timestamp | random number |
impl Suid<Hour> {
    pub fn new() -> Suid<Hour> {
        let value = (((now().as_secs() / HOUR) as u64) << 32 | random::<u32>() as u64)
            .to_be_bytes()
            .to_vec();
        Suid {
            data: PhantomData,
            value,
        }
    }

    pub fn new_with_timestamp(timestamp: u32) -> Self {
        let value = ((timestamp as u64) << 32 | random::<u32>() as u64)
            .to_be_bytes()
            .to_vec();
        Suid {
            data: PhantomData,
            value,
        }
    }

    pub fn as_u64(&self) -> u64 {
        self.value.iter().fold(0, |acc, &x| acc << 8 | x as u64)
    }
}

impl Display for Suid<Hour> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:016x}", self.as_u64())
    }
}

/// A simple unique identifier generator for 128-bit unsigned integers.
/// Creates a time-based unique identifier from
///
/// |   64 bits |       64 bits |
/// |-----------|---------------|
/// | timestamp | random number |
impl Suid<Milisecond> {
    pub fn new() -> Suid<Milisecond> {
        let value = ((now().as_millis() as u128) << 64 | random::<u64>() as u128)
            .to_be_bytes()
            .to_vec();
        Suid {
            data: PhantomData,
            value,
        }
    }
    pub fn new_with_timestamp(timestamp: u64) -> Self {
        let value = ((timestamp as u128) << 64 | random::<u64>() as u128)
            .to_be_bytes()
            .to_vec();
        Suid {
            data: PhantomData,
            value,
        }
    }
    pub fn as_u128(&self) -> u128 {
        self.value.iter().fold(0, |acc, &x| acc << 8 | x as u128)
    }
}

impl Display for Suid<Milisecond> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:032x}", self.as_u128())
    }
}

#[inline]
fn now() -> Duration {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Duration since epoch should always succeed")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_day() {
        let day = Suid::<Day>::new();
        assert_eq!(day.len(), 4);
    }

    #[test]
    fn test_new_hour() {
        let hour = Suid::<Hour>::new();
        assert_eq!(hour.len(), 8);
    }

    #[test]
    fn test_new_milisecond() {
        let milisecond = Suid::<Milisecond>::new();
        assert_eq!(milisecond.len(), 16);
    }
}
