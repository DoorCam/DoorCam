//! Data structures for configuration

use duration_str::deserialize_duration;
use serde::Deserialize;
use serde_with::{serde_as, Bytes};
use std::time::Duration;

lazy_static! {
    pub static ref CONFIG: Config = match Config::new() {
        Ok(conf) => conf,
        Err(error) => panic!("Config Error: {}", error),
    };
}

/// All errors which could happen during user creation, authentification and authorization.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    ConfigInternal(#[from] config::ConfigError),
    #[error("The `minimal_password_strength` has to be between 0 and 100 but is {0}.")]
    InvalidPasswordStrength(f64),
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
    /// The pepper is used to hash the passwords. It has to be 16 bytes long
    #[serde_as(as = "Bytes")]
    pub hash_pepper: [u8; 16],
    /// A minimal score of the user password which has to be exceeded to create/modify a user password.
    /// The password scoring is documented [here](https://docs.rs/passwords/latest/passwords/#scorer)
    pub minimal_password_strength: f64,
    /// The key is used to encrypt the MQTT passwords. It has to be 16 bytes long
    #[serde_as(as = "Bytes")]
    pub encryption_key: [u8; 16],
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

        println!("pepper: {:?}", conf.security.hash_pepper);

        Ok(conf)
    }

    fn validate(&self) -> Result<(), Error> {
        if !(0.0..=100.0).contains(&self.security.minimal_password_strength) {
            return Err(Error::InvalidPasswordStrength(
                self.security.minimal_password_strength,
            ));
        }
        Ok(())
    }
}
