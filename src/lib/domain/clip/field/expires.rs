use crate::domain::{clip::ClipError, time::Time};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Expires(Option<Time>);

impl Expires {
    pub fn new<T: Into<Option<Time>>>(expires: T) -> Self {
        Self(expires.into())
    }
    pub fn into_inner(self) -> Option<Time> {
        self.0
    }
}

impl Default for Expires {
    fn default() -> Self {
        Self::new(None)
    }
}

impl FromStr for Expires {
    type Err = ClipError;
    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        if raw.is_empty() {
            Ok(Self(None))
        } else {
            match Time::from_str(raw) {
                Ok(time) => Ok(Self::new(time)),
                Err(e) => Err(e.into()),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let expires = Expires::new(Some(Time::from_seconds(3600)));
        assert_eq!(expires.into_inner(), Some(Time::from_seconds(3600)));

        let expires = Expires::new(None);
        assert_eq!(expires.into_inner(), None);
    }

    #[test]
    fn test_default() {
        let expires = Expires::default();
        assert_eq!(expires.into_inner(), None);
    }

    #[test]
    fn test_from_str() {
        let expires = Expires::from_str("");
        assert_eq!(expires.unwrap().into_inner(), None);
    }
}
