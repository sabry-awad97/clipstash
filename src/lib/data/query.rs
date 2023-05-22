use super::model;

use crate::data::{DataError, DatabasePool};

type Result<T> = std::result::Result<T, DataError>;

/// Retrieves a clip from the database based on the provided model and database connection pool.
///
/// # Arguments
///
/// * `model` - The model representing the clip to retrieve.
/// * `pool` - The database connection pool.
///
/// # Returns
///
/// A `Result` containing the retrieved clip on success, or an error on failure.
///
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

/// Inserts a new clip into the database based on the provided model and database connection pool.
///
/// # Arguments
///
/// * `model` - The model representing the new clip to insert.
/// * `pool` - The database connection pool.
///
/// # Returns
///
/// A `Result` containing the inserted clip on success, or an error on failure.
///
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

/// Updates an existing clip in the database based on the provided model and database connection pool.
///
/// # Arguments
///
/// * `model` - The model representing the clip to update.
/// * `pool` - The database connection pool.
///
/// # Returns
///
/// A `Result` containing the updated clip on success, or an error on failure.
///
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
    async fn test_insert_update_and_get_clip() {
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

        let updated_clip = model::UpdateClip {
            shortcode: "abcd1234".to_string(),
            content: "Updated content".to_string(),
            title: Some("Updated title".to_string()),
            expires: Some((Utc::now() + Duration::days(2)).timestamp()),
            password: None,
        };

        let updated_clip = update_clip(updated_clip, &pool).await.unwrap();
        assert_eq!(updated_clip.content, "Updated content");
        assert_eq!(updated_clip.title, Some("Updated title".to_string()));
        assert_eq!(updated_clip.password, None);
        assert_eq!(updated_clip.hits, 0);
    }
}
