#[cfg(feature = "iot")]
use super::GPIO;
use crate::utils::config::CONFIG;
use log::{error, info};
#[cfg(feature = "iot")]
use rppal::gpio::OutputPin;
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};
use std::thread;

#[cfg(test)]
#[path = "./door_control_test.rs"]
mod door_control_test;

///Used to activate the door-opener.
pub struct DoorControl {
    #[cfg(not(feature = "iot"))]
    is_open: Arc<Mutex<bool>>,

    #[cfg(feature = "iot")]
    dev: Arc<Mutex<OutputPin>>,
}

#[cfg(not(feature = "iot"))]
impl DoorControl {
    #[allow(clippy::mutex_atomic)]
    pub fn new(_pin: u8) -> Result<Self, ()> {
        Ok(Self {
            is_open: Arc::new(Mutex::new(false)),
        })
    }

    /// Activates the opener for the `door_opening_time`
    pub fn activate_opener(&mut self) -> Result<(), PoisonError<MutexGuard<bool>>> {
        let mut state = self.is_open.lock()?;
        // Stop if the opener is active
        if *state {
            info!("IoT: Opener already active");
            return Ok(());
        }

        info!("IoT: Activating opener");
        *state = true;

        // Spawn thread which waits the `door_opening_time` and stops the opener
        let is_open = Arc::clone(&self.is_open);
        thread::spawn(move || {
            thread::sleep(CONFIG.iot.door_opening_time);
            match is_open.lock() {
                Ok(mut state) => *state = false,
                Err(e) => error!("IoT: Can't deactivate opener: {}", e),
            }
        });
        Ok(())
    }

    #[cfg(test)]
    pub fn is_opener_active(&mut self) -> Result<bool, PoisonError<MutexGuard<bool>>> {
        let state = self.is_open.lock()?;
        Ok(*state)
    }
}

#[cfg(feature = "iot")]
impl DoorControl {
    pub fn new(pin: u8) -> Result<Self, rppal::gpio::Error> {
        Ok(Self {
            dev: Arc::new(Mutex::new(GPIO.get(pin)?.into_output())),
        })
    }

    /// Activates the opener for the `door_opening_time`
    pub fn activate_opener(&mut self) -> Result<(), PoisonError<MutexGuard<OutputPin>>> {
        let mut dev = self.dev.lock()?;
        // Stop if the opener is active
        if dev.is_set_high() {
            info!("IoT: Opener already active");
            return Ok(());
        }

        info!("IoT: Activating opener");
        dev.set_high();

        let dev = Arc::clone(&self.dev);

        // Spawn thread which waits the `door_opening_time` and stops the opener
        thread::spawn(move || {
            thread::sleep(CONFIG.iot.door_opening_time);
            match dev.lock() {
                Ok(mut dev) => dev.set_low(),
                Err(e) => error!("IoT: Can't deactivate opener: {}", e),
            }
        });
        Ok(())
    }

    #[cfg(test)]
    pub fn is_opener_active(&self) -> Result<bool, PoisonError<MutexGuard<OutputPin>>> {
        Ok(self.dev.lock()?.is_set_high())
    }
}
