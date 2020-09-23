use super::BellButton;
use crate::db_entry::UserEntry;
pub use rocket_contrib::databases::rusqlite::{Connection, Error};
use std::sync::{Arc, Mutex};

pub struct EventHandler {
    sync_flag: Arc<Mutex<bool>>,
    conn: Connection,
    buttons: Vec<BellButton>,
}

impl EventHandler {
    pub fn new(sf: Arc<Mutex<bool>>, conn: Connection) -> Self {
        let mut eh = EventHandler {
            sync_flag: sf,
            conn: conn,
            buttons: Vec::new(),
        };
        eh.fetch_user();
        eh
    }

    pub fn event_loop(&mut self) {
        loop {
            let mut should_sync = false;
            if let Ok(flag) = self.sync_flag.lock() {
                should_sync = *flag;
            }
            if should_sync {
                self.fetch_user();

                if let Ok(mut flag) = self.sync_flag.lock() {
                    *flag = false;
                }
            }
            for button in self.buttons.iter() {
                button.check_events();
            }
        }
    }

    fn fetch_user(&mut self) -> Result<(), Error> {
        let users = UserEntry::get_all_with_rusq(&self.conn)?;
        self.buttons = users.iter().map(|user| BellButton::new(&user)).collect();
        Ok(())
    }
}
