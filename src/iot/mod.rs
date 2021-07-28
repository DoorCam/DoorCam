//! All logic which is not web-based.

mod door_control;
pub use door_control::DoorControl;

mod bell_button;
use bell_button::BellButton;

mod event_handler;
pub use event_handler::event_loop;

#[cfg(feature = "iot")]
use rppal::gpio::Gpio;
#[cfg(feature = "iot")]
use std::time::Duration;

#[cfg(feature = "iot")]
lazy_static! {
    static ref GPIO: Gpio = Gpio::new().expect("Couldn't connect to the GPIO-chip!");
    // TODO Make magic constant configurable
    static ref MINIMAL_DEBOUNCED_ACTION_INTERVAL: Duration = Duration::from_millis(42);
}

#[cfg(feature = "iot")]
#[macro_export]
macro_rules! debounce_callback {
    ($last_action:ident) => {
        if $last_action.elapsed() < *MINIMAL_DEBOUNCED_ACTION_INTERVAL {
            $last_action = Instant::now();
            return;
        }
        $last_action = Instant::now();
    };
    ($last_action:ident, $debounce_interval:ident) => {
        if $last_action.elapsed() < $debounce_interval {
            $last_action = Instant::now();
            return;
        }
        $last_action = Instant::now();
    };
}
