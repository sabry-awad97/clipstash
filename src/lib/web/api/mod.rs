mod catcher;
mod error;
mod routes;

use std::str::FromStr;

use base64::engine::{general_purpose, Engine};

use self::error::ApiKeyError;

pub const API_KEY_HEADER: &str = "x-api-key";

#[derive(Debug, Clone, PartialEq)]
pub struct ApiKey(Vec<u8>);

impl ApiKey {
    pub fn to_base64(&self) -> String {
        general_purpose::STANDARD.encode(self.0.as_slice())
    }

    pub fn into_inner(self) -> Vec<u8> {
        self.0
    }
}

impl Default for ApiKey {
    fn default() -> Self {
        let key = (0..16).map(|_| rand::random::<u8>()).collect();
        Self(key)
    }
}

impl FromStr for ApiKey {
    type Err = ApiKeyError;
    fn from_str(key: &str) -> Result<Self, Self::Err> {
        general_purpose::STANDARD
            .decode(key)
            .map(ApiKey)
            .map_err(|e| Self::Err::DecodeError(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_base64() {
        let api_key = ApiKey(vec![1, 2, 3]);
        let expected_base64 = "AQID";
        assert_eq!(api_key.to_base64(), expected_base64);
    }

    #[test]
    fn test_into_inner() {
        let api_key = ApiKey(vec![1, 2, 3]);
        let expected_inner = vec![1, 2, 3];
        assert_eq!(api_key.into_inner(), expected_inner);
    }

    #[test]
    fn test_default() {
        let key = ApiKey::default();
        assert_eq!(key.0.len(), 16);
    }

    #[test]
    fn test_from_str() {
        let key_str = "AQIDBAUGBwgJCgsMDQ4PEA==";
        let expected_key = ApiKey(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
        assert_eq!(ApiKey::from_str(key_str).unwrap(), expected_key);
    }
}
