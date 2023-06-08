use rocket::serde::json::Json;
use rocket::Request;
use rocket::{catch, catchers, Catcher};

#[catch(default)]
fn default(req: &Request) -> Json<&'static str> {
    eprintln!("General error: {:?}", req);
    Json("something went wrong...")
}

#[catch(500)]
fn internal_error(req: &Request) -> Json<&'static str> {
    eprintln!("Internal error: {:?}", req);
    Json("internal server error")
}

#[catch(404)]
fn not_found() -> Json<&'static str> {
    Json("404")
}

#[catch(401)]
fn request_error() -> Json<&'static str> {
    Json("request error")
}

#[catch(400)]
fn missing_api_key() -> Json<&'static str> {
    Json("API key missing or invalid")
}

pub fn catchers() -> Vec<Catcher> {
    catchers![
        not_found,
        default,
        internal_error,
        missing_api_key,
        request_error
    ]
}
