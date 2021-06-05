use crate::db_entry::FlatEntry;
use crate::utils::crypto;
use log::{error, info};
use rumqttc::{Client, MqttOptions, QoS};
#[cfg(feature = "iot")]
use rust_gpiozero::input_devices::Button;
use std::convert::TryInto;
#[cfg(feature = "iot")]
use std::sync::{Arc, Mutex};
use std::thread;

/// Checks whether the button is pushed and sends a signal to the MQTT-Broker.
pub struct BellButton {
    #[cfg(feature = "iot")]
    drop_flag: Arc<Mutex<bool>>,
}

#[cfg(not(feature = "iot"))]
impl BellButton {
    pub fn new(flat: &FlatEntry) -> Self {
        let broker_password = match Self::decrypt_broker_password(flat) {
            Ok(broker_password) => broker_password,
            Err(e) => {
                error!("Error decrypting broker password: {}", e);
                return Self {};
            }
        };
        let mut mqtt_conn_options =
            MqttOptions::new("doorcam", flat.broker_address.clone(), flat.broker_port);
        mqtt_conn_options.set_credentials(flat.broker_user.clone(), broker_password);
        let (mut mqtt_client, mut mqtt_conn) = Client::new(mqtt_conn_options, 5);

        Self::send_bell_signal(&mut mqtt_client, &flat.bell_topic.clone());

        thread::spawn(move || {
            mqtt_conn.iter().for_each(|notification| {
                info!("IoT: Received MQTT notification: {:?}", notification)
            });
        });

        Self {}
    }
}

#[cfg(feature = "iot")]
impl BellButton {
    /// Spawns a thread with an event-loop
    pub fn new(flat: &FlatEntry) -> Self {
        let broker_password = match Self::decrypt_broker_password(&flat) {
            Ok(broker_password) => broker_password,
            Err(e) => {
                error!("Error decrypting broker password: {}", e);
                return Self {
                    drop_flag: Arc::new(Mutex::new(false)),
                };
            }
        };
        let mut mqtt_conn_options =
            MqttOptions::new("doorcam", flat.broker_address.clone(), flat.broker_port);
        mqtt_conn_options.set_credentials(flat.broker_user.clone(), broker_password);
        let (mut mqtt_client, mut mqtt_conn) = Client::new(mqtt_conn_options, 5);

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

            Self::send_bell_signal(&mut mqtt_client, &topic);
        });

        thread::spawn(move || {
            mqtt_conn.iter().for_each(|notification| {
                info!("IoT: Received MQTT notification: {:?}", notification)
            });
        });

        Self { drop_flag }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum DecryptionError {
    #[error(transparent)]
    Base64Decode(#[from] base64::DecodeError),
    #[error(transparent)]
    Decryption(#[from] block_modes::BlockModeError),
    #[error(transparent)]
    Utf8(#[from] std::string::FromUtf8Error),
    #[error("The Initialization Vector has the wrong length of {0} instead of 16")]
    InvalidInitializationVector(usize),
}

impl BellButton {
    /// Sends a topic to the broker
    fn send_bell_signal(mqtt_client: &mut Client, topic: &str) {
        if let Err(e) = mqtt_client.publish(topic, QoS::ExactlyOnce, false, b"".to_vec()) {
            error!("IoT: Can't send Bell Signal: {}", e);
        }
    }

    fn decrypt_broker_password(flat: &FlatEntry) -> Result<String, DecryptionError> {
        let broker_password_iv: [u8; 16] = base64::decode(flat.broker_password_iv.clone())?
            .try_into()
            .map_err(|iv: Vec<_>| DecryptionError::InvalidInitializationVector(iv.len()))?;
        let encrypted_broker_password = base64::decode(flat.broker_password.clone())?;

        Ok(String::from_utf8(crypto::symetric_decrypt(
            &crate::CONFIG.security.encryption_key,
            &broker_password_iv,
            &encrypted_broker_password,
        )?)?)
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
