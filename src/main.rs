use iced::{button, Align, Column, Element, Row, Sandbox, Settings, Text};

mod jwt;
mod ui;
mod unix_time;

pub fn main() {
    DevTools::run(Settings::default())
}

#[derive(Debug, Clone)]
enum Message {
    ChangeTab(SelectedPage),
    Unix(unix_time::Message),
    JWT(jwt::Message),
}

#[derive(Debug, Clone)]
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
struct Page<T: Default> {
    page: T,
    button: button::State,
}

#[derive(Default)]
struct DevTools {
    active_page: SelectedPage,
    unix: Page<unix_time::State>,
    jwt: Page<jwt::State>,
}

impl Sandbox for DevTools {
    type Message = Message;

    fn new() -> Self {
        Default::default()
    }

    fn title(&self) -> String {
        String::from("devtools")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ChangeTab(page) => {
                self.active_page = page;
            }
            Message::Unix(message) => {
                self.unix.page.update(message);
            }
            Message::JWT(message) => {
                self.jwt.page.update(message);
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let active_window = match self.active_page {
            SelectedPage::Unix => self.unix.page.view().map(Message::Unix),
            SelectedPage::JWT => self.jwt.page.view().map(Message::JWT),
        };

        let title_bar = Column::new()
            .padding(20)
            .align_items(Align::Center)
            .push(Text::new("Dev Tools"));

        let navigation = Column::new()
            .push(
                ui::button(&mut self.unix.button, "Unix time")
                    .on_press(Message::ChangeTab(SelectedPage::Unix)),
            )
            .push(
                ui::button(&mut self.jwt.button, "JWT")
                    .on_press(Message::ChangeTab(SelectedPage::JWT)),
            );

        let main_view = Row::new().push(active_window);

        Column::new()
            .push(title_bar)
            .push(navigation)
            .push(main_view)
            .into()
    }
}
