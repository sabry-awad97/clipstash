use derive_more::Constructor;
use serde::Serialize;

pub trait PageContext {
    fn title(&self) -> &str;
    fn template_path(&self) -> &str;
    fn parent(&self) -> &str;
}

#[derive(Debug, Serialize)]
pub struct Home {}

impl Default for Home {
    fn default() -> Self {
        Self {}
    }
}

impl PageContext for Home {
    fn template_path(&self) -> &str {
        "home"
    }
    fn title(&self) -> &str {
        "Stash Your Clipboard!"
    }
    fn parent(&self) -> &str {
        "base"
    }
}

#[derive(Debug, Serialize, Constructor)]
pub struct ViewClip {
    pub clip: crate::Clip,
}

impl PageContext for ViewClip {
    fn template_path(&self) -> &str {
        "clip"
    }
    fn title(&self) -> &str {
        "View Clip"
    }
    fn parent(&self) -> &str {
        "base"
    }
}

#[derive(Debug, Serialize, Constructor)]
pub struct PasswordRequired {
    shortcode: crate::ShortCode,
}

impl PageContext for PasswordRequired {
    fn template_path(&self) -> &str {
        "clip_need_password"
    }
    fn title(&self) -> &str {
        "Password Required"
    }
    fn parent(&self) -> &str {
        "base"
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{data::DbId, domain::clip::field, Clip, Time};

    use super::*;

    #[test]
    fn test_home_page_context() {
        let home = Home::default();
        assert_eq!(home.template_path(), "home");
        assert_eq!(home.title(), "Stash Your Clipboard!");
        assert_eq!(home.parent(), "base");
    }

    #[test]
    fn test_view_clip_page_context() {
        let clip_id = field::ClipId::new(DbId::new());
        let shortcode = field::ShortCode::try_from("abc123").unwrap();
        let content = field::Content::new("Hello, world!").unwrap();
        let title = field::Title::new("My Clip".to_string());
        let posted = field::Posted::new(Time::from_str("1997-05-01").unwrap());
        let expires = field::Expires::new(Some(Time::from_seconds(3600)));
        let password = field::Password::new("password123".to_string()).unwrap();
        let hits = field::Hits::new(0);

        let clip = Clip {
            clip_id,
            shortcode,
            content,
            title,
            posted,
            expires,
            password,
            hits,
        };

        let view_clip = ViewClip::new(clip);
        assert_eq!(view_clip.template_path(), "clip");
        assert_eq!(view_clip.title(), "View Clip");
        assert_eq!(view_clip.parent(), "base");
    }

    #[test]
    fn test_password_required_page_context() {
        let shortcode = crate::ShortCode::from("abcd1234ef");
        let password_required = PasswordRequired::new(shortcode);
        assert_eq!(password_required.template_path(), "clip_need_password");
        assert_eq!(password_required.title(), "Password Required");
        assert_eq!(password_required.parent(), "base");
    }
}
