mod header;
mod utils;

use std::fmt::Display;

use header::{header, HeaderScreen};
use iced::widget::{column, container, pick_list, row, text};
use iced::{Alignment, Element, Length, Task};
use tokio_serial::SerialPortInfo;
use utils::{icon_button, rounded_button, BORDER_RADIUS};

use crate::backend::profile::Profile;
use crate::backend::stream::ActiveStream;

#[derive(Debug, Default)]
pub struct DaktronicsSingularUiApp {
    screen: Screen,
    profile: Profile,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SerialPortInfoWrapper(SerialPortInfo);

impl Display for SerialPortInfoWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0.port_type {
            tokio_serial::SerialPortType::UsbPort(info) => write!(
                f,
                "{} ({} {})",
                self.0.port_name,
                info.manufacturer
                    .as_ref()
                    .map(String::as_ref)
                    .unwrap_or("unknown manufacturer"),
                info.product
                    .as_ref()
                    .map(String::as_ref)
                    .unwrap_or("unknown product")
            ),
            _ => write!(f, "{}", self.0.port_name),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    NewProfile,
    ImportProfile,
    ExportProfile,
    WelcomeNewProfile,
    WelcomeImportProfile,
    StartStream,
    EndStream,
    SwitchScreen(HeaderScreen),
    ProfileNameChange(String),
    SerialPortPicked(SerialPortInfoWrapper),
    RefreshSerialPorts,
}

#[derive(Debug, Default)]
enum Screen {
    Configure,
    Stream(ActiveStream),
    StreamStart(Vec<SerialPortInfoWrapper>, Option<SerialPortInfoWrapper>),
    #[default]
    Welcome,
}

fn enumerate_ports() -> Vec<SerialPortInfoWrapper> {
    tokio_serial::available_ports()
        .unwrap_or(vec![])
        .into_iter()
        .map(SerialPortInfoWrapper)
        .collect()
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
            Message::StartStream => {
                self.screen = Screen::Stream(todo!("get the active stream somehow"));
            }
            Message::EndStream => {
                // Drop the stream, killing the background threads automatically
                self.screen = Screen::StreamStart(enumerate_ports(), None);
            }
            Message::ProfileNameChange(new_name) => self.profile.name = new_name,
            Message::SwitchScreen(new_screen) => {
                self.screen = match new_screen {
                    HeaderScreen::Configure => Screen::Configure,
                    HeaderScreen::Stream => Screen::StreamStart(enumerate_ports(), None),
                }
            }
            Message::SerialPortPicked(new_port) => match self.screen {
                Screen::StreamStart(_, ref mut selected_port) => *selected_port = Some(new_port),
                _ => {}
            },
            Message::RefreshSerialPorts => match self.screen {
                Screen::StreamStart(ref mut serial_ports, _) => *serial_ports = enumerate_ports(),
                _ => {}
            },
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
                        Screen::Stream(_) | Screen::StreamStart(_, _) => HeaderScreen::Stream,
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
                        todo!()
                    }
                    Screen::StreamStart(serial_ports, selected_serial_port) => container(
                        column([
                            text("Ready to get started?")
                                .style(|theme: &iced::Theme| text::Style {
                                    color: Some(theme.palette().text),
                                })
                                .size(32)
                                .into(),
                            row([
                                pick_list(
                                    &serial_ports[..],
                                    selected_serial_port.as_ref(),
                                    Message::SerialPortPicked,
                                )
                                .placeholder("Serial port")
                                .padding(8)
                                .width(300)
                                .style(|theme, status| {
                                    let mut style = pick_list::default(theme, status);
                                    style.border.radius = BORDER_RADIUS.into();
                                    style
                                })
                                .into(),
                                icon_button(
                                    include_bytes!("../assets/icon_refresh.svg"),
                                    "Refresh ports",
                                    Some(Message::RefreshSerialPorts),
                                )
                                .into(),
                                icon_button(
                                    include_bytes!("../assets/icon_play_circle.svg"),
                                    "Start stream",
                                    Some(Message::StartStream),
                                )
                                .into(),
                            ])
                            .spacing(4)
                            .into(),
                        ])
                        .spacing(16)
                        .align_items(Alignment::Center),
                    )
                    .align_y(iced::alignment::Vertical::Center)
                    .align_x(iced::alignment::Horizontal::Center)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .into(),
                    Screen::Welcome => unreachable!(),
                },
            ])
            .align_items(Alignment::Center)
            .into()
        }
    }
}
