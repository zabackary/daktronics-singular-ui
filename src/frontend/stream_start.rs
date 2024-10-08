use std::fmt::Display;

use iced::{
    widget::{column, component, container, pick_list, row, svg, text, Component},
    Alignment, Border, Element, Length, Renderer, Shadow, Size, Theme,
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

pub struct StreamStart<'a, Message: Clone> {
    on_stream_start: Box<dyn Fn(String) -> Message>,
    error: Option<&'a str>,
    profile_is_dirty: bool,
}

impl<'a, Message: Clone> StreamStart<'a, Message> {
    pub fn new(
        on_stream_start: impl Fn(String) -> Message + 'static,
        error: Option<&'a str>,
        profile_is_dirty: bool,
    ) -> StreamStart<'a, Message> {
        StreamStart::<Message> {
            on_stream_start: Box::new(on_stream_start),
            error,
            profile_is_dirty,
        }
    }
}

pub fn stream_start<'a, Message: Clone>(
    on_stream_start: impl Fn(String) -> Message + 'static,
    error: Option<&'a str>,
    profile_is_dirty: bool,
) -> StreamStart<'a, Message> {
    StreamStart::<Message>::new(on_stream_start, error, profile_is_dirty)
}

#[derive(Debug, Clone)]
pub enum StreamStartEvent {
    StartStream,
    SerialPortPicked(SerialPortInfoWrapper),
    RefreshSerialPorts,
}

#[derive(Debug, Clone)]
pub struct StreamStartState {
    serial_ports: Vec<SerialPortInfoWrapper>,
    selected_serial_port: Option<SerialPortInfoWrapper>,
}

impl Default for StreamStartState {
    fn default() -> Self {
        StreamStartState {
            serial_ports: enumerate_ports(),
            selected_serial_port: None,
        }
    }
}

impl<'a, Message: Clone> Component<Message> for StreamStart<'a, Message> {
    type State = StreamStartState;

    type Event = StreamStartEvent;

    fn update(&mut self, state: &mut Self::State, event: Self::Event) -> Option<Message> {
        match event {
            StreamStartEvent::StartStream => Some((self.on_stream_start)(
                state
                    .selected_serial_port
                    .as_ref()
                    .expect("tty port should be selected before StartStream callback is run")
                    .0
                    .port_name
                    .clone(),
            )),
            StreamStartEvent::RefreshSerialPorts => {
                state.serial_ports = enumerate_ports();
                None
            }
            StreamStartEvent::SerialPortPicked(new_port) => {
                state.selected_serial_port = Some(new_port);
                None
            }
        }
    }

    fn view(&self, state: &Self::State) -> Element<Self::Event, Theme, Renderer> {
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
                            state.serial_ports.clone(), // TODO: would be good to not clone this all the time but compiler errors...
                            state.selected_serial_port.clone(), // TODO: this too
                            StreamStartEvent::SerialPortPicked,
                        )
                        .placeholder("Serial port")
                        .padding(8)
                        .width(300)
                        .style(rounded_pick_list_style)
                        .into(),
                        icon_button(
                            include_bytes!("../../assets/icon_refresh.svg"),
                            "Refresh ports",
                            Some(StreamStartEvent::RefreshSerialPorts),
                            super::utils::RoundedButtonVariant::Secondary,
                        )
                        .into(),
                        icon_button(
                            include_bytes!("../../assets/icon_play_circle.svg"),
                            "Start stream",
                            state
                                .selected_serial_port
                                .is_some()
                                .then_some(StreamStartEvent::StartStream),
                            super::utils::RoundedButtonVariant::Secondary,
                        )
                        .into(),
                    ])
                    .spacing(4)
                    .into(),
                ])
                .push_maybe(self.error.map(|error| {
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
                .push_maybe(self.profile_is_dirty.then(|| {
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

    fn size_hint(&self) -> Size<Length> {
        Size {
            width: Length::Fill,
            height: Length::Fill,
        }
    }
}

impl<'a, Message: Clone> From<StreamStart<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(header: StreamStart<'a, Message>) -> Self {
        component(header)
    }
}
