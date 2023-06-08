use crate::data::{query, DatabasePool, Transaction};
use crate::service::ask;
use crate::web::api::ApiKey;
use crate::{Clip, ServiceError, ShortCode};
use std::convert::TryInto;

/// Begins a new database transaction using the provided database pool.
///
/// # Arguments
///
/// * `pool` - A reference to the `DatabasePool` providing the database connection.
///
/// # Returns
///
/// Returns a `Result` containing a `Transaction` if the transaction is successfully started. Otherwise, returns a `ServiceError`.
///
pub async fn begin_transaction(pool: &DatabasePool) -> Result<Transaction<'_>, ServiceError> {
    Ok(pool.begin().await?)
}

/// Ends the provided database transaction by committing the changes.
///
/// # Arguments
///
/// * `transaction` - The `Transaction` to be ended and committed.
///
/// # Returns
///
/// Returns `Ok(())` if the transaction is successfully ended and committed. Otherwise, returns a `ServiceError`.
///
pub async fn end_transaction(transaction: Transaction<'_>) -> Result<(), ServiceError> {
    Ok(transaction.commit().await?)
}

/// Increases the hit count for a given clip shortcode by the specified number of hits.
///
/// # Arguments
///
/// * `shortcode` - A reference to the `ShortCode` representing the clip to increase the hit count for.
/// * `hits` - The number of hits to increment the hit count by.
/// * `pool` - A reference to the `DatabasePool` providing the database connection.
///
/// # Returns
///
/// Returns `Ok(())` if the hit count is successfully increased. Otherwise, returns a `ServiceError`.
///
pub async fn increase_hit_count(
    shortcode: &ShortCode,
    hits: u32,
    pool: &DatabasePool,
) -> Result<(), ServiceError> {
    Ok(query::increase_hit_count(shortcode, hits, pool).await?)
}

/// Creates a new clip based on the provided request and inserts it into the database.
///
/// # Arguments
///
/// * `req` - The request object containing necessary information to create the clip.
/// * `pool` - A reference to the database connection pool.
///
/// # Returns
///
/// A `Result` indicating either the newly created `Clip` or a `ServiceError` if an error occurs.
///
pub async fn new_clip(req: ask::NewClip, pool: &DatabasePool) -> Result<Clip, ServiceError> {
    Ok(query::insert_clip(req, pool).await?.try_into()?)
}

/// Updates an existing clip based on the provided request and updates it in the database.
///
/// # Arguments
///
/// * `req` - The request object containing necessary information to update the clip.
/// * `pool` - A reference to the database connection pool.
///
/// # Returns
///
/// A `Result` indicating either the updated `Clip` or a `ServiceError` if an error occurs.
///
pub async fn update_clip(req: ask::UpdateClip, pool: &DatabasePool) -> Result<Clip, ServiceError> {
    Ok(query::update_clip(req, pool).await?.try_into()?)
}

/// Retrieves a clip based on the provided request and database connection pool.
///
/// # Arguments
///
/// * `req` - The request object containing necessary information to retrieve the clip.
/// * `pool` - A reference to the database connection pool.
///
/// # Returns
///
/// A `Result` indicating either the retrieved `Clip` or a `ServiceError` if an error occurs.
///
pub async fn get_clip(req: ask::GetClip, pool: &DatabasePool) -> Result<Clip, ServiceError> {
    let user_password = req.password.clone();
    let clip: Clip = query::get_clip(req, pool).await?.try_into()?;
    if clip.password.has_password() {
        if clip.password == user_password {
            Ok(clip)
        } else {
            Err(ServiceError::PermissionError("Invalid password".to_owned()))
        }
    } else {
        Ok(clip)
    }
}

/// Generates a new API key and saves it in the database, returning the generated key.
///
/// # Arguments
///
/// * `pool` - A reference to a `DatabasePool` object representing the database connection pool.
///
/// # Returns
///
/// Returns a `Result` containing the generated `ApiKey` if the key generation and saving process is successful,
/// or a `ServiceError` if an error occurs during the process.
pub async fn generate_api_key(pool: &DatabasePool) -> Result<ApiKey, ServiceError> {
    let api_key = ApiKey::default();
    Ok(query::save_api_key(api_key, pool).await?)
}

/// Revokes an API key, returning the revocation status.
///
/// # Arguments
///
/// * `api_key` - An instance of the `ApiKey` struct representing the API key to be revoked.
/// * `pool` - A reference to a `DatabasePool` object representing the database connection pool.
///
/// # Returns
///
/// Returns a `Result` containing the `RevocationStatus` if the API key revocation is successful,
/// or a `ServiceError` if an error occurs during the revocation process.
pub async fn revoke_api_key(
    api_key: ApiKey,
    pool: &DatabasePool,
) -> Result<query::RevocationStatus, ServiceError> {
    Ok(query::revoke_api_key(api_key, pool).await?)
}

/// Checks if an API key is valid in the database.
///
/// # Arguments
///
/// * `api_key` - An `ApiKey` representing the API key to check.
/// * `pool` - A reference to a `DatabasePool` representing the connection pool to the database.
///
/// # Returns
///
/// Returns a `Result<bool, ServiceError>` indicating success or an error if the check operation fails.
/// If successful, it returns a boolean value indicating whether the API key is valid or not.
///
pub async fn api_key_is_valid(api_key: ApiKey, pool: &DatabasePool) -> Result<bool, ServiceError> {
    Ok(query::api_key_is_valid(api_key, pool).await?)
}

/// Deletes expired records from the database and returns the number of deleted records.
///
/// # Arguments
///
/// * `pool` - A reference to a `DatabasePool` object representing the database connection pool.
///
/// # Returns
///
/// Returns a `Result` containing the number of deleted records if the deletion process is successful,
/// or a `ServiceError` if an error occurs during the deletion process.
pub async fn delete_expired(pool: &DatabasePool) -> Result<u64, ServiceError> {
    Ok(query::delete_expired(pool).await?)
}
