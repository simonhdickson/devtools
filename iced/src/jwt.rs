use devtools_core::Jwt;
use iced::{text_input, Align, Column, Container, Element, Length, Text, TextInput};

use crate::ui::style::Theme;

#[derive(Debug, Clone)]
pub enum Message {
    JWTToken(String),
}

#[derive(Default)]
pub struct State {
    jwt_token: Jwt,
    jwt_input: text_input::State,
}

impl State {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::JWTToken(new_value) => match self.jwt_token.set_token_string(&new_value) {
                Ok(()) => (),
                Err(err) => println!("{:#?}", err),
            },
        }
    }

    pub fn view(&mut self, theme: Theme) -> Element<Message> {
        let text_input = TextInput::new(
            &mut self.jwt_input,
            "",
            &*self.jwt_token.get_token_string(),
            Message::JWTToken,
        )
        .padding(10)
        .width(Length::Fill)
        .size(30)
        .style(theme);

        let content = Column::new()
            .padding(20)
            .spacing(20)
            .align_items(Align::Start)
            .push(text_input)
            .push(Text::new("Headers").size(25))
            .push(Text::new(self.jwt_token.get_headers()).size(15))
            .push(Text::new("Payload").size(25))
            .push(Text::new(self.jwt_token.get_payload()).size(15));

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .align_y(Align::Start)
            .style(theme)
            .into()
    }
}
