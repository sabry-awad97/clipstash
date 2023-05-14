use crate::domain::time::Time;
use derive_more::Constructor;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Constructor, PartialEq)]
pub struct Posted(Time);

impl Posted {
    pub fn into_inner(self) -> Time {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use crate::domain::time::Time;

    #[test]
    fn test_into_inner() {
        let time_str = "1997-05-01";
        let time = Time::from_str(time_str).unwrap();
        let posted = Posted::new(time.clone());
        assert_eq!(posted.into_inner(), time);
    }
}
