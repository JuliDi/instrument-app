//! Collections of screens or screen-parts for the application
/// Settings screen where users specify connection parameters and establish a connection
pub mod settings;

/// Command selection screen where users build an SCPI command and send it to the device
pub mod command_selection;

/// Toolbar that is always displayed at the top of the screen and allows for switching between the other screens
pub mod toolbar;