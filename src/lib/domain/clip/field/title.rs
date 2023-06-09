use crate::domain::clip::ClipError;
use rocket::form;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Title(Option<String>);

impl Title {
    pub fn new<T: Into<Option<String>>>(title: T) -> Self {
        match title.into() {
            Some(title) => {
                if !title.trim().is_empty() {
                    Self(Some(title))
                } else {
                    Self(None)
                }
            }
            None => Self(None),
        }
    }

    pub fn into_inner(self) -> Option<String> {
        self.0
    }
}

impl Default for Title {
    fn default() -> Self {
        Self::new(None)
    }
}

impl FromStr for Title {
    type Err = ClipError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s.to_string()))
    }
}

#[rocket::async_trait]
impl<'r> form::FromFormField<'r> for Title {
    fn from_value(field: form::ValueField<'r>) -> form::Result<'r, Self> {
        Ok(Self::new(field.value.to_owned()))
    }
}

#[cfg(test)]
mod tests {

    use rocket::form::FromFormField;

    use super::*;

    #[test]
    fn test_new_title_with_valid_string() {
        let title = Title::new("Valid Title".to_string());
        assert_eq!(title.into_inner(), Some("Valid Title".to_string()));
    }

    #[test]
    fn test_new_title_with_empty_string() {
        let title = Title::new("".to_string());
        assert_eq!(title.into_inner(), None);
    }

    #[test]
    fn test_new_title_with_none() {
        let title = Title::new(None);
        assert_eq!(title.into_inner(), None);
    }

    #[test]
    fn test_default_title() {
        let title = <Title as std::default::Default>::default();
        assert_eq!(title.into_inner(), None);
    }

    #[test]
    fn test_from_str_with_valid_string() {
        let title = Title::from_str("Valid Title").unwrap();
        assert_eq!(title.into_inner(), Some("Valid Title".to_string()));
    }

    #[test]
    fn test_from_str_with_empty_string() {
        let title = Title::from_str("").unwrap();
        assert_eq!(title.into_inner(), None);
    }

    #[test]
    fn test_from_str_with_none() {
        let title = Title::from_str("None").unwrap();
        assert_eq!(title.into_inner(), Some("None".to_string()));
    }

    #[test]
    fn test_from_value() {
        let field = form::ValueField::parse("title=Title");
        let result = Title::from_value(field);
        let expected = Title::from_str("Title").unwrap();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
    }
}
