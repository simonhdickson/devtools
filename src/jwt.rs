use std::collections::BTreeMap;

use iced::{text_input, Align, Column, Element, Length, Row, Text, TextInput};
use jwt::{Header, Token, Unverified};

use crate::ui;

#[derive(Debug, Clone)]
pub enum Message {
    JWTToken(String),
}

#[derive(Default)]
pub struct State {
    jwt_token: String,
    jwt_input: text_input::State,
    headers: String,
    payload: String,
}

impl State {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::JWTToken(new_value) => {
                self.jwt_token = new_value;

                match Token::<Header, BTreeMap<String, serde_json::Value>, Unverified>::parse_unverified(
                    &*self.jwt_token,
                ) {
                    Ok(token) => {
                        self.headers = serde_json::to_string_pretty(&token.header()).unwrap();
                        self.payload = serde_json::to_string_pretty(&token.claims()).unwrap();
                    }
                    Err(err) => println!("{}", err),
                }
            }
        }
    }

    pub fn view(&mut self) -> Element<Message> {
        let text_input = TextInput::new(
            &mut self.jwt_input,
            "",
            &*self.jwt_token.to_string(),
            Message::JWTToken,
        )
        .padding(10)
        .width(Length::Fill)
        .size(30);

        Column::new()
            .padding(20)
            .align_items(Align::Center)
            .push(text_input)
            .push(
                Row::new()
                    .padding(20)
                    .width(Length::Fill)
                    .push(
                        Column::new()
                            .padding(20)
                            .push(Text::new("Headers").size(35))
                            .push(Text::new(&self.headers).size(25)),
                    )
                    .push(
                        Column::new()
                            .padding(20)
                            .push(Text::new("Payload").size(35))
                            .push(Text::new(&self.payload).size(25)),
                    ),
            )
            .into()
    }
}
