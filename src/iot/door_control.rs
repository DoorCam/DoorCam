use log::{error, info};
#[cfg(feature = "iot")]
use rust_gpiozero::output_devices::OutputDevice;
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};
use std::thread;
use std::time::Duration;

#[cfg(test)]
#[path = "./door_control_test.rs"]
mod door_control_test;

const OPENING_TIME_PERIOD: Duration = Duration::from_secs(10);

pub struct DoorControl {
    #[cfg(not(feature = "iot"))]
    is_open: Arc<Mutex<bool>>,
    #[cfg(feature = "iot")]
    dev: Arc<Mutex<OutputDevice>>,
}

#[cfg(not(feature = "iot"))]
impl DoorControl {
    pub fn new(_pin: u8) -> DoorControl {
        DoorControl {
            is_open: Arc::new(Mutex::new(false)),
        }
    }

    pub fn activate_opener(&mut self) -> Result<(), PoisonError<MutexGuard<bool>>> {
        let mut state = self.is_open.lock()?;
        if *state {
            return Ok(());
        }
        info!("Activating opener");
        *state = true;
        let is_open = Arc::clone(&self.is_open);
        thread::spawn(move || {
            thread::sleep(OPENING_TIME_PERIOD);
            match is_open.lock() {
                Ok(mut state) => *state = false,
                Err(e) => error!("Can't deactivate opener: {}", e),
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
    pub fn new(pin: u8) -> DoorControl {
        return DoorControl {
            dev: Arc::new(Mutex::new(OutputDevice::new(pin))),
        };
    }

    pub fn activate_opener(&mut self) -> Result<(), PoisonError<MutexGuard<OutputDevice>>> {
        let mut dev = self.dev.lock()?;
        if dev.is_active() {
            return Ok(());
        }
        info!("Activating opener");
        dev.on();
        let dev = Arc::clone(&self.dev);
        thread::spawn(move || {
            thread::sleep(OPENING_TIME_PERIOD);
            match dev.lock() {
                Ok(mut dev) => dev.off(),
                Err(e) => error!("Can't deactivate opener: {}", e),
            }
        });
        Ok(())
    }

    #[cfg(test)]
    pub fn is_opener_active(&mut self) -> Result<bool, PoisonError<MutexGuard<OutputDevice>>> {
        let dev = self.dev.lock()?;
        return Ok(dev.is_active());
    }
}
