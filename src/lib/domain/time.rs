use chrono::{DateTime, NaiveDateTime, Utc};
use derive_more::From;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Clone, Debug, From, PartialEq, Serialize, Deserialize)]
pub struct Time(DateTime<Utc>);

impl Time {
    pub fn into_inner(self) -> DateTime<Utc> {
        self.0
    }

    pub fn timestamp(&self) -> i64 {
        self.0.timestamp()
    }

    pub fn from_naive_utc(datetime: NaiveDateTime) -> Self {
        Time(DateTime::from_utc(datetime, Utc))
    }
}

impl FromStr for Time {
    type Err = chrono::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match format!("{}T00:00:00Z", s).parse::<DateTime<Utc>>() {
            Ok(time) => Ok(time.into()),
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{NaiveDate, NaiveTime, TimeZone};

    #[test]
    fn test_into_inner() {
        let datetime = Utc::now();
        let time = Time(datetime);
        assert_eq!(time.into_inner(), datetime);
    }

    #[test]
    fn test_timestamp() {
        let datetime = Utc::now();
        let time = Time(datetime);
        assert_eq!(time.timestamp(), datetime.timestamp());
    }

    #[test]
    fn test_from_naive_utc() {
        let d = NaiveDate::from_ymd_opt(1997, 5, 1).unwrap();
        let t = NaiveTime::from_hms_milli_opt(0, 0, 0, 0).unwrap();

        let naive_datetime = NaiveDateTime::new(d, t);
        let datetime = Utc.from_utc_datetime(&naive_datetime);
        let time = Time::from_naive_utc(naive_datetime);
        assert_eq!(time.into_inner(), datetime);
    }

    #[test]
    fn test_from_str() {
        let time_str = "1997-05-01";
        let dt: NaiveDateTime = NaiveDate::from_ymd_opt(1997, 5, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();
        let expected_time = Time::from_naive_utc(dt);
        assert_eq!(time_str.parse::<Time>().unwrap(), expected_time);
    }
}
