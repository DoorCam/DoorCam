use crate::db_entry::UserEntry;
#[cfg(feature = "iot")]
use rust_gpiozero::input_devices::Button;

pub struct BellButton {
    #[cfg(feature = "iot")]
    dev: Button,
    broker_adress: String,
    topic: String,
}

#[cfg(not(feature = "iot"))]
impl BellButton {
    pub fn new(_user: &UserEntry) -> Self {
        BellButton {
            broker_adress: String::new(),
            topic: String::new(),
        }
    }

    pub fn events(&self) {}

    pub fn send_bell_signal(&self) {}
}

#[cfg(feature = "iot")]
impl BellButton {
    pub fn new(_user: &UserEntry) -> Self {
        BellButton {
            dev: Button::new(0),
            broker_adress: String::new(),
            topic: String::new(),
        }
    }

    pub fn events(&self) {
        if self.dev.is_active() {
            self.send_bell_signal();
        }
    }

    pub fn send_bell_signal(&self) {}
}
