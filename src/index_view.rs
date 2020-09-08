use crate::guards::UserGuard;
use crate::template_contexts::CamContext;
use rocket_contrib::templates::Template;

#[get("/")]
pub fn index(_user: UserGuard) -> Template {
    let context = CamContext {
        cam_url: "http://doorcam.fritz.box:8081/".to_string(),
    };
    Template::render("cam", &context)
}
