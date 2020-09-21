use rust_gpiozero::output_devices::OutputDevice;

pub struct DoorControl {
    dev: OutputDevice,
}

impl DoorControl {
    pub fn new(pin: u8) -> DoorControl {
        return DoorControl {
            dev: OutputDevice::new(pin),
        };
    }

    pub fn open(&mut self) {
        return self.dev.on();
    }
}
