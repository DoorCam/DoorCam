/// Data structures for configuration
use serde::Deserialize;

/// All configuration options regarding the `iot` module
#[derive(Debug, Deserialize, Clone)]
pub struct IoT {
    /// The GPIO pin which controls the door-opener.
    /// [Pinout Diagram](https://pinout.xyz)
    pub door_opener_pin: u8,
}

/// Configuration options regarding the Web
#[derive(Debug, Deserialize, Clone)]
pub struct Web {
    /// The TCP port of the MJPEG streamer
    pub mjpeg_stream_port: u16,
}

/// All configuration options
#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub iot: IoT,
    pub web: Web,
}

impl Config {
    pub fn new() -> Result<Self, config::ConfigError> {
        let mut conf = config::Config::new();

        conf.merge(config::File::with_name("Config.toml"))?;

        conf.try_into()
    }
}
