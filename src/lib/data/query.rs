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

pub async fn insert_clip<M: Into<model::NewClip>>(
    model: M,
    pool: &DatabasePool,
) -> Result<model::Clip> {
    let model = model.into();
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
        model.clip_id,
        model.shortcode,
        model.content,
        model.title,
        model.posted,
        model.expires,
        model.password,
        0
    )
    .execute(pool)
    .await?;
    get_clip(model.shortcode, pool).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::model::Clip;
    use chrono::{Duration, NaiveDateTime, Utc};
    use sqlx::{Pool, Sqlite, SqlitePool};
    use uuid::Uuid;

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

        insert_clip_helper(&clip, &pool).await.unwrap();

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

    async fn insert_clip_helper(clip: &Clip, pool: &Pool<Sqlite>) -> sqlx::Result<()> {
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

    #[tokio::test]
    async fn test_insert_and_get_clip() {
        let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
        sqlx::migrate!("./migrations").run(&pool).await.unwrap();

        let clip_id = Uuid::new_v4().to_string();
        let shortcode = "abcd1234".to_string();
        let content = "Hello, world!".to_string();
        let title = Some("Test Clip".to_string());
        let posted = Utc::now();
        let expires = posted + Duration::days(7);
        let password = Some("password".to_string());

        let new_clip = model::NewClip {
            clip_id,
            shortcode: shortcode.clone(),
            content,
            title,
            posted: posted.timestamp(),
            expires: Some(expires.timestamp()),
            password,
        };

        let inserted_clip = insert_clip(new_clip, &pool).await.unwrap();
        
        let retrieved_clip = get_clip(shortcode, &pool).await.unwrap();
        
        assert_eq!(retrieved_clip.clip_id, inserted_clip.clip_id);
        assert_eq!(retrieved_clip.hits, 0);
    }
}
