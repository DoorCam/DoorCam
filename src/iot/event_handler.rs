use super::BellButton;
use crate::db_entry::FlatEntry;
use log::{error, info};
use rocket_contrib::databases::rusqlite::Connection;
use rsevents::{AutoResetEvent, Awaitable};
use std::sync::Arc;
use std::thread;

pub fn event_loop(sync: &Arc<AutoResetEvent>, conn: Connection) {
    let sync = Arc::clone(&sync);
    thread::spawn(move || {
        let mut buttons: Vec<BellButton> = Vec::new();
        info!("Loading flats in event_loop");
        loop {
            fetch_flats(&mut buttons, &conn);
            sync.wait();
            info!("Reloading flats in event_loop");
        }
    });
}

fn fetch_flats(buttons: &mut Vec<BellButton>, conn: &Connection) {
    let flats = match FlatEntry::get_active(&conn) {
        Ok(f) => f,
        Err(e) => {
            error!("Can't fetch flats in event_loop: {}", e);
            return;
        }
    };
    *buttons = flats.iter().map(|flat| BellButton::new(&flat)).collect();
}
