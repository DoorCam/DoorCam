use crate::template_contexts::ErrorContext;
use rocket_contrib::templates::Template;

#[get("/error/<msg>")]
pub fn error(msg: String) -> Template {
    let context = ErrorContext { error: msg };
    Template::render("error", &context)
}
