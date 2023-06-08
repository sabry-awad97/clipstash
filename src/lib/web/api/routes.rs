use rocket::{serde::json::Json, State};

use crate::{data::AppDatabase, service::action, web::api::error::ApiError};

#[rocket::get("/key")]
pub async fn new_api_key(database: &State<AppDatabase>) -> Result<Json<&str>, ApiError> {
    let api_key = action::generate_api_key(database.get_pool()).await?;
    println!("Api Key: {}", api_key.to_base64());
    Ok(Json("Api key generated. See logs for details."))
}

pub fn routes() -> Vec<rocket::Route> {
    rocket::routes!(new_api_key)
}
