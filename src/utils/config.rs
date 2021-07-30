//! Data structures for configuration

use super::serde::deserialize_optional_duration;
use bool_ext::BoolExt;
use duration_str::deserialize_duration;
use serde::Deserialize;
use serde_with::{hex::Hex, serde_as};
use std::collections::HashSet;
use std::ops::Not;
use std::time::Duration;

#[cfg(not(test))]
lazy_static! {
    pub static ref CONFIG: Config = match Config::new() {
        Ok(conf) => conf,
        Err(error) => panic!("Config Error: {}", error),
    };
}

#[cfg(test)]
lazy_static! {
    pub static ref CONFIG: Config = Config {
        iot: IoT {
            door_opener_pin: 0,
            tamper_sensor_pin: None,
            door_opening_time: Duration::from_secs(3),
            bell_debounce_interval: Duration::from_millis(42),
            tamper_sensor_debounce_interval: None,
        },
        web: Web {
            mjpeg_stream_port: 8081,
        },
        security: Security {
            minimal_password_strength_score: 80.0,
            hash_pepper: [
                0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab,
                0xcd, 0xef,
            ],
            encryption_key: [
                0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab,
                0xcd, 0xef,
            ],
            allowed_hash_configs: hashset!["Blake2b".to_string()],
        },
    };
}

/// All errors which could happen during user creation, authentification and authorization.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    ConfigInternal(#[from] config::ConfigError),
    #[error("The `{name}` entry has to be between 0 and 27 but is {pin}.")]
    InvalidGpioPin { name: String, pin: u8 },
    #[error("The `security.minimal_password_strength` entry has to be between 0.0 and 100.0 but is {0}.")]
    InvalidPasswordStrength(f64),
    #[allow(dead_code)]
    #[error("The `{0}` entry has to be changed to a secret value.")]
    SecretDefaultValue(String),
    #[error("The `security.allowed_hash_configs` entry is empty.")]
    EmptyHashConfigs,
}

/// A trait for validating the configuration recursively.
trait ConfigValidator {
    fn validate(&self) -> Result<(), Error>;
}

/// All configuration options regarding the `iot` module
#[derive(Debug, Deserialize, Clone)]
pub struct IoT {
    /// The GPIO pin which controls the door-opener.
    /// [Pinout Diagram](https://pinout.xyz).
    pub door_opener_pin: u8,

    /// The optional GPIO pin for a tamper sensor which sets an alarm off when there is no connection.
    pub tamper_sensor_pin: Option<u8>,

    /// The duration how long the door opener is activated.
    /// The format is documented [here](https://docs.rs/duration-str/latest/duration_str/).
    #[serde(deserialize_with = "deserialize_duration")]
    pub door_opening_time: Duration,

    /// The minimal duration between two signals.
    /// The timer of the last signal is resetted on every signal.
    /// The format is documented [here](https://docs.rs/duration-str/latest/duration_str/).
    #[serde(deserialize_with = "deserialize_duration")]
    pub bell_debounce_interval: Duration,

    /// The minimal duration between two signals.
    /// The timer of the last signal is resetted on every signal.
    /// The format is documented [here](https://docs.rs/duration-str/latest/duration_str/).
    #[serde(default, deserialize_with = "deserialize_optional_duration")]
    pub tamper_sensor_debounce_interval: Option<Duration>,
}

impl IoT {
    fn validate_gpio(pin: u8, name: String) -> Result<(), Error> {
        (0..=27)
            .contains(&pin)
            .err(Error::InvalidGpioPin { name, pin })
    }
}

impl ConfigValidator for IoT {
    fn validate(&self) -> Result<(), Error> {
        Self::validate_gpio(self.door_opener_pin, "iot.door_opener_pin".to_string())?;
        if let Some(tamper_sensor_pin) = self.tamper_sensor_pin {
            Self::validate_gpio(tamper_sensor_pin, "iot.tamper_sensor_pin".to_string())?;
        }
        Ok(())
    }
}

/// Configuration options regarding the Web
#[derive(Debug, Deserialize, Clone)]
pub struct Web {
    /// The TCP port of the MJPEG streamer
    pub mjpeg_stream_port: u16,
}

/// You can generate such a value in hexadecimal representation with OpenSSL.
/// ```sh
/// $ openssl rand -hex 16
/// ```
type Secret128Bit = [u8; 16];

/// Configuration options regarding the Security
#[serde_as]
#[derive(Debug, Deserialize, Clone)]
pub struct Security {
    /// A minimal score of the user password which has to be exceeded to create/modify a user password.
    /// The password scoring is documented [here](https://docs.rs/passwords/latest/passwords/#scorer)
    pub minimal_password_strength_score: f64,
    /// The pepper is used to hash the passwords. It has to be 16 bytes long.
    /// You can generate such a value with OpenSSL.
    /// ```sh
    /// $ openssl rand -hex 16
    /// ```
    #[serde_as(as = "Hex")]
    pub hash_pepper: Secret128Bit,
    /// The key is used to encrypt the MQTT passwords. It has to be 16 bytes long
    /// You can generate such a value with OpenSSL.
    /// ```sh
    /// $ openssl rand -hex 16
    /// ```
    #[serde_as(as = "Hex")]
    pub encryption_key: Secret128Bit,
    /// A set of the hash configurations, which are allowed for authentication.
    /// "plain" should be removed after the first setup.
    pub allowed_hash_configs: HashSet<String>,
}

impl Security {
    #[cfg(not(debug_assertions))]
    fn validate_secret(secret: Secret128Bit, name: String) -> Result<(), Error> {
        const DEFAULT_SECRET: Secret128Bit = [
            0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab,
            0xcd, 0xef,
        ];
        (secret != DEFAULT_SECRET).err(Error::SecretDefaultValue(name))
    }
    #[cfg(debug_assertions)]
    fn validate_secret(_secret: Secret128Bit, _name: String) -> Result<(), Error> {
        Ok(())
    }
}

impl ConfigValidator for Security {
    fn validate(&self) -> Result<(), Error> {
        (0.0..=100.0)
            .contains(&self.minimal_password_strength_score)
            .err(Error::InvalidPasswordStrength(
                self.minimal_password_strength_score,
            ))?;

        Self::validate_secret(self.hash_pepper, "security.hash_pepper".to_string())?;
        Self::validate_secret(self.hash_pepper, "security.hash_pepper".to_string())?;

        self.allowed_hash_configs
            .is_empty()
            .not()
            .err(Error::EmptyHashConfigs)
    }
}

/// All configuration options
#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub iot: IoT,
    pub web: Web,
    pub security: Security,
}

impl Config {
    pub fn new() -> Result<Self, Error> {
        let mut conf = config::Config::new();

        conf.merge(config::File::with_name("Config.toml"))?;

        let conf: Self = conf.try_into()?;

        conf.validate()?;

        Ok(conf)
    }
}

impl ConfigValidator for Config {
    fn validate(&self) -> Result<(), Error> {
        self.iot.validate()?;

        self.security.validate()
    }
}
