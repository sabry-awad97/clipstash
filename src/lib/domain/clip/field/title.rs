use crate::domain::clip::ClipError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
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

impl std::str::FromStr for Title {
    type Err = ClipError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s.to_string()))
    }
}
