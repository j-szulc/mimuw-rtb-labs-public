use chrono::{DateTime, NaiveDateTime, ParseError, Utc};
use rocket::serde::{Serialize, Serializer, Deserialize, Deserializer};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct WrappedDateTimeUtc(DateTime<Utc>);

impl WrappedDateTimeUtc {
    pub fn parse(time_str: &str) -> Result<Self, ParseError> {
        let add_utc: fn(NaiveDateTime) -> DateTime<Utc> = |naive| naive.and_local_timezone(Utc).unwrap();
        let wrap: fn(DateTime<Utc>) -> Self = |dt| Self { 0: dt };

        let try_1: Result<Self, ParseError> =
            NaiveDateTime::parse_from_str(time_str, "%Y-%m-%dT%H:%M:%S%.3fZ")
                .map(add_utc)
                .map(wrap);

        if try_1.is_ok() {
            return try_1;
        }

        let try_2: Result<Self, ParseError> =
            NaiveDateTime::parse_from_str(time_str, "%Y-%m-%dT%H:%M:%S%.3f")
                .map(add_utc)
                .map(wrap);

        if try_2.is_ok() {
            return try_2;
        };

        let try_3: Result<Self, ParseError> =
            NaiveDateTime::parse_from_str(time_str, "%Y-%m-%dT%H:%M:%S")
                .map(add_utc)
                .map(wrap);

        return try_3;
    }
}

impl Serialize for WrappedDateTimeUtc{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_i64(self.0.timestamp())
    }
}

impl<'de> serde::Deserialize<'de> for WrappedDateTimeUtc {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let ts = i64::deserialize(d)?;
        let result = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(ts, 0), Utc);
        Ok(WrappedDateTimeUtc{ 0: result })
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct TimeRange {
    pub start: WrappedDateTimeUtc,
    pub end: WrappedDateTimeUtc,
}

impl TimeRange{
    pub fn parse(time_range_str: &str) -> Self {
        let parts = time_range_str.split("_").collect::<Vec<&str>>();
        assert!(parts.len() == 2);
        let start = WrappedDateTimeUtc::parse(parts[0]).unwrap();
        let end = WrappedDateTimeUtc::parse(parts[1]).unwrap();
        TimeRange { start, end }
    }

    pub fn contains(self: &Self, time : &WrappedDateTimeUtc) -> bool {
        &(self.start) <= time && time < &(self.end)
    }
}
