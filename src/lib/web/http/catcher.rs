use rocket::Request;
use rocket::{catch, catchers, Catcher};

/// Catch unhandled errors.
#[catch(default)]
fn default(req: &Request) -> &'static str {
    eprintln!("General error: {:?}", req);
    "something went wrong..."
}

/// Catch server errors.
#[catch(500)]
fn internal_error(req: &Request) -> &'static str {
    eprintln!("Internal error: {:?}", req);
    "internal server error"
}

/// Catch missing data errors.
#[catch(404)]
fn not_found() -> &'static str {
    "404"
}

/// The [`catchers`](rocket::Catcher) which can be registered by [`rocket`].
pub fn catchers() -> Vec<Catcher> {
    catchers![not_found, default, internal_error]
}
