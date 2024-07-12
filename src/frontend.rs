mod header;
mod utils;

use header::header;
use iced::widget::{column, container, text, Column};
use iced::Alignment;

#[derive(Debug, Default)]
pub struct DaktronicsSingularUiApp {}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    NoOp,
}

impl DaktronicsSingularUiApp {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::NoOp => {}
        }
    }

    pub fn view(&self) -> Column<Message> {
        column([
            header(
                header::HeaderScreen::Configure,
                false,
                |_| Message::NoOp,
                |_| Message::NoOp,
                || Message::NoOp,
                || Message::NoOp,
                || Message::NoOp,
                Some(|| Message::NoOp),
            )
            .into(),
            container(text("hi there!").size(50)).padding(20).into(),
        ])
        .align_items(Alignment::Center)
    }
}
