use serde::{Deserialize, Serialize};

use crate::domain::clip::field;
use crate::ShortCode;

#[derive(Debug, Deserialize, Serialize)]
pub struct NewClip {
    pub content: field::Content,
    pub title: field::Title,
    pub expires: field::Expires,
    pub password: field::Password,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetClip {
    pub shortcode: ShortCode,
    pub password: field::Password,
}

impl GetClip {
    pub fn from_raw(shortcode: &str) -> Self {
        Self {
            shortcode: ShortCode::from(shortcode),
            password: field::Password::default(),
        }
    }
}

impl From<ShortCode> for GetClip {
    fn from(shortcode: ShortCode) -> Self {
        Self {
            shortcode,
            password: field::Password::default(),
        }
    }
}

impl From<&str> for GetClip {
    fn from(raw: &str) -> Self {
        Self::from_raw(raw)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_raw() {
        let shortcode = "abc123";
        let get_clip = GetClip::from_raw(shortcode);
        assert_eq!(get_clip.shortcode, ShortCode::from(shortcode));
        assert_eq!(get_clip.password, field::Password::default());
    }

    #[test]
    fn test_from_shortcode() {
        let shortcode = ShortCode::from("abc123");
        let get_clip = GetClip::from(shortcode.clone());
        assert_eq!(get_clip.shortcode, shortcode);
        assert_eq!(get_clip.password, field::Password::default());
    }

    #[test]
    fn test_from_str() {
        let shortcode = "abc123";
        let get_clip = GetClip::from(shortcode);
        assert_eq!(get_clip.shortcode, ShortCode::from(shortcode));
        assert_eq!(get_clip.password, field::Password::default());
    }
}
