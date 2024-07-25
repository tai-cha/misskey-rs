// AIDX
// 長さ8の[2000年1月1日からの経過ミリ秒をbase36でエンコードしたもの] + 長さ4の[個体ID] + 長さ4の[カウンタ]

use std::fmt::{self, Display};
use std::str::FromStr;

use chrono::{DateTime, TimeZone, Utc};
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Aidx {
    pub timestamp: i64,
    pub node_id: [u8; NODE_LENGTH],
    pub counter: u16,
}

impl Aidx {
    pub fn datetime(&self) -> DateTime<Utc> {
        // NOTE: this does not panic when parsed from valid Aid since the following does not panic
        // `Utc.timestamp_millis_opt(36_i64.pow(8) - 1).unwrap()`
        Utc.timestamp_millis_opt(self.timestamp).unwrap()
    }

    pub fn node_id_to_string(&self) -> String {
        self.node_id.iter().map(|&x| x as char).collect()
    }
}

// https://github.com/misskey-dev/misskey/blob/2023.9.0/packages/backend/src/misc/id/aidx.ts#L15
const TIME2000: i64 = 946684800000;
const TIME_LENGTH : usize = 8;
const NODE_LENGTH : usize = 4;
const NOISE_LENGTH : usize = 4;

#[derive(Debug, Error, Clone)]
#[error("invalid aidx")]
pub struct ParseAidxError {
    _priv: (),
}

impl FromStr for Aidx {
    type Err = ParseAidxError;

    fn from_str(s: &str) -> Result<Aidx, Self::Err> {
        let (timestamp_str, rest) = s.split_at(TIME_LENGTH);
        let (node_id_str, counter_str) = rest.split_at(NODE_LENGTH);


        let timestamp = match i64::from_str_radix(timestamp_str, 36) {
            Ok(x) => x + TIME2000,
            Err(_) => return Err(ParseAidxError { _priv: () }),
        };

        let node_id: [u8; NODE_LENGTH] = node_id_str.as_bytes().try_into().map_err(|_| ParseAidxError { _priv: () })?;

        let counter = match u16::from_str_radix(counter_str, 36) {
            Ok(x) => x,
            Err(_) => return Err(ParseAidxError { _priv: () }),
        };

        Ok(Aidx {
            timestamp,
            node_id,
            counter
        })
    }
}

struct Radix36(u64);

impl Radix36 {
    fn new(x: impl Into<u64>) -> Radix36 {
        Radix36(x.into())
    }
}

impl Display for Radix36 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use std::fmt::Write;

        let width = f
            .width()
            .unwrap_or_else(|| (self.0 as f64).log(36.0).floor() as usize + 1);

        (0..width)
            .rev()
            .map(|i| self.0 / 36_u64.pow(i.try_into().unwrap()) % 36)
            .map(|d| std::char::from_digit(d.try_into().unwrap(), 36).unwrap())
            .try_for_each(|c| f.write_char(c))
    }
}

impl Display for Aidx {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let since_2000 = u64::try_from(self.timestamp - TIME2000).unwrap_or(0);
        let timestamp_fmt = Radix36::new(since_2000);
        write!(f, "{:08}{}{:04}", timestamp_fmt, self.node_id_to_string(), self.counter)
    }
}

impl Serialize for Aidx {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(self)
    }
}

impl<'de> Deserialize<'de> for Aidx {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::Aidx;
    use chrono::{DateTime, Duration, TimeZone, Utc};
    use rand::{self, Rng};

    fn new() -> Aidx {
        from_datetime(Utc::now())
    }

    fn from_datetime<Tz>(datetime: DateTime<Tz>) -> Aidx
    where
        Tz: TimeZone,
    {
        from_datetime_with_source(datetime, &mut rand::thread_rng())
    }

    fn generate_random_string(length: usize) -> [u8; super::NODE_LENGTH] {
        let charset: &[u8] = b"0123456789abcdefghijklmnopqrstuvwxyz";
        let mut rng = rand::thread_rng();
        let mut result = [0u8; super::NODE_LENGTH];
        for i in 0..length {
            result[i] = charset[rng.gen_range(0..charset.len())];
        }
        result
    }

    fn from_datetime_with_source<Tz, R>(datetime: DateTime<Tz>, source: &mut R) -> Aidx
    where
        Tz: TimeZone,
        R: Rng,
    {
        let timestamp = datetime.timestamp_millis();
        let node_id = generate_random_string(super::NODE_LENGTH);
        let counter = source.gen::<u16>() % 10000;
        Aidx { timestamp, node_id, counter }
    }

    #[test]
    fn test_deserialize_const() {
        let string = "8dhemt9uabcd0001";
        let aidx: Aidx = string.parse().expect("failed to parse");
        assert_eq!(aidx.datetime(), Utc.timestamp_millis_opt(1602948787122).unwrap());
    }

    #[test]
    fn test_serialize_deserialize() {
        let aidx1 = new();
        let string = aidx1.to_string();
        let aidx2: Aidx = string.parse().expect("failed to parse");
        assert_eq!(aidx1, aidx2);
    }

    #[test]
    fn test_deserialize_serialize() {
        let string1 = "8dhe5zqiabcd5273";
        let aidx: Aidx = string1.parse().expect("failed to parse");
        let string2 = aidx.to_string();
        assert_eq!(string1, string2);
    }

    #[test]
    fn test_deserialize_serialize2() {
        let string1 = "8ejiidh50mgd8271";
        let aid: Aidx = string1.parse().expect("failed to parse");
        let string2 = aid.to_string();
        assert_eq!(string1, string2);
    }

    #[test]
    fn test_order() {
        let time = Utc::now();
        let aid1 = from_datetime(time);
        let aid2 = from_datetime(time + Duration::milliseconds(1));
        assert!(aid1 < aid2);
    }
}
