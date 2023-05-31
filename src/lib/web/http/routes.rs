use crate::web::{ctx, renderer::Renderer};
use rocket::{get, response::content::RawHtml, State};

#[get("/")]
fn home(renderer: &State<Renderer<'_>>) -> RawHtml<String> {
    let context = ctx::Home::default();
    RawHtml(renderer.render(context, &[]))
}

pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![home]
}
