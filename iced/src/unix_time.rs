use devtools_core::unix_time;
use iced::{button, text_input, Align, Column, Container, Element, Length, Text, TextInput};

use crate::ui::{self, style::Theme};

#[derive(Debug, Clone)]
pub enum Message {
    UnixTimeChanged(String),
    UnixTimeNow,
}

pub struct State {
    unix_time: unix_time::UnixTime,
    unix_state: text_input::State,
    now: button::State,
}

impl Default for State {
    fn default() -> Self {
        Self {
            unix_time: Default::default(),
            unix_state: text_input::State::new(),
            now: button::State::new(),
        }
    }
}

impl State {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::UnixTimeChanged(new_value) => match self.unix_time.set_unix_time_string(&new_value) {
                Ok(()) => (),
                Err(err) => println!("{:#?}", err),
            },
            Message::UnixTimeNow => self.unix_time.set_unix_time_to_now(),
        }
    }

    pub fn view(&mut self, theme: Theme) -> Element<Message> {
        let text_input = TextInput::new(
            &mut self.unix_state,
            "",
            &self.unix_time.get_unix_time().to_string(),
            Message::UnixTimeChanged,
        )
        .padding(10)
        .size(30)
        .style(theme);

        let content = Column::new()
            .padding(20)
            .spacing(20)
            .align_items(Align::Center)
            .push(
                ui::button(&mut self.now, "Now")
                    .on_press(Message::UnixTimeNow)
                    .style(theme),
            )
            .push(text_input)
            .push(Text::new(self.unix_time.get_utc_time().to_owned()).size(25))
            .push(Text::new(self.unix_time.get_local_time().to_owned()).size(25));

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .align_y(Align::Start)
            .style(theme)
            .into()
    }
}
