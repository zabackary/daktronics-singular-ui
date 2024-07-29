mod header;
mod stream_start;
mod utils;

use header::{header, HeaderScreen};
use iced::widget::{column, container, row, text};
use iced::{Alignment, Element, Length, Task};
use stream_start::stream_start;
use utils::rounded_button;

use crate::backend::profile::Profile;
use crate::backend::stream::ActiveStream;

#[derive(Debug, Default)]
pub struct DaktronicsSingularUiApp {
    screen: Screen,
    profile: Profile,
}

#[derive(Debug, Clone)]
pub enum Message {
    NewProfile,
    ImportProfile,
    ExportProfile,
    WelcomeNewProfile,
    WelcomeImportProfile,
    StartStream(String),
    EndStream,
    SwitchScreen(HeaderScreen),
    ProfileNameChange(String),
}

#[derive(Debug, Default)]
enum Screen {
    Configure,
    Stream(ActiveStream),
    StreamStart(Option<String>),
    #[default]
    Welcome,
}

impl DaktronicsSingularUiApp {
    pub fn update(&mut self, message: Message) -> impl Into<Task<Message>> {
        match message {
            Message::ExportProfile => {
                todo!()
            }
            Message::ImportProfile => {
                todo!()
            }
            Message::NewProfile => {
                self.profile = Profile::default();
            }
            Message::WelcomeImportProfile => {
                todo!()
            }
            Message::WelcomeNewProfile => {
                self.screen = Screen::Configure;
            }
            Message::StartStream(tty_path) => match self.screen {
                Screen::StreamStart(ref mut error) => {
                    self.profile.sport_type =
                        Some(crate::backend::sports::DynamicSportType::Basketball);
                    match ActiveStream::new(self.profile.to_owned(), tty_path) {
                        Ok(stream) => {
                            self.screen = Screen::Stream(stream);
                        }
                        Err(err) => *error = Some(err.to_string()),
                    }
                }
                _ => {}
            },
            Message::EndStream => {
                // Drop the stream, killing the background threads automatically
                self.screen = Screen::StreamStart(None);
            }
            Message::ProfileNameChange(new_name) => self.profile.name = new_name,
            Message::SwitchScreen(new_screen) => {
                self.screen = match new_screen {
                    HeaderScreen::Configure => Screen::Configure,
                    HeaderScreen::Stream => Screen::StreamStart(None),
                }
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        if matches!(self.screen, Screen::Welcome) {
            container(
                column([
                    text(concat!(
                        "Daktronics Singular UI v",
                        env!("CARGO_PKG_VERSION")
                    ))
                    .size(18)
                    .style(|theme: &iced::Theme| text::Style {
                        color: Some(theme.palette().text.scale_alpha(0.6)),
                    })
                    .into(),
                    text("Welcome.")
                        .style(|theme: &iced::Theme| text::Style {
                            color: Some(theme.palette().text),
                        })
                        .size(76)
                        .into(),
                    row([
                        rounded_button(
                            text("Import profile").size(18),
                            utils::RoundedButtonVariant::Primary,
                        )
                        .on_press(Message::WelcomeImportProfile)
                        .into(),
                        rounded_button(
                            text("New profile").size(18),
                            utils::RoundedButtonVariant::Primary,
                        )
                        .on_press(Message::WelcomeNewProfile)
                        .into(),
                    ])
                    .spacing(8)
                    .into(),
                ])
                .spacing(16)
                .align_items(Alignment::Start),
            )
            .align_y(iced::alignment::Vertical::Center)
            .align_x(iced::alignment::Horizontal::Center)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
        } else {
            column([
                header(
                    match self.screen {
                        Screen::Configure => HeaderScreen::Configure,
                        Screen::Stream(_) | Screen::StreamStart(_) => HeaderScreen::Stream,
                        Screen::Welcome => unreachable!(),
                    },
                    !matches!(self.screen, Screen::Stream(_)),
                    Message::SwitchScreen,
                    &self.profile.name,
                    Message::ProfileNameChange,
                    Message::ImportProfile,
                    Message::ExportProfile,
                    Message::NewProfile,
                    matches!(self.screen, Screen::Stream(_)).then_some(Message::EndStream),
                )
                .into(),
                match &self.screen {
                    Screen::Configure => column([]).height(Length::Fill).width(Length::Fill).into(),
                    Screen::Stream(_active_stream) => {
                        container(text("latency")).center(Length::Fill).into()
                    }
                    Screen::StreamStart(error) => {
                        stream_start(Message::StartStream, error.as_deref()).into()
                    }
                    Screen::Welcome => unreachable!(),
                },
            ])
            .align_items(Alignment::Center)
            .into()
        }
    }
}
