//! Provides a settings screen to define connection parameters and connect to the device

use crate::{styles::{self, ui_button}, messages::*, networking::DeviceConnection};
use iced::{Container, Text, text_input, button, Length, Align, TextInput, Color, Column};
use std::{ops::Add, net::{SocketAddr}};

/// Textfield and button states held by the settings screen
#[derive(Default)]
pub struct SettingsScreen {
    ip_address_state: text_input::State,
    ip_address: String,
    port_state: text_input::State,
    port: String,
    save_button_state: button::State,
    address: Option<SocketAddr>,
    status_text: String,
    connection_status_text: String,
    device_connection: DeviceConnection,
}

impl SettingsScreen {
    /// Creates a new settings screen with default values and a reference to a device connection
    pub fn default() -> Self {
        Self {
            status_text: "Enter an IP".into(),
            connection_status_text: "Not connected".into(),
            ..Default::default()
        }
    }

    /// Display a settings screen according to what data has been entered by the user
    /// This also takes into account whether the IP and port are parsable and provides information
    /// about the current status of the connection.
    pub fn view(&mut self) -> Container<Message> {
        // Make a TextInput for the IP address
        let mut ti_ip_address =
            TextInput::new(&mut self.ip_address_state,
                           "IP Address",
                           &self.ip_address,
                           |ip| { Message::Settings(SettingsMessage::IpEntered(ip)) })
                .width(Length::Units(150))
                .padding(5);
        // Make a TextInput for the port
        let mut ti_port =
            TextInput::new(&mut self.port_state,
                           "Port",
                           &self.port,
                           |port| { Message::Settings(SettingsMessage::PortEntered(port)) })
                .width(Length::Units(80))
                .padding(5);

        // Make a save button
        let mut save_button =
            ui_button(&mut self.save_button_state,
                      "Save".into(),
                      styles::Button::Submit)
                .width(Length::Units(150));

        // If we have a valid address, make the form submittable via save button or enter key
        if let Some(addr) = self.address {
            save_button = save_button.on_press(Message::Settings(SettingsMessage::Connect(addr)));
            ti_ip_address = ti_ip_address.on_submit(Message::Settings(SettingsMessage::Connect(addr)));
            ti_port = ti_port.on_submit(Message::Settings(SettingsMessage::Connect(addr)));
        }

        // Put everything together in a container and add some info text
        Container::new(
            Column::new()
                .align_items(Align::Center)
                .push(Text::new(&self.connection_status_text).color(Color::BLACK).width(Length::Shrink))
                .spacing(20)
                .push(ti_ip_address)
                .push(ti_port)
                .push(Text::new(&self.status_text).color(Color::BLACK).width(Length::Shrink))
                .push(save_button)
        )
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
    }

    /// Update the fields according to generated [`SettingsMessage`]s
    pub fn update(&mut self, message: SettingsMessage) {
        match message {
            SettingsMessage::IpEntered(ip) => self.ip_address = ip,
            SettingsMessage::PortEntered(port) => self.port = port,
            SettingsMessage::Connect(addr) => {
                self.connection_status_text = match self.device_connection.connect(&addr) {
                    Ok(_) => { format!("Connected to {}", addr) }
                    Err(e) => { format!("Connection failed: Error: {}", e) }
                }
            }
        }

        // Check whether the entered IP is valid or not
        if let Ok(addr) = crate::networking::parse_ip(&self.combine_ip_and_port()) {
            self.address = Some(addr);
            self.status_text = "IP OK".into();
        } else {
            self.address = None; // TODO: Check whether it is smart to "delete" the address here or not
            self.status_text = "IP/port not parsable".into();
        }
    }

    /// Utility function to create an address in the form of `IP:PORT`
    fn combine_ip_and_port(&self) -> String {
        self.ip_address.clone().add(":").add(&self.port)
    }

    /// Return the [`SocketAddr`] held by this screen wrapped in an option
    /// Returns [`None`] if there is no valid address entered
    #[allow(dead_code)]
    pub fn get_address(&self) -> Option<SocketAddr> {
        self.address
    }

    /// Return a mutable reference to the [`DeviceConnection`]
    pub fn device_connection(&mut self) -> &mut DeviceConnection {
        &mut self.device_connection
    }
}
