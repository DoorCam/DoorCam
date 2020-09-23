use super::BellButton;
use crate::db_entry::UserEntry;
use log::{error, info};
use rocket_contrib::databases::rusqlite::{Connection, Error};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

const SLEEP_PERIOD: Duration = Duration::from_millis(50);

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
        if let Err(e) = eh.fetch_user() {
            error!("Can't fetch users in EventLoop: {}", e)
        }
        eh
    }

    pub fn event_loop(&mut self) {
        loop {
            // Sync users
            let mut should_sync = false;
            match self.sync_flag.lock() {
                Ok(flag) => should_sync = *flag,
                Err(e) => error!("Can't lock sync_flag: {}", e),
            }
            if should_sync {
                info!("Reloading users in EventLoop");
                if let Err(e) = self.fetch_user() {
                    error!("Can't reload users in EventLoop: {}", e)
                }

                match self.sync_flag.lock() {
                    Ok(mut flag) => *flag = false,
                    Err(e) => error!("Can't lock sync_flag after sync: {}", e),
                }
            }

            // check and handle BellButton events
            for button in self.buttons.iter() {
                button.events();
            }
            thread::sleep(SLEEP_PERIOD);
        }
    }

    fn fetch_user(&mut self) -> Result<(), Error> {
        let users = UserEntry::get_all_with_rusq(&self.conn)?;
        self.buttons = users.iter().map(|user| BellButton::new(&user)).collect();
        Ok(())
    }
}
