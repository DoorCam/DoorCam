use crate::db_entry::FlatEntry;
#[cfg(feature = "iot")]
use log::{error, info};
#[cfg(feature = "iot")]
use rumqttc::{Client, MqttOptions, QoS};
#[cfg(feature = "iot")]
use rust_gpiozero::input_devices::Button;
#[cfg(feature = "iot")]
use std::sync::{Arc, Mutex};
#[cfg(feature = "iot")]
use std::thread;

/// Checks whether the button is pushed and sends a signal to the MQTT-Broker.
pub struct BellButton {
    #[cfg(feature = "iot")]
    drop_flag: Arc<Mutex<bool>>,
}

#[cfg(not(feature = "iot"))]
impl BellButton {
    pub fn new(_flat: &FlatEntry) -> Self {
        BellButton {}
    }
}

#[cfg(feature = "iot")]
impl BellButton {
    /// Spawns a thread with an event-loop
    pub fn new(flat: &FlatEntry) -> Self {
        let mqtt_conn_options =
            MqttOptions::new("doorcam", flat.broker_address.clone(), flat.broker_port);
        let (mut mqtt_client, _) = Client::new(mqtt_conn_options, 5);

        let topic = flat.bell_topic.clone();

        let mut dev = Button::new(flat.bell_button_pin);

        let drop_flag = Arc::new(Mutex::new(false));
        let drop = drop_flag.clone();

        thread::spawn(move || loop {
            dev.wait_for_press(None);
            info!("IoT: Button pressed");

            // Stops the thread if the drop-flag is set
            match drop.lock() {
                Ok(state) => {
                    if *state {
                        break;
                    }
                }
                Err(e) => error!("IoT: Can't lock drop: {}", e),
            }

            BellButton::send_bell_signal(&mut mqtt_client, &topic);
        });

        BellButton { drop_flag }
    }

    /// Sends a topic to the broker
    fn send_bell_signal(mqtt_client: &mut Client, topic: &String) {
        if let Err(e) = mqtt_client.publish(topic, QoS::ExactlyOnce, false, b"".to_vec()) {
            error!("IoT: Can't send Bell Signal: {}", e);
        }
    }
}

#[cfg(feature = "iot")]
impl Drop for BellButton {
    /// Sets the drop-flag
    fn drop(&mut self) {
        match self.drop_flag.lock() {
            Ok(mut state) => *state = true,
            Err(e) => error!("IoT: Can't lock drop: {}", e),
        }
    }
}
