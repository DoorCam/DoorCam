use crate::db_conn::DbConn;
use crate::guards::{AdminGuard, UserGuard};
use crate::template_contexts::{CamContext, ErrorContext, NoContext, UserOverviewContext};
use crate::user_entry::UserEntry;
use rocket::request::Form;
use rocket_contrib::templates::Template;

#[derive(FromForm)]
pub struct UserForm {
    name: String,
    pw: String,
    pw_repeat: String,
    admin: bool,
}

#[get("/admin/user/create")]
pub fn get_create(_admin: AdminGuard) -> Template {
    let context = NoContext {};
    Template::render("user_create", &context)
}

#[post("/admin/user/create", data = "<user_data>")]
pub fn post_create_data(user_data: Form<UserForm>, _admin: AdminGuard, conn: DbConn) -> Template {
    if user_data.name.is_empty() {
        let context = ErrorContext {
            error: "Name is empty".to_string(),
        };
        return Template::render("error", &context);
    }
    if user_data.pw.is_empty() {
        let context = ErrorContext {
            error: "Password is empty".to_string(),
        };
        return Template::render("error", &context);
    }
    if user_data.pw != user_data.pw_repeat {
        let context = ErrorContext {
            error: "Passwords are not the same".to_string(),
        };
        return Template::render("error", &context);
    }
    match UserEntry::create(conn, &user_data.name, &user_data.pw, user_data.admin) {
        Err(e) => {
            let context = ErrorContext {
                error: format!("DB Error: {}", e),
            };
            return Template::render("error", &context);
        }
        _ => {}
    }

    let context = CamContext {
        cam_url: "http://doorcam.fritz.box:8081/".to_string(),
    };
    Template::render("cam", &context)
}

#[get("/admin/user")]
pub fn get_users(_admin: AdminGuard, conn: DbConn) -> Template {
    let users = match UserEntry::get_all(conn) {
        Ok(users) => users,
        Err(e) => {
            let context = ErrorContext {
                error: format!("DB Error: {}", e),
            };
            return Template::render("error", &context);
        }
    };
    let context = UserOverviewContext { users: users };
    Template::render("user_overview", &context)
}
