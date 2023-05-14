use crate::domain::clip::ClipError;
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

#[cfg(test)]
mod tests {
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
        let password = Password::default();
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
}
