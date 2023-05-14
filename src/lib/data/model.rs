use std::convert::TryFrom;

use chrono::{NaiveDateTime, Utc};

use crate::data::DbId;
use crate::{ClipError, ShortCode, Time};

#[derive(Debug, sqlx::FromRow)]
pub struct Clip {
    pub(in crate::data) clip_id: String,
    pub(in crate::data) shortcode: String,
    pub(in crate::data) content: String,
    pub(in crate::data) title: Option<String>,
    pub(in crate::data) posted: NaiveDateTime,
    pub(in crate::data) expires: Option<NaiveDateTime>,
    pub(in crate::data) password: Option<String>,
    pub(in crate::data) hits: i64,
}

impl TryFrom<Clip> for crate::domain::Clip {
    type Error = ClipError;

    fn try_from(clip: Clip) -> Result<Self, Self::Error> {
        use crate::domain::clip::field;
        use std::str::FromStr;

        Ok(Self {
            clip_id: field::ClipId::new(DbId::from_str(clip.clip_id.as_str())?),
            shortcode: field::ShortCode::from(clip.shortcode),
            content: field::Content::new(clip.content.as_str())?,
            title: field::Title::new(clip.title),
            posted: field::Posted::new(Time::from_naive_utc(clip.posted)),
            expires: field::Expires::new(clip.expires.map(Time::from_naive_utc)),
            password: field::Password::new(clip.password.unwrap_or_default())?,
            hits: field::Hits::new(u64::try_from(clip.hits)?),
        })
    }
}

pub struct GetClip {
    pub(in crate::data) shortcode: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::clip::field::*;
    use std::str::FromStr;

    #[test]
    fn test_try_from_clip() {
        let id_str = "01234567-89ab-cdef-0123-456789abcdef";
        let clip = Clip {
            clip_id: id_str.to_string(),
            shortcode: "abc123".to_string(),
            content: "Hello, world!".to_string(),
            title: Some("Test Clip".to_string()),
            posted: NaiveDateTime::from_timestamp_opt(862070800, 0).unwrap(),
            expires: NaiveDateTime::from_timestamp_opt(862060800, 0),
            password: Some("password".to_string()),
            hits: 10,
        };

        let result = crate::domain::Clip::try_from(clip).unwrap();

        assert_eq!(result.clip_id, ClipId::new(DbId::from_str(id_str).unwrap()));
        assert_eq!(result.shortcode, ShortCode::from("abc123"));
        assert_eq!(result.content, Content::new("Hello, world!").unwrap());
        assert_eq!(result.title, Title::new(Some("Test Clip".to_string())));
        assert_eq!(
            result.posted,
            Posted::new(Time::from_naive_utc(
                NaiveDateTime::from_timestamp_opt(862070800, 0).unwrap()
            ))
        );
        assert_eq!(
            result.expires,
            Expires::new(Time::from_naive_utc(
                NaiveDateTime::from_timestamp_opt(862060800, 0).unwrap()
            ))
        );
        assert_eq!(
            result.password,
            Password::new("password".to_string()).unwrap()
        );
        assert_eq!(result.hits, Hits::new(10));
    }
}
