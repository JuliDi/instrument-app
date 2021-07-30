//! Provides access to a device configuration file via Rust structs
pub mod command;
use serde::Deserialize;
use config::{Config, ConfigError, File};

/// Contains one [`Device`] and all [`Command`]s available for it
#[derive(Debug, Deserialize, Clone)]
pub struct Configuration {
    pub device: Device,
    pub commands: Vec<Command>,
}
/// Device properties
#[derive(Debug, Deserialize, Clone)]
pub struct Device {
    /// Address in format IP:PORT, currently unused
    pub address: String,
    /// Number of channels available on the device
    pub channels: u8,
}

/// An available SCPI command
#[derive(Debug, Deserialize, Default, Clone, PartialEq, Eq)]
pub struct Command {
    /// Per-channel command (i.e. not global)?
    pub channel: bool,
    /// Name of the command that should be displayed
    pub name: String,
    /// SCPI top-level command like "OUTPut"
    pub scpi: String,
    /// Values that may be appended to `scpi`
    pub values: Vec<String>
}

impl std::fmt::Display for Command {
    /// Make the [`Command`] displayable by returning its `name` field
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}


impl Configuration {
    /// Construct new settings object from a devices file at path `configfile`.
    pub fn from(configfile: &str) -> Result<Self, ConfigError> {
        let mut config = Config::new();
        // Start off by merging in the "default" configuration file
        config.merge(File::with_name(configfile))?;
        // You can deserialize (and thus freeze) the entire configuration as
        config.try_into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}
