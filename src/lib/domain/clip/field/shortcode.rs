use std::str::FromStr;

use derive_more::From;
use rand::{seq::SliceRandom, thread_rng};
use rocket::request::FromParam;
use serde::{Deserialize, Serialize};

use super::super::ClipError;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, From)]
pub struct ShortCode(String);

impl ShortCode {
    pub fn new() -> Self {
        let allowed_chars = ['a', 'b', 'c', 'd', '1', '2', '3', '4'];

        let mut rng = thread_rng();
        let mut shortcode = String::with_capacity(10);
        for _ in 0..10 {
            shortcode.push(
                *allowed_chars
                    .choose(&mut rng)
                    .expect("sampling array should have values"),
            );
        }
        Self(shortcode)
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

impl Default for ShortCode {
    fn default() -> Self {
        Self::new()
    }
}

impl From<ShortCode> for String {
    fn from(shortcode: ShortCode) -> Self {
        shortcode.0
    }
}

impl From<&str> for ShortCode {
    fn from(shortcode: &str) -> Self {
        ShortCode(shortcode.to_owned())
    }
}

impl FromStr for ShortCode {
    type Err = ClipError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.into()))
    }
}

impl<'r> FromParam<'r> for ShortCode {
    type Error = &'r str;

    fn from_param(param: &'r str) -> Result<Self, Self::Error> {
        Ok(ShortCode::from(param))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let shortcode = ShortCode::new();
        assert_eq!(shortcode.0.len(), 10);
        assert!(shortcode
            .0
            .chars()
            .all(|c| ['a', 'b', 'c', 'd', '1', '2', '3', '4'].contains(&c)));
    }

    #[test]
    fn test_as_str() {
        let shortcode = ShortCode::from("abcd1234ef");
        assert_eq!(shortcode.as_str(), "abcd1234ef");
    }

    #[test]
    fn test_into_inner() {
        let shortcode = ShortCode::from("abcd1234ef");
        let inner = shortcode.into_inner();
        assert_eq!(inner, "abcd1234ef");
    }

    #[test]
    fn test_default() {
        let shortcode = ShortCode::default();
        assert_eq!(shortcode.0.len(), 10);
    }

    #[test]
    fn test_from() {
        let shortcode = ShortCode::new();
        let string: String = shortcode.clone().into();
        assert_eq!(string.len(), 10);
        assert!(string
            .chars()
            .all(|c| ['a', 'b', 'c', 'd', '1', '2', '3', '4'].contains(&c)));

        let shortcode_from_str = ShortCode::from(shortcode.as_str());
        assert_eq!(shortcode_from_str, shortcode);
    }

    #[test]
    fn test_from_str() {
        let shortcode = ShortCode::new();
        let shortcode_from_str = ShortCode::from_str(shortcode.as_str()).unwrap();
        assert_eq!(shortcode_from_str, shortcode);
    }

    #[test]
    fn test_from_param() {
        let shortcode = ShortCode::new();
        let shortcode_from_param = ShortCode::from_param(shortcode.as_str()).unwrap();
        assert_eq!(shortcode_from_param, shortcode);
    }
}
