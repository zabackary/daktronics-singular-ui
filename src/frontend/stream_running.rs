use iced::{
    widget::{column, component, container, horizontal_space, row, scrollable, text, Component},
    Element, Font, Length, Renderer, Size, Theme,
};

use crate::backend::stream::ActiveStream;

use super::{
    graph::graph,
    utils::{icon_button, rounded_pane},
};

pub struct StreamRunning<'a, Message> {
    active_stream: &'a ActiveStream,
    clear_errors: Message,
}

impl<'a, Message> StreamRunning<'a, Message> {
    pub fn new(
        active_stream: &'a ActiveStream,
        clear_errors: Message,
    ) -> StreamRunning<'a, Message> {
        StreamRunning {
            active_stream,
            clear_errors,
        }
    }
}

#[derive(Clone, Debug, Copy)]
pub enum StreamRunningEvent {
    ClearErrors,
}

pub fn stream_running<'a, Message>(
    active_stream: &'a ActiveStream,
    clear_errors: Message,
) -> StreamRunning<'a, Message> {
    StreamRunning::new(&active_stream, clear_errors)
}

fn pane_header<'a, Message: 'a>(
    label: &'a str,
    stat: i32,
    unit: &'a str,
    button: Option<impl Into<Element<'a, Message, Theme, Renderer>>>,
) -> Element<'a, Message, Theme, Renderer> {
    row([
        text(label).width(Length::Fill).size(18).into(),
        text(stat)
            .size(32)
            .style(|theme: &Theme| text::Style {
                color: Some(theme.extended_palette().secondary.strong.color),
            })
            .into(),
        text(unit)
            .size(14)
            .style(|theme: &Theme| text::Style {
                color: Some(theme.extended_palette().secondary.strong.color),
            })
            .into(),
    ])
    .push_maybe(button.is_some().then(|| horizontal_space().width(4)))
    .push_maybe(button)
    .height(64)
    .align_y(iced::Alignment::Center)
    .padding(8)
    .spacing(2)
    .into()
}

impl<'a, Message: Clone> Component<Message> for StreamRunning<'a, Message> {
    type State = ();

    type Event = StreamRunningEvent;

    fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<Message> {
        match event {
            StreamRunningEvent::ClearErrors => Some(self.clear_errors.clone()),
        }
    }

    fn view(&self, _state: &Self::State) -> Element<Self::Event, Theme, Renderer> {
        let latency_pane = column([
            pane_header(
                "Latency",
                self.active_stream
                    .latency_graph_data()
                    .samples
                    .last()
                    .map(|x| x.latency.as_millis().try_into().unwrap_or(i32::MAX))
                    .unwrap_or(0),
                "ms",
                // not sure why Rust needs annotations but whatever
                Option::<&str>::None,
            ),
            rounded_pane(
                container(graph(&self.active_stream))
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .padding(16),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .into(),
        ]);
        let payload_pane = {
            column([
                pane_header(
                    "Latest payload",
                    self.active_stream
                        .latest_payload_size()
                        .unwrap_or(0)
                        .try_into()
                        .unwrap_or(i32::MAX),
                    "B",
                    // not sure why Rust needs annotations but whatever
                    // &str can be `into`'ed into an Element, so it works
                    Option::<&str>::None,
                ),
                rounded_pane(scrollable(
                    container(
                        text(self.active_stream.latest_payload().unwrap_or("No data"))
                            .font(Font::MONOSPACE),
                    )
                    .width(Length::Fill)
                    .padding(16),
                ))
                .width(Length::Fill)
                .height(Length::Fill)
                .into(),
            ])
        };
        let error_pane = (!self.active_stream.errors().is_empty()).then(|| {
            column([
                pane_header(
                    "Errors (last 20)",
                    self.active_stream
                        .errors()
                        .len()
                        .try_into()
                        .unwrap_or(i32::MAX),
                    "x",
                    Some(icon_button(
                        include_bytes!("../../assets/icon_delete.svg"),
                        "Clear messages",
                        Some(StreamRunningEvent::ClearErrors),
                        super::utils::RoundedButtonVariant::Secondary,
                    )),
                ),
                scrollable(
                    column(self.active_stream.errors().iter().map(|error| {
                        rounded_pane(
                            column([
                                text(timeago::Formatter::new().convert(error.timestamp.elapsed()))
                                    .size(12)
                                    .into(),
                                text(&error.msg).font(Font::MONOSPACE).into(),
                            ])
                            .padding(8)
                            .width(Length::Fill)
                            .spacing(4),
                        )
                        .into()
                    }))
                    .spacing(8),
                )
                .into(),
            ])
        });
        row([latency_pane.into(), payload_pane.into()])
            .push_maybe(error_pane)
            .spacing(12)
            .padding(16)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn size_hint(&self) -> Size<Length> {
        Size {
            width: Length::Fill,
            height: Length::Fill,
        }
    }
}

impl<'a, Message: Clone> From<StreamRunning<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(header: StreamRunning<'a, Message>) -> Self {
        component(header)
    }
}
