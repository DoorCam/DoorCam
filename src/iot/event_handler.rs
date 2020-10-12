///Syncs the flats between web and IoT and manages the BellButtons.
use super::BellButton;
use crate::db_entry::FlatEntry;
use log::{error, info};
use rocket_contrib::databases::rusqlite::Connection;
use rsevents::{AutoResetEvent, Awaitable};
use std::sync::Arc;
use std::thread;

/// Create multi-threaded event-loops in order to sync with the web application and creates and drops BellButtons, which have their own event-loop
pub fn event_loop(flat_sync_event: &Arc<AutoResetEvent>, conn: Connection) {
    let flat_sync_event = Arc::clone(&flat_sync_event);
    thread::spawn(move || {
        let mut buttons: Vec<BellButton> = Vec::new();
        info!("IoT: Loading flats in event_loop");
        loop {
            fetch_flats(&mut buttons, &conn);
            flat_sync_event.wait();
            info!("IoT: Reloading flats in event_loop");
        }
    });
}

/// Fetches all active flats from the database if a flat has been changed
fn fetch_flats(buttons: &mut Vec<BellButton>, conn: &Connection) {
    let flats = match FlatEntry::get_active(&conn) {
        Ok(f) => f,
        Err(e) => {
            error!("IoT: Can't fetch flats in event_loop: {}", e);
            return;
        }
    };
    *buttons = flats.iter().map(BellButton::new).collect();
}
