use devtools_core::Base64;
use iced::{text_input, Align, Column, Container, Element, Length, Text, TextInput};

use crate::ui::style::Theme;

#[derive(Debug, Clone)]
pub enum Message {
    SetBase64(String),
    SetPlainText(String),
}

pub struct State {
    base64: Base64,
    from_base64_state: text_input::State,
    to_base64_state: text_input::State,
}

impl Default for State {
    fn default() -> Self {
        Self {
            base64: Default::default(),
            from_base64_state: text_input::State::new(),
            to_base64_state: text_input::State::new(),
        }
    }
}

impl State {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::SetBase64(new_value) => match self.base64.set_base64(&new_value) {
                Ok(()) => (),
                Err(err) => println!("{:#?}", err),
            },
            Message::SetPlainText(new_value) => {
                self.base64.set_plain_text(&new_value);
            }
        }
    }

    pub fn view(&mut self, theme: Theme) -> Element<Message> {
        let to_input = TextInput::new(
            &mut self.to_base64_state,
            "",
            &*self.base64.get_plain_text(),
            Message::SetPlainText,
        )
        .padding(10)
        .size(30)
        .style(theme);

        let from_input = TextInput::new(
            &mut self.from_base64_state,
            "",
            &*self.base64.get_base64(),
            Message::SetBase64,
        )
        .padding(10)
        .size(30)
        .style(theme);

        let content = Column::new()
            .padding(20)
            .spacing(20)
            .align_items(Align::Center)
            .push(Text::new("Plain Text").size(35))
            .push(to_input)
            .push(Text::new("Base64").size(35))
            .push(from_input);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .align_y(Align::Start)
            .style(theme)
            .into()
    }
}
