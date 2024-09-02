use rand::random;
use std::fmt::{Display, Formatter, Result};
use std::marker::PhantomData;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub struct Day;
pub struct Hour;
pub struct Milisecond;

/// Represents a Simple Unique Identifier (SUID) with a generic type parameter.
///
/// The SUID is stored as a vector of bytes and includes a phantom data field
/// to carry the generic type information.
///
/// The generic type `T` is used to differentiate between different time-based
/// SUID implementations (e.g., Day, Hour, Millisecond).
///
/// # Type Parameters
///
/// * `T` - A type parameter that determines the specific SUID implementation.
#[derive(Debug)]
pub struct Suid<T> {
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

    use std::thread::sleep;
    use std::time::Duration;

    #[test]
    fn test_day_structure() {
        let day1 = Suid::<Day>::new();

        let timestamp = (day1.as_u32() >> 16) as u16;
        let day3 = Suid::<Day>::new_with_timestamp(timestamp);
        assert_eq!(day3.as_u32() >> 16, timestamp as u32);
    }

    #[test]
    fn test_hour_structure() {
        let hour1 = Suid::<Hour>::new();

        let timestamp = (hour1.as_u64() >> 32) as u32;
        let hour3 = Suid::<Hour>::new_with_timestamp(timestamp);
        assert_eq!(hour3.as_u64() >> 32, timestamp as u64);
    }

    #[test]
    fn test_milisecond_structure() {
        let ms1 = Suid::<Milisecond>::new();
        let timestamp = (ms1.as_u128() >> 64) as u64;
        let ms3 = Suid::<Milisecond>::new_with_timestamp(timestamp);
        assert_eq!(ms3.as_u128() >> 64, timestamp as u128);
    }

    #[test]
    fn test_milisecond_structure2() {
        let ms1 = Suid::<Milisecond>::new();
        sleep(Duration::from_millis(1));
        let ms2 = Suid::<Milisecond>::new();
        assert!(ms2.as_u128() > ms1.as_u128());
    }

    #[test]
    fn test_display() {
        let day = Suid::<Day>::new();
        assert_eq!(format!("{}", day).len(), 8);

        let hour = Suid::<Hour>::new();
        assert_eq!(format!("{}", hour).len(), 16);

        let ms = Suid::<Milisecond>::new();
        assert_eq!(format!("{}", ms).len(), 32);
    }

    #[test]
    fn test_uniqueness() {
        let mut set = std::collections::HashSet::new();
        for _ in 0..10000 {
            let id = Suid::<Milisecond>::new();
            assert!(set.insert(id.as_u128()));
        }
    }

    #[test]
    fn test_edge_cases() {
        let min_day = Suid::<Day>::new_with_timestamp(0);
        let max_day = Suid::<Day>::new_with_timestamp(u16::MAX);
        assert!(max_day.as_u32() > min_day.as_u32());

        let min_hour = Suid::<Hour>::new_with_timestamp(0);
        let max_hour = Suid::<Hour>::new_with_timestamp(u32::MAX);
        assert!(max_hour.as_u64() > min_hour.as_u64());

        let min_ms = Suid::<Milisecond>::new_with_timestamp(0);
        let max_ms = Suid::<Milisecond>::new_with_timestamp(u64::MAX);
        assert!(max_ms.as_u128() > min_ms.as_u128());
    }
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
