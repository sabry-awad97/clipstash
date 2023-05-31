use crate::domain::clip::ClipError;
use rocket::form;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Password(Option<String>);

impl Password {
    pub fn new<T: Into<Option<String>>>(password: T) -> Result<Self, ClipError> {
        let password: Option<String> = password.into();
        match password {
            Some(password) => {
                if !password.trim().is_empty() {
                    Ok(Self(Some(password)))
                } else {
                    Ok(Self(None))
                }
            }
            None => Ok(Self(None)),
        }
    }

    pub fn into_inner(self) -> Option<String> {
        self.0
    }

    pub fn has_password(&self) -> bool {
        self.0.is_some()
    }
}

impl Default for Password {
    fn default() -> Self {
        Self(None)
    }
}

impl std::str::FromStr for Password {
    type Err = ClipError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s.to_string())
    }
}

#[rocket::async_trait]
impl<'r> form::FromFormField<'r> for Password {
    fn from_value(field: form::ValueField<'r>) -> form::Result<'r, Self> {
        Ok(Self::new(field.value.to_owned())
            .map_err(|e| form::Error::validation(format!("{}", e)))?)
    }
}

#[cfg(test)]
mod tests {
    use rocket::form::FromFormField;

    use super::*;

    #[test]
    fn test_new_password() {
        // Test with valid password
        let password = Password::new("password123".to_string()).unwrap();
        assert_eq!(password.has_password(), true);
        assert_eq!(password.into_inner().unwrap(), "password123".to_string());

        // Test with empty password
        let password = Password::new("".to_string()).unwrap();
        assert_eq!(password.has_password(), false);
        assert_eq!(password.into_inner(), None);

        // Test with None
        let password = Password::new(None).unwrap();
        assert_eq!(password.has_password(), false);
        assert_eq!(password.into_inner(), None);
    }

    #[test]
    fn test_default_password() {
        let password = <Password as std::default::Default>::default();
        assert_eq!(password.has_password(), false);
        assert_eq!(password.into_inner(), None);
    }

    #[test]
    fn test_from_str_password() {
        // Test with valid password
        let password: Password = "password123".parse().unwrap();
        assert_eq!(password.has_password(), true);
        assert_eq!(password.into_inner().unwrap(), "password123");

        // Test with empty password
        let password: Password = "".parse().unwrap();
        assert_eq!(password.has_password(), false);
        assert_eq!(password.into_inner(), None);
    }

    #[test]
    fn test_from_value() {
        let field = form::ValueField::parse("password=123");
        let result = Password::from_value(field);
        let expected = Password::new("123".to_string()).unwrap();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
    }
}
