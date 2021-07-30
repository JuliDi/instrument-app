//! Provides a toolbar for navigating the application
use iced::{button, Button, Container, Row, Text, Length, Align, Column, Rule};

use crate::messages::*;
use crate::styles;

/// Button states held by the toolbar
#[derive(Default)]
pub struct ToolBar {
    settings_button_state: button::State,
    command_button_state: button::State,
}

impl ToolBar {

    /// Define the button styles according to what [`ScreenType`] is displayed
    pub fn view(&mut self, content: &ScreenType) -> Container<Message> {

        Container::new(Column::new().push(
            Row::new()
                .align_items(Align::Center)
                .spacing(24)
                .push(
                    Button::new(&mut self.command_button_state, Text::new("Command"))
                        .on_press(Message::ChangeView(ScreenType::Command))
                        .style(match content {
                            ScreenType::Settings => {styles::Button::Control}
                            ScreenType::Command => {styles::Button::ActiveControl}
                        }),
                )
                .push(
                    Button::new(&mut self.settings_button_state, Text::new("Settings"))
                        .on_press(Message::ChangeView(ScreenType::Settings))
                        .style(match content {
                            ScreenType::Settings => {styles::Button::ActiveControl}
                            ScreenType::Command => {styles::Button::Control}
                        }),
                )
        )
            .push(Rule::horizontal(20))
            .width(Length::Fill)
            .align_items(Align::Center)
        )
            .align_x(Align::Center)
            //.style(styles::Container::Basic)
            .align_y(Align::Start)
            .width(Length::Fill)
            .padding(4)
    }
    /*
    pub fn update(&mut self, message: ToolbarMessage) {

        /* maybe do some toolbar updating here */
    }
     */
}