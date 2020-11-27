use iced::{
    executor, Align, Application, Column, Command, Container, Element, Length, Radio, Row, Settings,
};

mod base64;
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
    Base64(base64::Message),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum SelectedPage {
    Unix,
    Jwt,
    Base64,
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
    base64: base64::State,
    theme: ui::style::Theme,
}

impl Application for DevTools {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Dev Tools")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
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
            Message::Base64(message) => {
                self.base64.update(message);
            }
        }

        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        let active_window = match self.active_page {
            SelectedPage::Unix => self.unix.view(self.theme).map(Message::Unix),
            SelectedPage::Jwt => self.jwt.view(self.theme).map(Message::JWT),
            SelectedPage::Base64 => self.base64.view(self.theme).map(Message::Base64),
        };

        let navigation = Column::new()
            .padding(20)
            .spacing(20)
            .push(
                Radio::new(
                    SelectedPage::Unix,
                    "Unix time",
                    Some(self.active_page),
                    Message::ChangeTab,
                )
                .style(self.theme),
            )
            .push(
                Radio::new(
                    SelectedPage::Jwt,
                    "JWT",
                    Some(self.active_page),
                    Message::ChangeTab,
                )
                .style(self.theme),
            )
            .push(
                Radio::new(
                    SelectedPage::Base64,
                    "Base64",
                    Some(self.active_page),
                    Message::ChangeTab,
                )
                .style(self.theme),
            );

        let content = Column::new()
            .padding(20)
            .spacing(20)
            .align_items(Align::Center)
            .push(Row::new().push(navigation).push(active_window));

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .style(self.theme)
            .into()
    }
}
