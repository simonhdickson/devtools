use iced::{Align, Column, Element, Length, Radio, Row, Sandbox, Settings, Text};

mod jwt;
mod ui;
mod unix_time;

pub fn main() -> iced::Result {
    DevTools::run(Settings::default())
}

#[derive(Debug, Clone)]
enum Message {
    ChangeTab(SelectedPage),
    Unix(unix_time::Message),
    JWT(jwt::Message),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum SelectedPage {
    Unix,
    JWT,
}

impl Default for SelectedPage {
    fn default() -> Self {
        Self::Unix
    }
}

#[derive(Default)]
struct DevTools {
    active_page: SelectedPage,
    unix: unix_time::State,
    jwt: jwt::State,
}

impl Sandbox for DevTools {
    type Message = Message;

    fn new() -> Self {
        Default::default()
    }

    fn title(&self) -> String {
        String::from("Dev Tools")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ChangeTab(page) => {
                self.active_page = page;
            }
            Message::Unix(message) => {
                self.unix.update(message);
            }
            Message::JWT(message) => {
                self.jwt.update(message);
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let active_window = match self.active_page {
            SelectedPage::Unix => self.unix.view().map(Message::Unix),
            SelectedPage::JWT => self.jwt.view().map(Message::JWT),
        };

        let title_bar = Text::new("Dev Tools").width(Length::Shrink).size(50);

        let navigation = Column::new()
            .padding(20)
            .spacing(20)
            .push(Radio::new(
                SelectedPage::Unix,
                "Unix time",
                Some(self.active_page),
                Message::ChangeTab,
            ))
            .push(Radio::new(
                SelectedPage::JWT,
                "JWT",
                Some(self.active_page),
                Message::ChangeTab,
            ));

        Column::new()
            .padding(20)
            .spacing(20)
            .align_items(Align::Center)
            .push(title_bar)
            .push(Row::new().push(navigation).push(active_window))
            .into()
    }
}
