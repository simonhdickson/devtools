use chrono::{Local, TimeZone, Utc};
use iced::{button, text_input, Align, Column, Element, Text, TextInput};

use crate::ui;

#[derive(Debug, Clone)]
pub enum Message {
    UnixTimeChanged(String),
    UnixTimeNow,
}

pub struct State {
    unix_time: i64,
    unix_state: text_input::State,
    now: button::State,
}

impl Default for State {
    fn default() -> Self {
        Self {
            unix_time: 0,
            unix_state: text_input::State::new(),
            now: button::State::new(),
        }
    }
}

impl State {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::UnixTimeChanged(new_value) => {
                if let Ok(v) = new_value.parse() {
                    if v >= 0 {
                        self.unix_time = v;
                    }
                }
            }
            Message::UnixTimeNow => {
                self.unix_time = Utc::now().timestamp();
            }
        }
    }

    pub fn view(&mut self) -> Element<Message> {
        let text_input = TextInput::new(
            &mut self.unix_state,
            "",
            &*self.unix_time.to_string(),
            Message::UnixTimeChanged,
        )
        .padding(10)
        .size(30);
        Column::new()
            .padding(20)
            .align_items(Align::Center)
            .push(ui::button(&mut self.now, "Now").on_press(Message::UnixTimeNow))
            .push(Text::new("Unix Time Conversion."))
            .push(text_input)
            .push(Text::new(Utc.timestamp(self.unix_time, 0).to_string()).size(50))
            .push(Text::new(Local.timestamp(self.unix_time, 0).to_string()).size(50))
            .into()
    }
}
