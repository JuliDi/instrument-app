use crate::{styles::{self, ui_button}, ScpiCommand, devices, messages::{*, CommandMessage::*}};
use iced::{Container, Text, button, Length, alignment::Alignment, Row, Column, PickList, pick_list, TextInput, text_input};

/// Textfield, button and dropdown states held by the settings screen
#[derive(Default)]
pub struct CommandScreen {
    /// List of [`ScpiCommand`]s
    commands: Vec<ScpiCommand>,
    /// Currently selected [`ScpiCommand`]
    command_selection: ScpiCommand,
    /// State of the picklist for commands
    command_list_state: pick_list::State<ScpiCommand>,
    /// State of the command-send button
    send_button_state: button::State,
    /// List of available channels as [`u8`]
    channels: Vec<u8>,
    /// Currently selected channel
    channel_selection: u8,
    /// State of the picklist for channels
    channel_list: pick_list::State<u8>,
    /// List of arguments for the command
    arguments: Vec<String>,
    /// Currently selected Argument
    argument_selection: String,
    /// State of the picklist for arguments
    argument_list: pick_list::State<String>,
    /// Content of the freetext textbox
    freetext: String,
    /// State of the freetext textbox
    freetext_state: text_input::State,

    /// Currently selected complete command
    current_command: String,
    /// Status message
    status: String,
}

impl CommandScreen {
    pub fn from(config: devices::Configuration) -> Self {
        let mut command_screen = Self {
            commands: config.commands.clone(),
            command_selection: config.commands[0].clone(),
            send_button_state: button::State::default(),
            channels: (1..=config.device.channels.clone()).collect(),
            channel_selection: 1,
            arguments: config.commands[0].values.clone(),
            argument_selection: config.commands[0].values[0].clone(),
            status: "Choose a command to start!".into(),
            ..Default::default()
        };
        command_screen.current_command = command_screen.get_command();
        command_screen
    }
    /// Display a the command selection screen using dropdown selection lists and textboxes
    /// Provides information about the response the device sent.
    pub fn view(&mut self) -> Container<Message> {
        // Create a status text label from the current status
        let status_text = Text::new(format!(
            "{}", &self.status
        )).size(20);

        // Create a text label that displays the SCPI command that will be sent to the device
        let command_text = Text::new(format!(
            "{}", &self.current_command
        )).size(40);

        // Create a submit button that sends a message to the application root containing the selected SCPI command
        let submit_button = ui_button(&mut self.send_button_state, "Submit".into(), styles::Button::Submit)
                .on_press(Message::SendCommand(self.current_command.clone()));

        // Create a textbox for freetext entry if the command argument contains a "<TXT>"
        // if not, add an empty row to avoid re-arranging the UI whenever this switches
        let freetext =
            if self.argument_selection.contains("<TXT>") {
                Row::new().push(
                    TextInput::new(&mut self.freetext_state, "TXT", &self.freetext, |txt| { Message::Command(CommandMessage::FreetextEntered(txt)) })
                        .width(Length::Units(150))
                        .style(styles::Textbox::Freetext)
                        .padding(5)
                        .on_submit(Message::SendCommand(self.current_command.clone()))
                )
            } else {
                Row::new().push(iced::widget::Space::new(Length::Units(150), Length::Units(30)))
            };

        // Build the container from the above widgets and add the appropriate dropdown menus
        Container::new(
            Column::new()
                .align_items(Alignment::Center)
                .push(status_text)
                .spacing(20)
                .push(command_text)
                .spacing(20)
                .push(
                    Row::new().
                        push(
                            PickList::new(
                                &mut self.command_list_state,
                                &self.commands,
                                Some(self.command_selection.clone()),
                                |cmd| Message::Command(CommandMessage::CommandSelected(cmd)),
                            ).width(Length::Units(200))
                        ).spacing(20)
                        .push(
                            PickList::new(
                                &mut self.channel_list,
                                &self.channels,
                                Some(self.channel_selection.clone()),
                                |chan| Message::Command(CommandMessage::ChannelSelected(chan)),
                            ).width(Length::Units(100))
                        ).spacing(20)
                        .push(
                            PickList::new(
                                &mut self.argument_list,
                                &self.arguments,
                                Some(self.argument_selection.clone()),
                                |arg| Message::Command(CommandMessage::ArgumentSelected(arg)),
                            ).width(Length::Units(200))
                        )
                )
                .push(freetext)
                .push(
                    submit_button
                )
        )
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
    }

    /// Update the fields according to generated [`CommandMessage`]s
    pub fn update(&mut self, message: CommandMessage) {
        match message {
            CommandSelected(cmd) => {
                println!("{}", &cmd.name);
                self.command_selection = cmd.clone();
                self.arguments = cmd.values.clone();
                self.argument_selection = self.arguments[0].clone();
                self.argument_list = pick_list::State::default();
            }
            ChannelSelected(chan) => self.channel_selection = chan,
            ArgumentSelected(arg) => self.argument_selection = arg,
            FreetextEntered(txt) => self.freetext = txt,
        }
        self.current_command = self.get_command();
    }

    /// Returns a complete SCPI command from the selected/entered values on the screen
    pub fn get_command(&self) -> String {
        devices::command::make_scpi_command(self.command_selection.clone(), self.channel_selection, &self.argument_selection, &self.freetext).unwrap()
    }

    /// Set the status text that is displayed above the scpi command
    pub fn set_status_text(&mut self, status: String) {
        self.status = status
    }
}

