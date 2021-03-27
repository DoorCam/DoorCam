use duration_str::deserialize_duration;
/// Data structures for configuration
use serde::Deserialize;
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
    #[error("The pepper has a length of {0} but should have a length of 16")]
    PepperWrongLen(usize),
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
#[derive(Debug, Deserialize, Clone)]
pub struct Security {
    /// The pepper is used to hash the passwords. It should be 16 bytes long
    #[serde(with = "serde_bytes")]
    pub hash_pepper: Vec<u8>,
    /// A minimal score of the user password which has to be exceeded to create/modify a user password.
    /// The password scoring is documented [here](https://docs.rs/passwords/latest/passwords/#scorer)
    pub minimal_password_strength: f64,
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
        if self.security.hash_pepper.len() != 16 {
            return Err(Error::PepperWrongLen(self.security.hash_pepper.len()));
        }
        Ok(())
    }
}
