mod catcher;
mod error;
mod routes;

use base64::engine::{general_purpose, Engine};

pub const API_KEY_HEADER: &str = "x-api-key";

#[derive(Debug, Clone)]
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
}
