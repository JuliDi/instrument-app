//! Provides styles for [`button`]s and [`text_input`]s

use iced::{button, button::State, Background, Color, Vector, Text, text_input, text_input::Style, alignment};
use crate::Message;

pub const GREEN_LIGHT: Color = Color { r: 0.08, g: 0.7, b: 0.16, a: 1.0 };
pub const GREEN_DARK: Color = Color { r: 0.06, g: 0.6, b: 0.16, a: 1.0 };

pub const GRAY_DARK: Color = Color { r: 0.8, g: 0.8, b: 0.8, a: 1.0 };
pub const GRAY_LIGHT: Color = Color { r: 0.9, g: 0.9, b: 0.9, a: 1.0 };

/// Different style [`Button`]s
pub enum Button {
    /// Control button for e.g. the toolbar
    Control,
    /// Active control button for e.g. the toolbar when the respective view is open
    ActiveControl,
    /// Submit button, used to save or send a form
    Submit,
    /// Cancel button, used to discard changes or stop an operation
    #[allow(dead_code)]
    Cancel,
}

/// Returns a standardized button for use throughout the UI
/// ## Arguments
/// - `state`: the button's [`State`] that is held by its parent
/// - `label`: a [`String`] specifying the text on the button
/// - `style`: a variant of a [`Button`] that defines its style
pub fn ui_button(state: &mut State, label: String, style: Button) -> iced::Button<Message> {
    iced::Button::new(
        state,
        Text::new(label)
            .horizontal_alignment(alignment::Horizontal::Center),
    )
        .width(80.into())
        .padding(10)
        .style(style)
}

impl button::StyleSheet for Button {
    /// Defines the active style of the button
    fn active(&self) -> button::Style {
        let mut st = self.default_style();
        match self {
            Button::Submit => {
                st.background = Some(Background::Color(GREEN_LIGHT));
            }
            Button::Control => {}
            Button::ActiveControl => {
                st.background = Some(Background::Color(GRAY_LIGHT));
            }
            Button::Cancel => {
                st.background = Some(Background::Color(Color::from_rgb8(255, 0, 0)));
            }
        }
        st
    }
    /// Defines the hovered style of the button
    fn hovered(&self) -> button::Style {
        let mut st = self.default_style();
        match self {
            Button::Submit => {
                st.background = Some(Background::Color(GREEN_DARK));
            }
            Button::Control | Button::ActiveControl => {
                st.background = Some(Background::Color(GRAY_DARK));
            }

            Button::Cancel => {
                st.background = Some(Background::Color(Color::from_rgb8(255, 0, 50)));
            }
        }
        st
    }
}

impl Button {
    /// Defines the default style of the button
    fn default_style(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(Color::WHITE)),
            shadow_offset: Vector::new(1.0, 1.0),
            text_color: Color::BLACK,
            border_color: Color::BLACK,
            border_radius: 4.0,
            border_width: 1.0,
            ..button::Style::default() // .. means: other fields from default
        }
    }
}

/// Different style [`Textbox`]es
pub enum Textbox {
    /// Freetext the user should enter
    Freetext
}

impl text_input::StyleSheet for Textbox {
    /// Defines the active style of the textbox
    fn active(&self) -> text_input::Style {
        text_input::Style { background: Background::Color(Color::WHITE), border_color: Color::from_rgb(0.7, 0.7, 0.7), border_radius: 0.0, border_width: 1.0}
    }
    /// Defines the focused (i.e. cursor inside) style of the textbox
    fn focused(&self) -> Style {
        text_input::Style {border_color: Color::from_rgb(0.5, 0.5, 0.5), ..self.active() }
    }

    /// Defines the color of the placeholder text
    fn placeholder_color(&self) -> Color {
        GRAY_DARK
    }

    /// Defines the color of the user-entered text
    fn value_color(&self) -> Color {
        Color::BLACK
    }

    /// Defines the color of selected user-entered text
    fn selection_color(&self) -> Color {
        Color::from_rgb(0.8, 0.8, 1.0)
    }
}
