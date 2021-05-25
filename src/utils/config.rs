//! Data structures for configuration

use duration_str::deserialize_duration;
use serde::Deserialize;
use serde_with::{hex::Hex, serde_as};
use std::collections::HashSet;
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
            door_opening_time: Duration::from_secs(3),
        },
        web: Web {
            mjpeg_stream_port: 8081,
        },
        security: Security {
            minimal_password_strength: 80.0,
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
    #[error("The `security.minimal_password_strength` entry has to be between 0.0 and 100.0 but is {0}.")]
    InvalidPasswordStrength(f64),
    #[allow(dead_code)]
    #[error("The `{0}` entry has to be changed to a secret value.")]
    SecretDefaultValue(String),
}

/// All configuration options regarding the `iot` module
#[derive(Debug, Deserialize, Clone)]
pub struct IoT {
    /// The GPIO pin which controls the door-opener.
    /// [Pinout Diagram](https://pinout.xyz)
    pub door_opener_pin: u8,
    /// The duration how long the door opener is activated.
    /// The format is documented [here](https://docs.rs/duration-str/latest/duration_str/)
    #[serde(deserialize_with = "deserialize_duration")]
    pub door_opening_time: Duration,
}

/// Configuration options regarding the Web
#[derive(Debug, Deserialize, Clone)]
pub struct Web {
    /// The TCP port of the MJPEG streamer
    pub mjpeg_stream_port: u16,
}

/// Configuration options regarding the Security
#[serde_as]
#[derive(Debug, Deserialize, Clone)]
pub struct Security {
    /// A minimal score of the user password which has to be exceeded to create/modify a user password.
    /// The password scoring is documented [here](https://docs.rs/passwords/latest/passwords/#scorer)
    pub minimal_password_strength: f64,
    /// The pepper is used to hash the passwords. It has to be 16 bytes long.
    /// You can generate such a value with OpenSSL.
    /// ```sh
    /// $ openssl rand -hex 16
    /// ```
    #[serde_as(as = "Hex")]
    pub hash_pepper: [u8; 16],
    /// The key is used to encrypt the MQTT passwords. It has to be 16 bytes long
    /// You can generate such a value with OpenSSL.
    /// ```sh
    /// $ openssl rand -hex 16
    /// ```
    #[serde_as(as = "Hex")]
    pub encryption_key: [u8; 16],
    /// A set of the hash configurations, which are allowed for authentication.
    /// "plain" should be removed after the first setup.
    pub allowed_hash_configs: HashSet<String>,
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

    fn validate(&self) -> Result<(), Error> {
        if !(0.0..=100.0).contains(&self.security.minimal_password_strength) {
            return Err(Error::InvalidPasswordStrength(
                self.security.minimal_password_strength,
            ));
        }

        #[cfg(not(debug_assertions))]
        {
            let default_secret = [
                0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab,
                0xcd, 0xef,
            ];
            if self.security.hash_pepper == default_secret {
                return Err(Error::SecretDefaultValue(
                    "security.hash_pepper".to_string(),
                ));
            }
            if self.security.encryption_key == default_secret {
                return Err(Error::SecretDefaultValue(
                    "security.encryption_key".to_string(),
                ));
            }
        }
        Ok(())
    }
}
