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

pub async fn update_clip<M: Into<model::UpdateClip>>(
    model: M,
    pool: &DatabasePool,
) -> Result<model::Clip> {
    let model = model.into();
    let _ = sqlx::query!(
        r#"UPDATE clips
            SET
                content = ?,
                expires = ?,
                password = ?,
                title = ?
            WHERE shortcode = ?"#,
        model.content,
        model.expires,
        model.password,
        model.title,
        model.shortcode
    )
    .execute(pool)
    .await?;
    get_clip(model.shortcode, pool).await
}

#[cfg(test)]
mod tests {
    use crate::data::Database;

    use super::*;
    use chrono::{Duration, Utc};
    use uuid::Uuid;

    #[tokio::test]
    async fn test_insert_and_get_clip() {
        let db = Database::new("sqlite::memory:").await;
        let pool = db.get_pool();
        sqlx::migrate!("./migrations").run(pool).await.unwrap();
        let posted = Utc::now();
        let expires = posted + Duration::days(7);

        let new_clip = model::NewClip {
            clip_id: Uuid::new_v4().to_string(),
            shortcode: "abcd1234".to_string(),
            content: "Hello, world!".to_string(),
            title: Some("Test Clip".to_string()),
            posted: posted.timestamp(),
            expires: Some(expires.timestamp()),
            password: Some("password".to_string()),
        };

        let inserted_clip = insert_clip(new_clip, &pool).await.unwrap();
        let retrieved_clip = get_clip(inserted_clip.shortcode, &pool).await.unwrap();

        assert_eq!(retrieved_clip.clip_id, inserted_clip.clip_id);
        assert_eq!(retrieved_clip.hits, 0);
    }
}
