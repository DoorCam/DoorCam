use super::*;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct DbConn(Arc<Mutex<Connection>>);

impl DbConn {
    pub fn fairing() -> impl rocket::fairing::Fairing {
        let sql_scheme = include_str!("../../scheme.sql");
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(sql_scheme).unwrap();
        let conn = Self(Arc::new(Mutex::new(conn)));

        rocket::fairing::AdHoc::on_attach("'DbConn' in memory Database", |rocket| {
            Ok(rocket.manage(conn))
        })
    }
    pub fn get_one(rocket: &rocket::Rocket) -> Option<Self> {
        rocket.state::<Self>().cloned()
    }
}

impl std::ops::Deref for DbConn {
    type Target = Connection;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute::<&Connection, &'static Connection>(&*self.0.lock().unwrap()) }
    }
}

impl<'a, 'r> rocket::request::FromRequest<'a, 'r> for DbConn {
    type Error = ();

    fn from_request(
        request: &'a rocket::request::Request<'r>,
    ) -> rocket::request::Outcome<Self, ()> {
        request
            .guard::<::rocket::State<Self>>()
            .map(|conn| conn.clone())
    }
}
