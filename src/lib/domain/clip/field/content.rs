use crate::domain::clip::ClipError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Content(String);

impl Content {
    pub fn new(content: &str) -> Result<Self, ClipError> {
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
        let content = Content::new("Hello, world!").unwrap();
        assert_eq!(content.as_str(), "Hello, world!");
    }

    #[test]
    fn test_empty_content() {
        let result = Content::new("");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ClipError::EmptyContent);
    }

    #[test]
    fn test_into_inner() {
        let content = Content::new("Hello, world!").unwrap();
        let inner = content.into_inner();
        assert_eq!(inner, "Hello, world!");
    }
}
