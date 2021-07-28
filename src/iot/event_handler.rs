//! Syncs the flats between web and IoT and manages the BellButtons.

use super::BellButton;
#[cfg(feature = "iot")]
use super::{GPIO, MINIMAL_DEBOUNCED_ACTION_INTERVAL};
use crate::db_entry::FlatEntry;
#[cfg(feature = "iot")]
use crate::debounce_callback;
use crate::utils::no_operation;
#[cfg(feature = "iot")]
use crate::CONFIG;
use log::{error, info};
use rocket_contrib::databases::rusqlite::Connection;
#[cfg(feature = "iot")]
use rppal::gpio::{InputPin, Trigger};
use rsevents::{AutoResetEvent, Awaitable};
use std::sync::{Arc, Mutex};
use std::thread;
#[cfg(feature = "iot")]
use std::time::Instant;

/// Create multi-threaded event-loops in order to sync with the web application and creates and drops BellButtons, which have their own event-loop
pub fn event_loop(flat_sync_event: &Arc<AutoResetEvent>, conn: Connection) {
    let flat_sync_event = Arc::clone(flat_sync_event);
    let buttons: Arc<Mutex<Vec<BellButton>>> = Arc::new(Mutex::new(Vec::new()));

    let tamper_sensor_pin = setup_tamper_sensor(Arc::clone(&buttons));

    thread::spawn(move || {
        info!("IoT: Loading flats in event_loop");
        loop {
            // Move the tamper_sensor_pin into this thread to live the complete runtime
            no_operation(&tamper_sensor_pin);

            match buttons.lock() {
                Ok(mut buttons) => fetch_flats(&mut buttons, &conn),
                Err(e) => error!("IoT: Can't lock bell buttons: {}", e),
            }
            flat_sync_event.wait();
            info!("IoT: Reloading flats in event_loop");
        }
    });
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
            error!("IoT: Can't create bell button: {}", e);
            return;
        }
    }
}

#[cfg(not(feature = "iot"))]
fn setup_tamper_sensor(_connections: Arc<Mutex<Vec<BellButton>>>) {}
#[cfg(feature = "iot")]
fn setup_tamper_sensor(connections: Arc<Mutex<Vec<BellButton>>>) -> Option<InputPin> {
    let tamper_sensor_pin = CONFIG.iot.tamper_sensor_pin?;
    let mut dev = GPIO
        .get(tamper_sensor_pin)
        .expect("Can't use the tamper sensor pin")
        .into_input_pulldown();

    let mut last_action = Instant::now()
        .checked_sub(*MINIMAL_DEBOUNCED_ACTION_INTERVAL)
        .unwrap();

    dev.set_async_interrupt(Trigger::RisingEdge, move |_level| {
        debounce_callback!(last_action);

        info!("IoT: tamper sensor recieved alarm");

        match connections.lock() {
            Ok(mut connections) => connections
                .iter_mut()
                .for_each(BellButton::send_tamper_alarm),
            Err(e) => error!("IoT: Can't lock connections: {}", e),
        }
        info!("IoT: tamper sensor sent alarm(s)");
    })
    .expect("IoT: Couldn't arm the tamper sensor!");
    Some(dev)
}
