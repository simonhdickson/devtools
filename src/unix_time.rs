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
    utc_time: String,
    local_time: String,
    unix_state: text_input::State,
    now: button::State,
}

impl Default for State {
    fn default() -> Self {
        Self {
            unix_time: Default::default(),
            utc_time: Default::default(),
            local_time: Default::default(),
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
                        self.utc_time = Utc.timestamp(self.unix_time, 0).to_string();
                        self.local_time = Local.timestamp(self.unix_time, 0).to_string();
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
            .push(text_input)
            .push(Text::new(self.utc_time.to_owned()).size(25))
            .push(Text::new(self.local_time.to_owned()).size(25))
            .into()
    }
}
