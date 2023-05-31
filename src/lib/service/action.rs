use crate::data::{query, DatabasePool};
use crate::service::ask;
use crate::{Clip, ServiceError};
use std::convert::TryInto;

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
