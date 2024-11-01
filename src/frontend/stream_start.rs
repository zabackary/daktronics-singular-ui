use std::fmt::Display;

use iced::{
    widget::{column, container, pick_list, row, svg, text},
    Alignment, Border, Length, Shadow, Theme,
};
use tokio_serial::SerialPortInfo;

use super::utils::{icon_button, rounded_pick_list_style};

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
            tokio_serial::SerialPortType::BluetoothPort => {
                write!(f, "{} (via Bluetooth)", self.0.port_name)
            }
            tokio_serial::SerialPortType::PciPort => write!(f, "{} (via PCI)", self.0.port_name),
            tokio_serial::SerialPortType::Unknown => write!(f, "{}", self.0.port_name),
        }
    }
}

impl PartialOrd for SerialPortInfoWrapper {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.port_name.partial_cmp(&other.0.port_name)
    }
}

impl Ord for SerialPortInfoWrapper {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.port_name.cmp(&other.0.port_name)
    }
}

fn enumerate_ports() -> Vec<SerialPortInfoWrapper> {
    let mut ports = tokio_serial::available_ports()
        .unwrap_or(vec![])
        .into_iter()
        .map(SerialPortInfoWrapper)
        .collect::<Vec<_>>();
    ports.sort();
    ports
}

#[derive(Debug)]
pub struct StreamStart {
    serial_ports: Vec<SerialPortInfoWrapper>,
    selected_serial_port: Option<SerialPortInfoWrapper>,
}

#[derive(Debug, Clone)]
pub enum StreamStartMessage {
    StartStream,
    SerialPortPicked(SerialPortInfoWrapper),
    RefreshSerialPorts,
}

pub enum Update {
    None,
    StartStream { port: String },
}

impl StreamStart {
    pub fn new() -> Self {
        Self {
            selected_serial_port: None,
            serial_ports: enumerate_ports(),
        }
    }

    pub fn view<'a>(
        &'a self,
        error: Option<&'a str>,
        profile_is_dirty: bool,
    ) -> iced::Element<'a, StreamStartMessage> {
        iced::widget::stack([
            container(
                svg(svg::Handle::from_memory(include_bytes!(
                    "../../assets/splash_start.svg"
                )))
                .width(900)
                .height(450)
                .content_fit(iced::ContentFit::Cover)
                .opacity(0.8),
            )
            .align_x(iced::alignment::Horizontal::Right)
            .align_y(iced::alignment::Vertical::Bottom)
            .width(Length::Fill)
            .height(Length::Fill)
            .into(),
            container(
                column([
                    text("Ready to get started?")
                        .style(|theme: &iced::Theme| text::Style {
                            color: Some(theme.palette().text),
                        })
                        .size(32)
                        .into(),
                    row([
                        pick_list(
                            self.serial_ports.clone(), // TODO: would be good to not clone this all the time but compiler errors...
                            self.selected_serial_port.clone(), // TODO: this too
                            StreamStartMessage::SerialPortPicked,
                        )
                        .placeholder("Serial port")
                        .padding(8)
                        .width(300)
                        .style(rounded_pick_list_style)
                        .into(),
                        icon_button(
                            include_bytes!("../../assets/icon_refresh.svg"),
                            "Refresh ports",
                            Some(StreamStartMessage::RefreshSerialPorts),
                            super::utils::RoundedButtonVariant::Secondary,
                        )
                        .into(),
                        icon_button(
                            include_bytes!("../../assets/icon_play_circle.svg"),
                            "Start stream",
                            self.selected_serial_port
                                .is_some()
                                .then_some(StreamStartMessage::StartStream),
                            super::utils::RoundedButtonVariant::Secondary,
                        )
                        .into(),
                    ])
                    .spacing(4)
                    .into(),
                ])
                .push_maybe(error.map(|error| {
                    container(
                        row([
                            container(
                                svg(svg::Handle::from_memory(include_bytes!(
                                    "../../assets/icon_error.svg"
                                )))
                                .style(|theme: &Theme, _| svg::Style {
                                    color: Some(theme.palette().danger),
                                })
                                .content_fit(iced::ContentFit::Fill),
                            )
                            .width(24)
                            .height(24)
                            .into(),
                            text(error).style(text::danger).into(),
                        ])
                        .align_y(Alignment::Center)
                        .spacing(8),
                    )
                    .style(|theme: &iced::Theme| container::Style {
                        background: Some(theme.palette().background.into()),
                        text_color: None,
                        shadow: Shadow::default(),
                        border: Border {
                            color: theme.palette().danger,
                            width: 1.0,
                            radius: 999.into(),
                        },
                    })
                    .padding([8, 16])
                }))
                .push_maybe(profile_is_dirty.then(|| {
                    container(
                        row([
                            container(
                                svg(svg::Handle::from_memory(include_bytes!(
                                    "../../assets/icon_warning.svg"
                                )))
                                .style(|theme: &Theme, _| svg::Style {
                                    color: Some(theme.palette().danger),
                                })
                                .content_fit(iced::ContentFit::Fill),
                            )
                            .width(24)
                            .height(24)
                            .into(),
                            text("The profile has not been saved yet.")
                                .style(text::danger)
                                .into(),
                        ])
                        .align_y(Alignment::Center)
                        .spacing(8),
                    )
                    .style(|theme: &iced::Theme| container::Style {
                        background: Some(theme.palette().background.into()),
                        text_color: None,
                        shadow: Shadow::default(),
                        border: Border {
                            color: theme.palette().danger,
                            width: 1.0,
                            radius: 999.into(),
                        },
                    })
                    .padding([8, 16])
                }))
                .spacing(16)
                .align_x(Alignment::Center),
            )
            .align_y(iced::alignment::Vertical::Center)
            .align_x(iced::alignment::Horizontal::Center)
            .width(Length::Fill)
            .height(Length::Fill)
            .into(),
        ])
        .into()
    }

    pub fn update<'a>(&mut self, message: StreamStartMessage) -> Update {
        match message {
            StreamStartMessage::StartStream => Update::StartStream {
                port: self
                    .selected_serial_port
                    .as_ref()
                    .expect("tty port should be selected before StartStream callback is run")
                    .0
                    .port_name
                    .clone(),
            },
            StreamStartMessage::RefreshSerialPorts => {
                self.serial_ports = enumerate_ports();
                Update::None
            }
            StreamStartMessage::SerialPortPicked(new_port) => {
                self.selected_serial_port = Some(new_port);
                Update::None
            }
        }
    }
}
