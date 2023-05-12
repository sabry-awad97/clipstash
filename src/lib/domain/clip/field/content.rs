use crate::domain::clip::ClipError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Content(String);

impl Content {
    pub fn from_str(content: &str) -> Result<Self, ClipError> {
        if !content.trim().is_empty() {
            Ok(Self(content.to_owned()))
        } else {
            Err(ClipError::EmptyContent)
        }
    }
    pub fn into_inner(self) -> String {
        self.0
    }
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_content() {
        let content = Content::from_str("Hello, world!").unwrap();
        assert_eq!(content.as_str(), "Hello, world!");
    }

    #[test]
    fn test_into_inner() {
        let content = Content::from_str("Hello, world!").unwrap();
        let inner = content.into_inner();
        assert_eq!(inner, "Hello, world!");
    }
}
