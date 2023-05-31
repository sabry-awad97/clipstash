use crate::domain::clip::ClipError;
use rocket::form;
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

#[rocket::async_trait]
impl<'r> form::FromFormField<'r> for Content {
    fn from_value(field: form::ValueField<'r>) -> form::Result<'r, Self> {
        Ok(Self::new(field.value).map_err(|e| form::Error::validation(format!("{}", e)))?)
    }
}

#[cfg(test)]
mod tests {
    use rocket::form::{self, FromFormField};

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

    #[test]
    fn test_from_value() {
        let field = form::ValueField::parse("content=Hello, world!");
        let expected = Content::new("Hello, world!").unwrap();
        let result = Content::from_value(field);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
    }
}
