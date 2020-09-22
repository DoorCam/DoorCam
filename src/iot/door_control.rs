#[cfg(feature = "iot")]
use rust_gpiozero::output_devices::OutputDevice;
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};
use std::thread;
use std::time::Duration;

pub struct DoorControl {
    #[cfg(feature = "only_web")]
    is_open: Arc<Mutex<bool>>,
    #[cfg(feature = "iot")]
    dev: Arc<Mutex<OutputDevice>>,
}

#[cfg(feature = "only_web")]
impl DoorControl {
    pub fn new(_pin: u8) -> DoorControl {
        return DoorControl {
            is_open: Arc::new(Mutex::new(false)),
        };
    }

    pub fn open(&mut self) -> Result<(), PoisonError<MutexGuard<bool>>> {
        let mut state = self.is_open.lock()?;
        if *state {
            return Ok(());
        }
        *state = true;
        let is_open = Arc::clone(&self.is_open);
        thread::spawn(move || {
            thread::sleep(Duration::from_secs(10));
            match is_open.lock() {
                Ok(mut state) => *state = false,
                Err(_) => {}
            }
        });
        Ok(())
    }
}

#[cfg(feature = "iot")]
impl DoorControl {
    pub fn new(pin: u8) -> DoorControl {
        return DoorControl {
            dev: Arc::new(Mutex::new(OutputDevice::new(pin))),
        };
    }

    pub fn open(&mut self) -> Result<(), PoisonError<MutexGuard<OutputDevice>>> {
        let mut dev = self.dev.lock()?;
        if dev.is_active() {
            return Ok(());
        }
        dev.on();
        let dev = Arc::clone(&self.dev);
        thread::spawn(move || {
            thread::sleep(Duration::from_secs(10));
            match dev.lock() {
                Ok(mut dev) => dev.off(),
                Err(_) => {}
            }
        });
        Ok(())
    }
}
