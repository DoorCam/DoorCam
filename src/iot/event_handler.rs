use super::BellButton;
use crate::db_entry::FlatEntry;
use log::{error, info};
use rocket_contrib::databases::rusqlite::Connection;
use rsevents::{AutoResetEvent, Awaitable};
///Syncs the flats between web and IoT and manages the BellButtons.
#[cfg(feature = "iot")]
use rust_gpiozero::input_devices::DigitalInputDevice;
use std::sync::{Arc, Mutex};
use std::thread;

/// Create multi-threaded event-loops in order to sync with the web application and creates and drops BellButtons, which have their own event-loop
pub fn event_loop(flat_sync_event: &Arc<AutoResetEvent>, conn: Connection) {
    let flat_sync_event = Arc::clone(flat_sync_event);
    let buttons: Arc<Mutex<Vec<BellButton>>> = Arc::new(Mutex::new(Vec::new()));
    let tamper_loop_buttons = Arc::clone(&buttons);
    thread::spawn(move || {
        info!("IoT: Loading flats in event_loop");
        loop {
            fetch_flats(&mut buttons.lock().unwrap(), &conn);
            flat_sync_event.wait();
            info!("IoT: Reloading flats in event_loop");
        }
    });
    tamper_sensor_loop(tamper_loop_buttons);
}

/// Fetches all active flats from the database if a flat has been changed
fn fetch_flats(buttons: &mut Vec<BellButton>, conn: &Connection) {
    let flats = match FlatEntry::get_active(conn) {
        Ok(f) => f,
        Err(e) => {
            error!("IoT: Can't fetch flats in event_loop: {}", e);
            return;
        }
    };
    *buttons = match flats.into_iter().map(BellButton::new).collect() {
        Ok(buttons) => buttons,
        Err(e) => {
            error!("Error decrypting broker password: {}", e);
            return;
        }
    }
}

#[cfg(not(feature = "iot"))]
fn tamper_sensor_loop(_connections: Arc<Mutex<Vec<BellButton>>>) {}
#[cfg(feature = "iot")]
fn tamper_sensor_loop(connections: Arc<Mutex<Vec<BellButton>>>) {
    let tamper_sensor_pin = match CONFIG.iot.tamper_sensor_pin {
        Some(pin) => pin,
        None => return,
    };
    let mut dev = DigitalInputDevice::new(tamper_sensor_pin);
    thread::spawn(move || loop {
        dev.wait_for_inactive(None);
        info!("IoT: tamper sensor sent alarm");

        // Stops the thread if the drop-flag is set
        match connections.lock() {
            Ok(connections) => connections
                .mut_iter()
                .for_each(BellButton::send_tamper_alarm),
            Err(e) => error!("IoT: Can't lock connections: {}", e),
        }
    });
}
