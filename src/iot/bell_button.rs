use crate::db_entry::FlatEntry;
#[cfg(feature = "iot")]
use rumqttc::{Client, ClientError, MqttOptions, QoS};
#[cfg(feature = "iot")]
use rust_gpiozero::input_devices::Button;

pub struct BellButton {
    #[cfg(feature = "iot")]
    dev: Button,
    #[cfg(feature = "iot")]
    mqtt_client: Client,
    #[cfg(feature = "iot")]
    topic: String,
}

#[cfg(not(feature = "iot"))]
impl BellButton {
    pub fn new(_flat: &FlatEntry) -> Self {
        BellButton {}
    }

    pub fn events(&self) -> Result<(), String> {
        Ok(())
    }
}

#[cfg(feature = "iot")]
impl BellButton {
    pub fn new(flat: &FlatEntry) -> Self {
        let mqtt_conn_options = MqttOptions::new("doorcam", flat.broker_address.clone(), 1883);
        let (client, _) = Client::new(mqtt_conn_options, 5);
        BellButton {
            dev: Button::new(flat.bell_button_pin),
            mqtt_client: client,
            topic: flat.bell_topic.clone(),
        }
    }

    pub fn events(&mut self) -> Result<(), ClientError> {
        if self.dev.is_active() {
            self.send_bell_signal()?;
        }
        Ok(())
    }

    pub fn send_bell_signal(&mut self) -> Result<(), ClientError> {
        self.mqtt_client
            .publish(&self.topic, QoS::ExactlyOnce, false, b"".to_vec())?;
        Ok(())
    }
}
