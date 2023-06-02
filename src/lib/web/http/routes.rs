use crate::{
    data::AppDatabase,
    service::action,
    web::{ctx, renderer::Renderer, PageError},
    ServiceError, ShortCode,
};
use rocket::{
    get,
    http::Status,
    response::{content::RawHtml, status},
    State,
};

#[get("/")]
fn home(renderer: &State<Renderer<'_>>) -> RawHtml<String> {
    let context = ctx::Home::default();
    RawHtml(renderer.render(context, &[]))
}

#[get("/clip/<shortcode>")]
pub async fn get_clip(
    shortcode: ShortCode,
    database: &State<AppDatabase>,
    renderer: &State<Renderer<'_>>,
) -> Result<status::Custom<RawHtml<String>>, PageError> {
    match action::get_clip(shortcode.clone().into(), database.get_pool()).await {
        Ok(clip) => {
            let context = ctx::ViewClip::new(clip);
            Ok(status::Custom(
                Status::Ok,
                RawHtml(renderer.render(context, &[])),
            ))
        }
        Err(e) => match e {
            ServiceError::PermissionError(_) => {
                let context = ctx::PasswordRequired::new(shortcode);
                Ok(status::Custom(
                    Status::Unauthorized,
                    RawHtml(renderer.render(context, &[])),
                ))
            }
            ServiceError::NotFound => Err(PageError::NotFound("Clip not found".to_owned())),
            _ => Err(PageError::Internal("Server Error".to_owned())),
        },
    }
}

pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![home, get_clip]
}
