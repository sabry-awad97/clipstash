use super::model;

use crate::data::{DataError, DatabasePool};

type Result<T> = std::result::Result<T, DataError>;

pub async fn get_clip<M: Into<model::GetClip>>(
    model: M,
    pool: &DatabasePool,
) -> Result<model::Clip> {
    let model = model.into();
    let shortcode = model.shortcode.as_str();
    Ok(sqlx::query_as!(
        model::Clip,
        "SELECT * FROM clips WHERE shortcode = ?",
        shortcode
    )
    .fetch_one(pool)
    .await?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::model::Clip;
    use chrono::NaiveDateTime;
    use sqlx::{Pool, Sqlite};

    const CLIP_ID: &str = "01234567-89ab-cdef-0123-456789abcdef";

    async fn initialize() -> Pool<Sqlite> {
        let pool = Pool::connect("sqlite::memory:").await.unwrap();
        sqlx::migrate!("./migrations").run(&pool).await.unwrap();
        pool
    }

    #[tokio::test]
    async fn test_get_clip() {
        let pool = initialize().await;
        let clip = Clip {
            clip_id: CLIP_ID.to_string(),
            shortcode: "abc123".to_string(),
            content: "Hello, world!".to_string(),
            title: Some("Test Clip".to_string()),
            posted: NaiveDateTime::from_timestamp_opt(862070800, 0).unwrap(),
            expires: NaiveDateTime::from_timestamp_opt(862060800, 0),
            password: Some("password".to_string()),
            hits: 10,
        };

        insert_clip(&clip, &pool).await.unwrap();

        // Test getting the clip by shortcode
        let result = get_clip(
            model::GetClip {
                shortcode: clip.shortcode.clone(),
            },
            &pool,
        )
        .await
        .unwrap();
        assert_eq!(result.clip_id, clip.clip_id);
    }

    async fn insert_clip(clip: &Clip, pool: &Pool<Sqlite>) -> sqlx::Result<()> {
        sqlx::query!(
            r#"INSERT INTO
                clips (
                    clip_id,
                    shortcode,
                    content,
                    title,
                    posted,
                    expires,
                    password,
                    hits
                )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)"#,
            clip.clip_id,
            clip.shortcode,
            clip.content,
            clip.title,
            clip.posted,
            clip.expires,
            clip.password,
            clip.hits
        )
        .execute(pool)
        .await
        .map(|_| ())
    }
}
