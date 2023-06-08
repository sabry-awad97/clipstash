use rocket::{http::CookieJar, serde::json::Json, State};

use crate::{
    data::AppDatabase,
    service::{self, action},
    web::{
        api::{error::ApiError, ApiKey},
        HitCounter, PASSWORD_COOKIE,
    },
};

#[rocket::get("/<shortcode>")]
pub async fn get_clip(
    shortcode: &str,
    database: &State<AppDatabase>,
    cookies: &CookieJar<'_>,
    hit_counter: &State<HitCounter>,
    _api_key: ApiKey,
) -> Result<Json<crate::Clip>, ApiError> {
    use crate::domain::clip::field::Password;

    let req = service::ask::GetClip {
        shortcode: shortcode.into(),
        password: cookies
            .get(PASSWORD_COOKIE)
            .map(|cookie| cookie.value())
            .map(|raw_password| Password::new(raw_password.to_string()).ok())
            .flatten()
            .unwrap_or_else(Password::default),
    };
    let clip = action::get_clip(req, database.get_pool()).await?;
    hit_counter.hit(shortcode.into(), 1);
    Ok(Json(clip))
}

#[rocket::get("/key")]
pub async fn new_api_key(database: &State<AppDatabase>) -> Result<Json<&str>, ApiError> {
    let api_key = action::generate_api_key(database.get_pool()).await?;
    println!("Api Key: {}", api_key.to_base64());
    Ok(Json("Api key generated. See logs for details."))
}

pub fn routes() -> Vec<rocket::Route> {
    rocket::routes!(get_clip, new_api_key)
}
