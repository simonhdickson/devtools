use devtools_core::encoding::{self, ViewModel};
use iced::{text_input, Align, Column, Container, Element, Length, Text, TextInput};

use crate::ui::style::Theme;

#[derive(Debug, Clone)]
pub enum Message {
    SetBase64(String),
    SetPlainText(String),
}

pub struct State {
    vm: encoding::ViewModelImpl,
    from_base64_state: text_input::State,
    to_base64_state: text_input::State,
}

impl Default for State  {
    fn default() -> Self {
        let mut vm = encoding::create();
        vm.set_kind(encoding::Kind::Base64);

        Self {
            vm,
            from_base64_state: text_input::State::new(),
            to_base64_state: text_input::State::new(),
        }
    }
}

impl State {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::SetBase64(new_value) => self.vm.set_encoded_text(&new_value),
            Message::SetPlainText(new_value) => self.vm.set_plain_text(&new_value),
            
        }
    }

    pub fn view(&mut self, theme: Theme) -> Element<Message> {
        let to_input = TextInput::new(
            &mut self.to_base64_state,
            "",
            &*self.vm.plain_text().unwrap(),
            Message::SetPlainText,
        )
        .padding(10)
        .size(30)
        .style(theme);

        let from_input = TextInput::new(
            &mut self.from_base64_state,
            "",
            &*self.vm.encoded_text().unwrap(),
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
