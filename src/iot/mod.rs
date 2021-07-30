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
lazy_static! {
    static ref GPIO: Gpio = Gpio::new().expect("Couldn't connect to the GPIO-chip!");
}

#[cfg(feature = "iot")]
#[macro_export]
macro_rules! setup_debounce {
    ($debounce_interval:expr) => {
        Instant::now()
            .checked_sub($debounce_interval)
            .expect("IoT: Invalid debounce interval")
    };
}

#[cfg(feature = "iot")]
#[macro_export]
macro_rules! debounce_callback {
    ($last_action:ident, $debounce_interval:expr) => {
        if $last_action.elapsed() < $debounce_interval {
            $last_action = Instant::now();
            return;
        }
        $last_action = Instant::now();
    };
}
