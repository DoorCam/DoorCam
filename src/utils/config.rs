//! Data structures for configuration

use super::serde::deserialize_optional_duration;
use bool_ext::BoolExt;
use duration_str::deserialize_duration;
use serde::Deserialize;
use serde_with::{hex::Hex, serde_as};
use std::collections::HashSet;
use std::convert::{TryFrom, TryInto};
use std::ops::Not;
use std::time::Duration;

#[cfg(test)]
#[path = "./config_test.rs"]
mod config_test;

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
            used_password_hash: PasswordHashConfig::Argon2(Argon2Config {
                algorithm: Argon2Algorithm::Argon2d,
                memory_cost: 1,
                time_cost: 1,
                parallelism: 1,
            }),
            minimal_hashing_duration: Duration::from_millis(42),
            maximal_hashing_duration: Duration::from_millis(170),
        },
    };
}

/// All errors which could happen during user creation, authentification and authorization.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    ConfigInternal(#[from] config::ConfigError),
    #[error("The `security.used_password_hash` entry is invalid: {0}")]
    InvalidHashConfig(argon2::Error),
    #[error("The `{name}` entry has to be between 0 and 27 but is {pin}.")]
    InvalidGpioPin { name: String, pin: u8 },
    #[error("The `security.minimal_password_strength` entry has to be between 0.0 and 100.0 but is {0}.")]
    InvalidPasswordStrength(f64),
    #[allow(dead_code)]
    #[error("The `{0}` entry has to be changed to a secret value.")]
    SecretDefaultValue(String),
    #[error(
        "The `security.minimal_hashing_duration` is bigger as `security.maximal_hashing_duration`."
    )]
    WrongHashingDurationOrdering,
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

    /// The password hash used to store new passwords.
    pub used_password_hash: PasswordHashConfig,

    /// The minimal `Duration` which is needed to hash the password. This should be configured
    /// according to the configured `used_password_hash` and the hardware. This is needed to prevent a
    /// malicious actor from enumerating the valid users by th time differential.
    #[serde(deserialize_with = "deserialize_duration")]
    pub minimal_hashing_duration: Duration,

    /// The maximal `Duration` which is needed to hash the password. This should be configured
    /// according to the configured `used_password_hash` and the hardware. This is needed to prevent a
    /// malicious actor from enumerating the valid users by th time differential.
    #[serde(deserialize_with = "deserialize_duration")]
    pub maximal_hashing_duration: Duration,
}

impl Security {
    #[cfg(not(all(debug_assertions, test)))]
    fn validate_secret(secret: Secret128Bit, name: String) -> Result<(), Error> {
        const DEFAULT_SECRET: Secret128Bit = [
            0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab,
            0xcd, 0xef,
        ];
        (secret != DEFAULT_SECRET).err(Error::SecretDefaultValue(name))
    }
    #[cfg(any(debug_assertions, test))]
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
            .err(Error::EmptyHashConfigs)?;

        self.used_password_hash.validate()?;

        (self.minimal_hashing_duration <= self.maximal_hashing_duration)
            .err(Error::WrongHashingDurationOrdering)
    }
}

/// Configure all hashing algorithms. It is untagged so you don't have to store it redundantly.
#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum PasswordHashConfig {
    Argon2(Argon2Config),
}

impl ConfigValidator for PasswordHashConfig {
    fn validate(&self) -> Result<(), Error> {
        match self {
            Self::Argon2(argon2_config) => argon2_config.validate(),
        }
    }
}

/// The configuration interface for the Argon2 hashing function. It uses the same [parameters as
/// the argon2 crate](https://docs.rs/argon2/latest/argon2/struct.Argon2.html). The OWASP
/// recomenations are [here](https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html#argon2id).
#[derive(Debug, Deserialize, Clone)]
pub struct Argon2Config {
    algorithm: Argon2Algorithm,
    memory_cost: u32,
    time_cost: u32,
    parallelism: u8,
}

impl ConfigValidator for Argon2Config {
    fn validate(&self) -> Result<(), Error> {
        self.try_into()
            .map(|_: (argon2::Algorithm, argon2::Params)| ())
            .map_err(Error::InvalidHashConfig)
    }
}

impl TryFrom<&Argon2Config> for (argon2::Algorithm, argon2::Params) {
    type Error = argon2::Error;

    fn try_from(config: &Argon2Config) -> Result<Self, Self::Error> {
        Ok((
            (&config.algorithm).into(),
            argon2::Params::new(
                config.memory_cost,
                config.time_cost,
                config.parallelism as u32,
                None,
            )?,
        ))
    }
}

/// The (de)serialization is in lowercase.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Argon2Algorithm {
    Argon2d,
    Argon2i,
    Argon2id,
}

impl From<&Argon2Algorithm> for argon2::Algorithm {
    fn from(config: &Argon2Algorithm) -> Self {
        match config {
            Argon2Algorithm::Argon2d => Self::Argon2d,
            Argon2Algorithm::Argon2i => Self::Argon2i,
            Argon2Algorithm::Argon2id => Self::Argon2id,
        }
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
        let conf: Self = config::Config::builder()
            .add_source(config::File::with_name("Config"))
            .build()?
            .try_deserialize()?;

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
