pub mod style;

use iced::{button, Button, HorizontalAlignment, Text};

pub fn button<'a, Message>(state: &'a mut button::State, label: &str) -> Button<'a, Message>
where
    Message: Clone,
{
    Button::new(
        state,
        Text::new(label).horizontal_alignment(HorizontalAlignment::Center),
    )
    .padding(12)
    .min_width(100)
}
