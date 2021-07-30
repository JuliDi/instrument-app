//! Collection of Message types used througout the application
use std::net::SocketAddr;

type ScpiCommand = crate::devices::Command;

/// Messages that are passed to and handled by the main application
#[derive(Debug, Clone)]
pub enum Message{
    /// Contains the request [`ScreenType`]
    ChangeView(ScreenType),
    /// Wrapper for [`SettingsMessage`] that should be passed to the settings screen
    Settings(SettingsMessage),
    /// Wrapper for [`CommandMessage`] that should be passed to the command screen
    Command(CommandMessage),
    /// Contains the command to be sent to the device
    SendCommand(String)
}

/// Represents all available types of screens/views
#[derive(Debug, Clone)]
pub enum ScreenType {
    Settings,
    Command,
}

impl Default for ScreenType {
    /// Set the default screen that should be shown after startup
    fn default() -> Self { ScreenType::Settings }
}

/// Represents all available messages generated by the command screen
#[derive(Debug, Clone)]
pub enum CommandMessage {
    /// New command as defined in the device config file has been selected
    CommandSelected(ScpiCommand),
    /// The channel the command should be sent to has been selected
    ChannelSelected(u8),
    /// An argument for the command has been selected from the dropdown menu
    ArgumentSelected(String),
    /// A freetext argument for the command has been entered
    FreetextEntered(String)
}

/// Represents all available messages generated by the settings screen
#[derive(Debug, Clone)]
pub enum SettingsMessage {
    /// A device IP address has been entered
    IpEntered(String),
    /// A port has been entered
    PortEntered(String),
    /// Connection to the [`SocketAddr`] has been requested
    Connect(SocketAddr)
}
