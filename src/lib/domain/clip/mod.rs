pub mod field;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum ClipError {
    #[error("invalid password: {0}")]
    InvalidPassword(String),

    #[error("invalid title: {0}")]
    InvalidTitle(String),

    #[error("empty content")]
    EmptyContent,

    #[error("invalid date: {0}")]
    InvalidDate(String),

    #[error("date parse error: {0}")]
    DateParse(#[from] chrono::ParseError),

    #[error("id parse error: {0}")]
    Id(#[from] uuid::Error),

    #[error("hits parse error: {0}")]
    Hits(#[from] std::num::TryFromIntError),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Clip {
    #[serde(skip)]
    pub clip_id: field::ClipId,
    pub shortcode: field::ShortCode,
    pub content: field::Content,
    pub title: field::Title,
    pub posted: field::Posted,
    pub expires: field::Expires,
    pub password: field::Password,
    pub hits: field::Hits,
}

#[cfg(test)]
mod tests {
    use crate::{data::DbId, Time};

    use super::*;
    use std::{convert::TryFrom, str::FromStr};

    #[test]
    fn test_clip() {
        let db_id: DbId = DbId::new();
        let clip_id = field::ClipId::new(db_id.clone());
        let shortcode = field::ShortCode::try_from("abc123").unwrap();
        let content = field::Content::new("Hello, world!").unwrap();
        let title = field::Title::new("My Clip".to_string());
        let time_str = "1997-05-01";
        let time = Time::from_str(time_str).unwrap();
        let posted = field::Posted::new(time.clone());
        let expires = field::Expires::new(Some(Time::from_seconds(3600)));
        let password = field::Password::new("password123".to_string()).unwrap();
        let hits = field::Hits::new(0);

        let clip = Clip {
            clip_id: clip_id.clone(),
            shortcode: shortcode.clone(),
            content: content.clone(),
            title: title.clone(),
            posted: posted.clone(),
            expires: expires.clone(),
            password: password.clone(),
            hits: hits.clone(),
        };

        assert_eq!(clip.clip_id, clip_id);
        assert_eq!(clip.shortcode, shortcode);
        assert_eq!(clip.content, content);
        assert_eq!(clip.title, title);
        assert_eq!(clip.posted, posted);
        assert_eq!(clip.expires, expires);
        assert_eq!(clip.password, password);
        assert_eq!(clip.hits, hits);
    }
}
